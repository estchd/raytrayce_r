use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Instant;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow};
use workers_pool::WorkersPool;
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, MSG, PeekMessageW, PM_REMOVE, TranslateMessage, WM_QUIT};
use std::mem::MaybeUninit;
use crate::directx::DirectX;
use crate::gui::{GUI, GUIModeTree};
use crate::image::save_texture_to_path;
use crate::raytracing::color::Color;
use crate::raytracing::{RaytracingContext, RaytracingWorker};
use crate::raytracing::scene::RaytracingScene;
use crate::raytracing::texture::{Texture, TextureWrapMode};
use crate::raytracing::work::generator::{GenerationMode, RaytracingWorkGenerator};
use crate::rendering::win32::dxgidebug::dump_debug_messages;
use crate::window::Window;

pub struct RaytracingApplicationSettings {
	pub width: u32,
	pub height: u32,
	pub x: i32,
	pub y: i32,
	pub window_name: String
}

pub struct RaytracingApplication {
	width: u32,
	height: u32,
	window: Window,
	directx: DirectX
}

impl RaytracingApplication {
	pub fn create(settings: RaytracingApplicationSettings) -> Self {
		let RaytracingApplicationSettings {
			width,
			height,
			x,
			y,
			window_name
		} = settings;

		let window = Window::create(width, height, x, y, &window_name);

		let directx= DirectX::create(width, height, &window);

		Self {
			width,
			height,
			window,
			directx
		}
	}

	pub fn run(self) -> ! {
		let aspect_ratio = self.width as f64 / self.height as f64;

		let scene = Arc::new(RaytracingScene::create_scene(aspect_ratio));

		let mut texture = Texture::new(
			NonZeroU32::new(self.width).unwrap(),
			NonZeroU32::new(self.height).unwrap(),
			TextureWrapMode::Clamp,
			Color {
				r: 0.0,
				g: 0.0,
				b: 1.0,
				a: 0.0
			}
		);

		let mut last_frame = Instant::now();
		let mut current_computation_workers: Option<WorkersPool<RaytracingWorker>> = None;
		let mut gui = GUI::new(&self.window, &self.directx.device);

		self.window.event_loop.run(move |event, _window_target, control_flow| {
			*control_flow = ControlFlow::Poll;

			match event {
				Event::NewEvents(_) => {
					let duration = last_frame.elapsed();
					last_frame = Instant::now();
					gui.update_delta(duration);
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
					gui.handle_event(event);
					return;
				}
			}

			self.directx.start_frame(&texture);

			let imgui_result = gui.draw();

			if imgui_result.render_start_button_clicked {
				let samples_per_pixel = gui.state().samples_per_pixel;
				let max_bounces = gui.state().max_bounces;

				let gui_state_mut = gui.state_mut();
				let commissioned_count = &mut gui_state_mut.commissioned_count;
				let mode_tree = &mut gui_state_mut.mode_tree;

				start_rendering(&mut texture, commissioned_count, self.width, self.height, samples_per_pixel, max_bounces, scene.clone(), mode_tree, &mut current_computation_workers);

				gui.state_mut().completed_count = 0;
			}

			if imgui_result.render_stop_button_clicked {
				stop_rendering(&mut current_computation_workers);
			}

			if imgui_result.export_button_clicked {
				save_texture_to_path(&gui.state().image_path, &texture).unwrap();
			}

			if let Some(workers) = &mut current_computation_workers {
				update_texture(&mut gui.state_mut().completed_count, &mut texture, workers);

				if gui.state().completed_count >= gui.state().commissioned_count {
					stop_rendering(&mut current_computation_workers);
				}
			}

			self.directx.end_frame();
		});
	}
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

fn start_rendering(texture: &mut Texture, commissioned_count: &mut usize, width: u32, height: u32, samples_per_pixel: usize, max_bounces: usize, scene: Arc<RaytracingScene>, gui_mode_tree: &GUIModeTree, workers: &mut Option<WorkersPool<RaytracingWorker>>) {
    *workers = None;

    texture.clear(Color::create(0.0,0.0,0.0,1.0));

    let context = RaytracingContext {
        image_width: width,
        image_height: height,
        samples_per_pixel,
        max_bounces,
        scene
    };

    let mut new_workers = WorkersPool::new(context);

    let generation_mode = GenerationMode::from_gui_mode_tree(gui_mode_tree);

    let generator = RaytracingWorkGenerator {
        width,
        height,
        generation_mode
    };

    generator.generate(&mut new_workers).unwrap();

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
