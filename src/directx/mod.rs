pub mod device_context;
pub mod viewport;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct3D11::{D3D11_APPEND_ALIGNED_ELEMENT, D3D11_INPUT_ELEMENT_DESC, D3D11_INPUT_PER_VERTEX_DATA, D3D11_VIEWPORT, ID3D11Device, ID3D11DeviceContext, ID3D11Resource};
use windows::Win32::Graphics::Dxgi::IDXGISwapChain;
use windows::core::{Interface, PCSTR};
use std::fs::File;
use ascii::AsciiString;
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_R32G32_FLOAT, DXGI_FORMAT_R32G32B32_FLOAT, DXGI_FORMAT_R32G32B32A32_FLOAT};
use std::intrinsics::copy;
use std::io::Read;
use std::str::FromStr;
use crate::raytracing::color::Color;
use crate::raytracing::texture::Texture;
use crate::rendering::{bind_background_texture, bind_index_buffer, bind_input_layout, bind_pixel_shader, bind_render_target_view, bind_vertex_buffer, bind_vertex_shader, clear_render_target_view, create_back_buffer, create_background_texture, create_background_texture_resource_view, create_index_buffer, create_input_layout, create_pixel_shader, create_render_target_view, create_vertex_buffer, create_vertex_shader, draw_indexed, map_background_texture, present, set_primitive_topology, set_viewport, setup_directx_device_and_swapchain, unmap_background_texture};
use crate::rendering::win32::dxgidebug::dump_debug_messages;
use crate::rendering::win32::errhandlingapi::get_last_error;
use crate::window::Window;

pub struct DirectX {
	pub device: ID3D11Device,
	pub background_texture_resource: ID3D11Resource,
	pub swap_chain: IDXGISwapChain,
	pub device_context: ID3D11DeviceContext,
}

impl DirectX {
	pub fn create(width: u32, height: u32, window: &Window) -> Self {
		let raw_window_handle = window.window.raw_window_handle();
		let window_handle = if let RawWindowHandle::Windows(handle) = raw_window_handle {
			HWND(handle.hwnd as isize)
		}
		else {
			panic!("Invalid Window Handle Type");
		};

		let vertex_shader_data = load_vertex_shader_data();
		let pixel_shader_data = load_pixel_shader_data();

		let (
			_feature_level,
			swap_chain,
			device,
			device_context
		) = setup_directx_device_and_swapchain(width, height, window_handle).unwrap();

		let viewport = create_viewport(width, height);

		set_viewport(viewport, &device_context);

		let vertices = create_vertex_buffer_data();

		let vertex_buffer = create_vertex_buffer(&vertices, &device).unwrap();
		bind_vertex_buffer::<Vertex>(vertex_buffer, &device_context);

		let triangle_indices = create_index_buffer_data();

		let index_buffer = create_index_buffer(&triangle_indices, &device).unwrap();
		bind_index_buffer(&index_buffer, &device_context);

		let (input_element_descriptions, _) = create_input_layout_description();

		let input_layout = create_input_layout(&input_element_descriptions, &vertex_shader_data, &device);

		let input_layout = match input_layout {
			Ok(layout) => layout,
			Err(err) => {
				dump_debug_messages();
				eprintln!("last error: {:?}", get_last_error());
				panic!("{}", err);
			}
		};

		bind_input_layout(&input_layout, &device_context);

		set_primitive_topology(&device_context);

		let vertex_shader = create_vertex_shader(&vertex_shader_data, &device);

		let vertex_shader = match vertex_shader {
			Ok(shader) => shader,
			Err(err) => {
				dump_debug_messages();
				eprintln!("last error: {:?}", get_last_error());
				panic!("{}", err);
			}
		};

		bind_vertex_shader(&vertex_shader, &device_context);

		let pixel_shader = create_pixel_shader(&pixel_shader_data, &device).unwrap();

		bind_pixel_shader(&pixel_shader, &device_context);

		let background_texture = create_background_texture(width, height, &device).unwrap();
		let background_texture_resource: ID3D11Resource = background_texture.cast::<ID3D11Resource>().unwrap();

		Self {
			device,
			background_texture_resource,
			swap_chain,
			device_context
		}
	}

	pub fn start_frame(&self, texture: &Texture) {
		draw_background_texture(&texture, &self.background_texture_resource, &self.swap_chain, &self.device, &self.device_context);
	}

	pub fn end_frame(&self) {
		present(1,0, &self.swap_chain).unwrap();
	}
}

fn load_vertex_shader_data() -> Vec<u8> {
    let mut vertex_shader_file = File::open("shaders/vertex_shader.cso").unwrap();

    let mut vertex_shader_data = Vec::new();
    vertex_shader_file.read_to_end(&mut vertex_shader_data).unwrap();
    vertex_shader_data
}

fn load_pixel_shader_data() -> Vec<u8> {
    let mut pixel_shader_file = File::open("shaders/pixel_shader.cso").unwrap();
    let mut pixel_shader_data = Vec::new();
    pixel_shader_file.read_to_end(&mut pixel_shader_data).unwrap();
    pixel_shader_data
}

fn create_viewport(width: u32, height: u32) -> D3D11_VIEWPORT {
    D3D11_VIEWPORT {
        TopLeftX: 0.0,
        TopLeftY: 0.0,
        Width: width as f32,
        Height: height as f32,
        MinDepth: 0.0,
        MaxDepth: 0.0
    }
}

