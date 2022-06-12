#![allow(dead_code)]

use std::fs::File;
use std::io::Read;
use std::mem::MaybeUninit;
use std::num::NonZeroU32;
use std::ptr::copy;
use std::str::FromStr;
use std::time::Instant;
use ascii::AsciiString;
use imgui::{Condition, Context};
use imgui_dx11_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use windows::core::PCSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct3D11::{D3D11_APPEND_ALIGNED_ELEMENT, D3D11_INPUT_ELEMENT_DESC, D3D11_INPUT_PER_VERTEX_DATA, D3D11_VIEWPORT, ID3D11Device, ID3D11DeviceContext, ID3D11Resource};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_R32G32_FLOAT, DXGI_FORMAT_R32G32B32_FLOAT, DXGI_FORMAT_R32G32B32A32_FLOAT};
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, MSG, PeekMessageW, PM_REMOVE, TranslateMessage, WM_QUIT};
use winit::dpi::{PhysicalSize, Size};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use raytracing::color::Color;
use crate::rendering::{bind_background_texture, bind_index_buffer, bind_input_layout, bind_pixel_shader, bind_render_target_view, bind_vertex_buffer, bind_vertex_shader, clear_render_target_view, create_back_buffer, create_background_texture, create_background_texture_resource_view, create_index_buffer, create_input_layout, create_pixel_shader, create_render_target_view, create_vertex_buffer, create_vertex_shader, draw_indexed, map_background_texture, present, set_primitive_topology, set_viewport, setup_directx_device_and_swapchain, unmap_background_texture};
use crate::rendering::win32::dxgidebug::dump_debug_messages;
use crate::rendering::win32::errhandlingapi::get_last_error;
use raytracing::texture::{Texture, TextureWrapMode};
use windows::core::Interface;
use windows::Win32::Graphics::Dxgi::IDXGISwapChain;
use workers_pool::WorkersPool;
use raytracing::{RaytracingContext, RaytracingWorkData};
use raytracing::scene::RaytracingScene;
use crate::image::save_texture_to_path;
use crate::raytracing::{MAX_BOUNCES, RaytracingWorker, SAMPLES_PER_PIXEL};
use crate::raytracing::vector_3d::Vec3;

mod rendering;
mod image;
mod raytracing;

pub const CLASS_NAME: &str = "raytrace_window_class";
pub const WINDOW_NAME: &str = "Raytrace Window";
pub const WINDOW_POSITION: (u32, u32) = (50, 50);
pub const WINDOW_SIZE: (u32, u32) = (1200, 800);
pub const TILE_COUNT: (u32, u32) = (16,16);

