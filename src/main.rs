use std::borrow::BorrowMut;
use std::env;
use std::ffi::c_void;
use std::fs::File;
use std::io::Read;
use std::mem::{MaybeUninit, size_of};
use std::ptr::null;
use std::str::FromStr;
use ::image::EncodableLayout;
use ascii::AsciiString;
use windows::core::{GUID, PCSTR};
use windows::Win32::Foundation::{HWND, WIN32_ERROR};
use windows::Win32::Graphics::Direct3D11::{D3D11_BIND_INDEX_BUFFER, D3D11_BIND_VERTEX_BUFFER, D3D11_BUFFER_DESC, D3D11_INPUT_ELEMENT_DESC, D3D11_INPUT_PER_VERTEX_DATA, D3D11_RENDER_TARGET_VIEW_DESC, D3D11_SUBRESOURCE_DATA, D3D11_USAGE_DEFAULT, ID3D11Buffer, ID3D11InputLayout, ID3D11RenderTargetView, ID3D11Resource};
use windows::Win32::Graphics::Dxgi::{DXGI_INFO_QUEUE_MESSAGE, DXGI_INFO_QUEUE_MESSAGE_CATEGORY_MISCELLANEOUS, DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR, DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO, DXGIGetDebugInterface1, IDXGIInfoQueue};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_R32_UINT, DXGI_FORMAT_R32G32_FLOAT, DXGI_FORMAT_R32G32B32_FLOAT};
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, MSG, PeekMessageW, PM_REMOVE, TranslateMessage, WM_QUIT};
use crate::rendering::{create_and_register_window_class, create_and_show_window, create_device, DXGIFormat, setup_directx_device_and_swapchain};
use crate::rendering::win32::dxgidebug::dump_debug_messages;
use crate::rendering::win32::errhandlingapi::{get_last_error, set_last_error, WIN32Error};

mod vector_2d;
mod vector_3d;
mod rendering;
mod image;
mod texture;
mod color;
mod pixel;