fn create_index_buffer_data() -> Vec<u32> {
    vec![0,1,2,3,2,1]
}

fn create_vertex_buffer_data() -> Vec<Vertex> {
    vec![
        Vertex {
            x: -1.0,
            y: 1.0,
            z: 0.0,
            u: 0.0,
            v: 0.0,
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
        Vertex {
            x: 1.0,
            y: 1.0,
            z: 0.0,
            u: 1.0,
            v: 0.0,
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        },
        Vertex {
            x: -1.0,
            y: -1.0,
            z: 0.0,
            u: 0.0,
            v: 1.0,
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0
        },
        Vertex {
            x: 1.0,
            y: -1.0,
            z: 0.0,
            u: 1.0,
            v: 1.0,
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
    ]
}

fn create_input_layout_description() -> (Vec<D3D11_INPUT_ELEMENT_DESC>, Vec<Vec<u8>>) {
    let position_element_name = AsciiString::from_str("POSITION").unwrap();
    let mut position_element_name_bytes: Vec<u8> = position_element_name.as_bytes().iter().map(|item| *item).collect();
    position_element_name_bytes.push(0);
    let position_element_description = D3D11_INPUT_ELEMENT_DESC {
        SemanticName: PCSTR(position_element_name_bytes.as_ptr()),
        SemanticIndex: 0,
        Format: DXGI_FORMAT_R32G32B32_FLOAT,
        InputSlot: 0,
        AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
        InstanceDataStepRate: 0
    };
    let uv_element_name = AsciiString::from_str("TEXCOORD").unwrap();
    let mut uv_element_name_bytes: Vec<u8> = uv_element_name.as_bytes().iter().map(|item| *item).collect();
    uv_element_name_bytes.push(0);
    let uv_element_description = D3D11_INPUT_ELEMENT_DESC {
        SemanticName: PCSTR(uv_element_name_bytes.as_ptr()),
        SemanticIndex: 0,
        Format: DXGI_FORMAT_R32G32_FLOAT,
        InputSlot: 0,
        AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
        InstanceDataStepRate: 0
    };
    let color_element_name = AsciiString::from_str("COLOR").unwrap();
    let mut color_element_name_bytes: Vec<u8> = color_element_name.as_bytes().iter().map(|item| *item).collect();
    color_element_name_bytes.push(0);
    let color_element_description = D3D11_INPUT_ELEMENT_DESC {
        SemanticName: PCSTR(color_element_name_bytes.as_ptr()),
        SemanticIndex: 0,
        Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
        InputSlot: 0,
        AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
        InstanceDataStepRate: 0
    };

    let input_element_descriptions = vec![
        position_element_description,
        uv_element_description,
        color_element_description
    ];

    let bytes = vec![
        position_element_name_bytes,
        uv_element_name_bytes,
        color_element_name_bytes
    ];

    (input_element_descriptions, bytes)
}

fn fill_texture_test(texture: &mut Texture) {
    let width = texture.get_width().get();
    let height = texture.get_height().get();

    let max = width + height;

    let min_color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0
    };

    let max_color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0
    };

    for x in 0..width {
        for y in 0..height {
            let position = (x + y) as f32 / max as f32;

            let color = blend_colors(min_color, max_color, position);

            texture.set_pixel(x,y,color);
        }
    }
}

fn blend_colors(color_a: Color, color_b: Color, pos: f32) -> Color {
    let pos = pos.clamp(0.0,1.0);

    let r = color_a.r + color_b.r * pos;
    let g = color_a.g + color_b.g * pos;
    let b = color_a.b + color_b.b * pos;
    let a = color_a.a + color_b.a * pos;

    Color {
        r,
        g,
        b,
        a
    }
}

fn draw_background_texture(texture: &Texture, background_texture_resource: &ID3D11Resource, swap_chain: &IDXGISwapChain, device: &ID3D11Device, device_context: &ID3D11DeviceContext) {
    let back_buffer = create_back_buffer(&swap_chain).unwrap();
    let render_target_view = create_render_target_view(&back_buffer, &device).unwrap();
    let render_target_view = bind_render_target_view(render_target_view, &device_context);

    let clear_color: [f32; 4] = [0.0,0.0,0.5,1.0];
    clear_render_target_view(clear_color, &render_target_view, &device_context);

    let mapped_background_texture = map_background_texture(background_texture_resource, &device_context);

    let mapped_background_texture = match mapped_background_texture {
        Ok(texture) => texture,
        Err(err) => {
            dump_debug_messages();
            eprintln!("last error: {:?}", get_last_error());
            panic!("{}", err);
        }
    };

    let data = texture.get_raw();

    unsafe {
        copy(data.as_ptr(), mapped_background_texture.pData as *mut f32, data.len());
    }

    unmap_background_texture(mapped_background_texture, background_texture_resource, &device_context);

    let background_texture_resource_view = create_background_texture_resource_view(background_texture_resource,&device);
    let background_texture_resource_view = match background_texture_resource_view {
        Ok(view) => view,
        Err(err) => {
            dump_debug_messages();
            eprintln!("last error: {:?}", get_last_error());
            panic!("{}", err);
        }
    };
    let _background_texture_resource_view = bind_background_texture(background_texture_resource_view, &device_context);

    draw_indexed(6, 0, 0, &device_context);
}

#[repr(C)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
    u: f32,
    v: f32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}
