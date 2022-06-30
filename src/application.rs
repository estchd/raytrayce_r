use std::sync::Arc;
use std::time::Instant;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow};
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, MSG, PeekMessageW, PM_REMOVE, TranslateMessage, WM_QUIT};
use std::mem::MaybeUninit;
use crate::directx::DirectX;
use crate::gui::{GUI};
use crate::image::save_texture_to_path;
use crate::raytracing::raytracer::cpu_raytracer::CPURaytracer;
use crate::raytracing::raytracer::{Raytracer, RaytracerSettings};
use crate::raytracing::raytracer::RaytracerState::Running;
use crate::raytracing::scene::RaytracingScene;
use crate::raytracing::work::generator::{GenerationMode};
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

		let mut last_frame = Instant::now();
		let mut gui = GUI::new(&self.window, &self.directx.device);

		let generation_mode = GenerationMode::from_gui_mode_tree(&gui.state().mode_tree);

		let settings = RaytracerSettings {
			width: self.width,
			height: self.height,
			samples_per_pixel: gui.state().samples_per_pixel,
			max_bounces: gui.state().max_bounces,
			generation_mode
		};

		let mut raytracer: Box<dyn Raytracer> = Box::new(CPURaytracer::new(settings, scene.clone()));
		gui.update_raytracer_state(&raytracer);

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

			self.directx.start_frame(raytracer.get_current_texture());

			gui.update_raytracer_state(&raytracer);

			let imgui_result = gui.draw();

			if imgui_result.render_start_button_clicked {
				let samples_per_pixel = gui.state().samples_per_pixel;
				let max_bounces = gui.state().max_bounces;

				let gui_state_mut = gui.state_mut();
				let mode_tree = &mut gui_state_mut.mode_tree;

				let generation_mode = GenerationMode::from_gui_mode_tree(mode_tree);

				let settings = RaytracerSettings {
					width: self.width,
					height: self.height,
					samples_per_pixel,
					max_bounces,
					generation_mode
				};

				raytracer.change_settings(settings);
				raytracer.set_scene(scene.clone());
				raytracer.start_rendering();
			}

			if imgui_result.render_stop_button_clicked {
				raytracer.stop_rendering();
			}

			if imgui_result.export_button_clicked {
				save_texture_to_path(&gui.state().image_path, raytracer.get_current_texture()).unwrap();
			}

			if let Running { .. } = raytracer.get_state() {
				raytracer.update();
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