fn main() {
    let image_width = WINDOW_SIZE.0;
    let image_height = WINDOW_SIZE.1;

    let winit_event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(
            Size::Physical(
                PhysicalSize::new(
                    image_width,
                    image_height
                )
            )
        )
        .with_title("Raytrace Window")
        .with_resizable(false)
        .build(&winit_event_loop).unwrap();

    let mut imgui_context = imgui::Context::create();

    let mut imgui_winit_platform = WinitPlatform::init(&mut imgui_context);
    imgui_winit_platform.attach_window(imgui_context.io_mut(), &window, HiDpiMode::Default);

    let raw_window_handle = window.raw_window_handle();
    let window_handle = if let RawWindowHandle::Windows(handle) = raw_window_handle {
        HWND(handle.hwnd as isize)
    }
    else {
        panic!("Invalid Window Handle Type");
    };

    let vertex_shader_data = load_vertex_shader_data();
    let pixel_shader_data = load_pixel_shader_data();

    //let _window_class_atom = create_and_register_window_class().unwrap();
    //let window_handle = create_and_show_window().unwrap();

    let (
        _feature_level,
        swap_chain,
        device,
        device_context
    ) = setup_directx_device_and_swapchain(window_handle).unwrap();

    let viewport = create_viewport();

    let _viewport = set_viewport(viewport, &device_context);

    let vertices = create_vertex_buffer_data();

    let vertex_buffer = create_vertex_buffer(&vertices, &device).unwrap();
    let _vertex_buffer = bind_vertex_buffer::<Vertex>(vertex_buffer, &device_context);

    let triangle_indices = create_index_buffer_data();

    let index_buffer = create_index_buffer(&triangle_indices, &device).unwrap();
    bind_index_buffer(&index_buffer, &device_context);

    let (input_element_descriptions, _input_element_description_bytes) = create_input_layout_description();

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

    let mut imgui_renderer = unsafe { imgui_dx11_renderer::Renderer::new(&mut imgui_context, &device) }.unwrap();

    let background_texture = create_background_texture(&device).unwrap();
    let background_texture_resource: ID3D11Resource = background_texture.cast::<ID3D11Resource>().unwrap();

    let mut texture = Texture::new(
        NonZeroU32::new(image_width).unwrap(),
        NonZeroU32::new(image_height).unwrap(),
        TextureWrapMode::Clamp,
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0
        }
    );

    //fill_texture_test(&mut texture);

    let mut last_frame = Instant::now();
    let mut current_computation_workers: Option<WorkersPool<RaytracingWorker>> = None;
    let mut commissioned_count: usize = 0;
    let mut completed_count: usize = 0;
    let mut imgui_state = ImguiState {
        completed_count,
        commissioned_count,
        image_path: "".to_string(),
        combo_open: false,
        selected: 0
    };

    let aspect_ratio = image_width as f64 / image_height as f64;

    let scene = RaytracingScene::create_scene(aspect_ratio);
    
    let raytracing_context = RaytracingContext {
        image_width,
        image_height,
        samples_per_pixel: SAMPLES_PER_PIXEL,
        max_bounces: MAX_BOUNCES,
        scene
    };

    winit_event_loop.run(move |event, _window_target, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::NewEvents(_) => {
                let duration = last_frame.elapsed();
                last_frame = Instant::now();
                imgui_context.io_mut().update_delta_time(duration);
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                dump_debug_messages();
                *control_flow = ControlFlow::Exit;
                return;
            }
            Event::MainEventsCleared => {}
            event => {
                imgui_winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
                return;
            }
        }

        draw_background_texture(&triangle_indices, &texture, &background_texture_resource, &swap_chain, &device, &device_context);

        let imgui_result = draw_imgui(&mut imgui_state, &window, &mut imgui_context, &mut imgui_renderer, &mut imgui_winit_platform);

        if imgui_result.render_start_button_clicked {
            start_rendering(&mut texture, &mut commissioned_count,WINDOW_SIZE.0, WINDOW_SIZE.1, raytracing_context.clone(), &mut current_computation_workers);
            completed_count = 0;

            imgui_state.completed_count = 0;
            imgui_state.commissioned_count = commissioned_count;
        }

        if imgui_result.render_stop_button_clicked {
            stop_rendering(&mut current_computation_workers);
        }

        if imgui_result.export_button_clicked {
            save_texture_to_path(&imgui_state.image_path, &texture).unwrap();
        }

        if let Some(workers) = &mut current_computation_workers {
            update_texture(&mut completed_count, &mut texture, workers);
            imgui_state.completed_count = completed_count;

            if completed_count >= commissioned_count {
                stop_rendering(&mut current_computation_workers);
            }
        }

        present(1,0, &swap_chain).unwrap();
    });

}

struct ImguiState {
    completed_count: usize,
    commissioned_count: usize,
    image_path: String,
    combo_open: bool,
    selected: usize
}