fn main() {
    let mut pixel_shader_file = File::open("shaders/pixel_shader.cso").unwrap();
    let mut vertex_shader_file = File::open("shaders/vertex_shader.cso").unwrap();

    let mut pixel_shader_data = Vec::new();
    pixel_shader_file.read_to_end(&mut pixel_shader_data).unwrap();
    let mut vertex_shader_data = Vec::new();
    vertex_shader_file.read_to_end(&mut vertex_shader_data).unwrap();

    let _window_class_atom = create_and_register_window_class().unwrap();
    let window_handle = create_and_show_window().unwrap();

    let (
        feature_level,
        swap_chain,
        device,
        device_context
    ) = setup_directx_device_and_swapchain(window_handle).unwrap();

    let back_buffer: ID3D11Resource = unsafe {
        swap_chain.GetBuffer::<ID3D11Resource>(0)
    }.unwrap();

    let render_target_view: ID3D11RenderTargetView = unsafe {
        device.CreateRenderTargetView(
            &back_buffer,
            null()
        )
    }.unwrap();

    let vertices = [
        Vertex {
            x: 0.0,
            y: 0.0,
            u: 0.0,
            v: 0.0,
            r: 0.0,
            g: 0.0,
            b: 0.0
        },
        Vertex {
            x: 1.0,
            y: 0.0,
            u: 1.0,
            v: 0.0,
            r: 0.0,
            g: 0.0,
            b: 1.0
        },
        Vertex {
            x: 0.0,
            y: 1.0,
            u: 0.0,
            v: 1.0,
            r: 0.0,
            g: 1.0,
            b: 0.0
        },
        Vertex {
            x: 1.0,
            y: 1.0,
            u: 1.0,
            v: 1.0,
            r: 1.0,
            g: 1.0,
            b: 1.0
        },
    ];

    let vertex_buffer_description = D3D11_BUFFER_DESC {
        ByteWidth: (size_of::<Vertex>() * 4) as u32,
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: D3D11_BIND_VERTEX_BUFFER.0,
        CPUAccessFlags: 0,
        MiscFlags: 0,
        StructureByteStride: 0
    };

    let vertex_buffer_subresource_data = D3D11_SUBRESOURCE_DATA {
        pSysMem: &vertices as *const Vertex as *const c_void,
        SysMemPitch: 0,
        SysMemSlicePitch: 0
    };

    let vertex_buffer: ID3D11Buffer = unsafe {
        device.CreateBuffer(&vertex_buffer_description as *const D3D11_BUFFER_DESC, &vertex_buffer_subresource_data as *const D3D11_SUBRESOURCE_DATA)
    }.unwrap();

    let vertex_buffer = Some(vertex_buffer);

    let vertex_buffer_stride = size_of::<Vertex>() as u32;
    let vertex_buffer_offset = 0u32;

    unsafe {
        device_context.IASetVertexBuffers(0,1, &vertex_buffer as *const Option<ID3D11Buffer>, &vertex_buffer_stride as *const u32, &vertex_buffer_offset as *const u32)
    }

    let vertex_buffer = vertex_buffer.unwrap();
    
    let triangle_indices = [0,1,2,3,2,1];

    let index_buffer_description = D3D11_BUFFER_DESC {
        ByteWidth: (size_of::<i32>() * 6) as u32,
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: D3D11_BIND_INDEX_BUFFER.0,
        CPUAccessFlags: 0,
        MiscFlags: 0,
        StructureByteStride: 0
    };

    let index_buffer_subresource_data = D3D11_SUBRESOURCE_DATA {
        pSysMem: &triangle_indices as *const i32 as *const c_void,
        SysMemPitch: 0,
        SysMemSlicePitch: 0
    };

    let index_buffer: ID3D11Buffer = unsafe {
        device.CreateBuffer(&index_buffer_description as *const D3D11_BUFFER_DESC, &index_buffer_subresource_data as *const D3D11_SUBRESOURCE_DATA)
    }.unwrap();

    unsafe {
        device_context.IASetIndexBuffer(&index_buffer, DXGI_FORMAT_R32_UINT, 0);
    }

    let position_element_name = AsciiString::from_str("POSITION").unwrap();
    let position_element_name_bytes: Vec<u8> = position_element_name.as_bytes().iter().map(|item| *item).collect();
    let position_element_description = D3D11_INPUT_ELEMENT_DESC {
        SemanticName: PCSTR(position_element_name_bytes.as_ptr()),
        SemanticIndex: 0,
        Format: DXGI_FORMAT_R32G32_FLOAT,
        InputSlot: 0,
        AlignedByteOffset: 0,
        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
        InstanceDataStepRate: 0
    };
    let uv_element_name = AsciiString::from_str("TEXCOORD").unwrap();
    let uv_element_name_bytes: Vec<u8> = uv_element_name.as_bytes().iter().map(|item| *item).collect();
    let uv_element_description = D3D11_INPUT_ELEMENT_DESC {
        SemanticName: PCSTR(uv_element_name_bytes.as_ptr()),
        SemanticIndex: 0,
        Format: DXGI_FORMAT_R32G32_FLOAT,
        InputSlot: 0,
        AlignedByteOffset: 0,
        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
        InstanceDataStepRate: 0
    };
    let color_element_name = AsciiString::from_str("COLOR").unwrap();
    let color_element_name_bytes: Vec<u8> = color_element_name.as_bytes().iter().map(|item| *item).collect();
    let color_element_description = D3D11_INPUT_ELEMENT_DESC {
        SemanticName: PCSTR(color_element_name_bytes.as_ptr()),
        SemanticIndex: 0,
        Format: DXGI_FORMAT_R32G32B32_FLOAT,
        InputSlot: 0,
        AlignedByteOffset: 0,
        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
        InstanceDataStepRate: 0
    };

    let input_element_descriptions = [
        position_element_description,
        uv_element_description,
        color_element_description
    ];

    let input_layout: ID3D11InputLayout = unsafe {
        device.CreateInputLayout(&input_element_descriptions,&vertex_shader_data)
    }.unwrap();

    unsafe {
        device_context.IASetInputLayout(&input_layout)
    }

    loop{
        if handle_messages() {
            break;
        };

        unsafe {
            let clear_color: [f32; 4] = [0.0,0.0,0.5,1.0];
            let color_ptr = &clear_color as *const f32;

            device_context.ClearRenderTargetView(
                &render_target_view,
                color_ptr
            );
        }

        unsafe {
            swap_chain.Present(1,0);
        }
    }

    dump_debug_messages();
}

fn handle_messages() -> bool {
    loop {
        let message = peek_message();

        match message {
            None => {
                return false;
            }
            Some(message) => {
                if message.message == WM_QUIT {
                    return true;
                }

                unsafe {
                    TranslateMessage(&message as *const MSG);
                    DispatchMessageW(&message as *const MSG);
                }
            }
        }
    }
}

fn peek_message() -> Option<MSG> {
    let mut message = MaybeUninit::<MSG>::uninit();

    let has_message = unsafe {
        PeekMessageW(message.as_mut_ptr(), None, 0, 0, PM_REMOVE)
    };

    if !has_message.as_bool() {
        return None;
    }

    let message = unsafe {
        message.assume_init()
    };

    Some(message)
}

#[repr(C)]
struct Vertex {
    x: f32,
    y: f32,
    u: f32,
    v: f32,
    r: f32,
    g: f32,
    b: f32
}