struct ImguiResult {
    render_start_button_clicked: bool,
    render_stop_button_clicked: bool,
    export_button_clicked: bool,
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
    z: f32,
    u: f32,
    v: f32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
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

fn create_viewport() -> D3D11_VIEWPORT {
    D3D11_VIEWPORT {
        TopLeftX: 0.0,
        TopLeftY: 0.0,
        Width: WINDOW_SIZE.0 as f32,
        Height: WINDOW_SIZE.1 as f32,
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

fn draw_background_texture(triangle_indices: &[u32], texture: &Texture, background_texture_resource: &ID3D11Resource, swap_chain: &IDXGISwapChain, device: &ID3D11Device, device_context: &ID3D11DeviceContext) {
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

    draw_indexed(triangle_indices.len() as u32, 0, 0, &device_context);

}

fn draw_imgui(imgui_state: &mut ImguiState, window: &Window, imgui_context: &mut Context, imgui_renderer: &mut Renderer, imgui_winit_platform: &mut WinitPlatform) -> ImguiResult {
    imgui_winit_platform.prepare_frame(imgui_context.io_mut(), &window).unwrap();
    let ui = imgui_context.frame();

    let mut render_start_button_clicked: bool = false;
    let mut render_stop_button_clicked: bool = false;
    let mut export_button_clicked: bool = false;

    imgui::Window::new("Imgui Window")
        .size([300.0, 100.0], Condition::FirstUseEver)
        .build(&ui, || {
            imgui::InputText::new(&ui, "Image Path", &mut imgui_state.image_path)
                .build();
            export_button_clicked = ui.button("Export to image");
            render_start_button_clicked = ui.button("Start Render");
            render_stop_button_clicked = ui.button("Stop Render");
            ui.text("Rendering:");
            if imgui_state.completed_count >= imgui_state.commissioned_count {
                ui.text("Done!");
            }
            else {
                let percentage = (imgui_state.completed_count as f32 / imgui_state.commissioned_count as f32) * 100.0;
                let text = format!("{}/{} : {}%", imgui_state.completed_count, imgui_state.commissioned_count, percentage);

                ui.text(text);
            }

            imgui_state.combo_open = ui.combo_simple_string("Select Test", &mut imgui_state.selected, &["First", "Second", "Banana"]);
        });

    imgui_winit_platform.prepare_render(&ui, &window);
    imgui_renderer.render(ui.render()).unwrap();

    ImguiResult {
        render_start_button_clicked,
        render_stop_button_clicked,
        export_button_clicked
    }
}

fn start_rendering(texture: &mut Texture, commissioned_count: &mut usize, width: u32, height: u32, context: RaytracingContext, workers: &mut Option<WorkersPool<RaytracingWorker>>) {
    *workers = None;

    texture.clear(Color::create(0.0,0.0,0.0,1.0));

    let mut new_workers = WorkersPool::new(context);

    /*
    // Per Pixel Task
    for y in 0..height {
        for x in 0..width {
            let work_data = RaytracingWorkData {
                x,
                y
            };

            new_workers.add_work(work_data).unwrap();
        }
    }
     */

    // Per Tile Task

    let tile_width = width / TILE_COUNT.0;
    let tile_height = height / TILE_COUNT.1;

    let last_tile_width = tile_width + (width % tile_width);
    let last_tile_height = tile_height + (height % tile_height);

    let right_border_x = tile_width * (TILE_COUNT.0 - 1);
    let bottom_border_y = tile_height * (TILE_COUNT.1 - 1);

    // Bottom Right Tile

    let task = RaytracingWorkData {
        x: right_border_x,
        y: bottom_border_y,
        width: last_tile_width,
        height: last_tile_height
    };

    new_workers.add_work(task).unwrap();

    // Bottom Border Tiles
    for tile_x in 0..(TILE_COUNT.0 - 1) {
        let x = tile_x * tile_width;

        let task = RaytracingWorkData {
            x,
            y: bottom_border_y,
            width: tile_width,
            height: last_tile_height
        };

        new_workers.add_work(task).unwrap();
    }

    // Right Border Tiles
    for tile_y in 0..(TILE_COUNT.1 - 1) {
        let y = tile_y * tile_height;

        let task = RaytracingWorkData {
            x: right_border_x,
            y,
            width: last_tile_width,
            height: tile_height
        };

        new_workers.add_work(task).unwrap();
    }



    // Main Grid Tiles
    for tile_y in 0..(TILE_COUNT.1 - 1) {
        for tile_x in 0..(TILE_COUNT.0 - 1) {
            let x = tile_x * tile_width;
            let y = tile_y * tile_height;

            let task = RaytracingWorkData {
                x,
                y,
                width: tile_width,
                height: tile_height
            };

            new_workers.add_work(task).unwrap();
        }
    }

    *commissioned_count = width as usize * height as usize;
    *workers = Some(new_workers);
}

fn stop_rendering(workers: &mut Option<WorkersPool<RaytracingWorker>>) {
    *workers = None;
}

fn update_texture(completed_count: &mut usize, texture: &mut Texture, workers: &mut WorkersPool<RaytracingWorker>) {
    let results = workers.collect_finished().unwrap();

    for result in results {
        texture.set_pixel(result.x, result.y, result.pixel_color);
        *completed_count = *completed_count + 1;
    }
}
