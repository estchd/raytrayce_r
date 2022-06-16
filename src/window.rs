use std::sync::Arc;
use winit::dpi::{PhysicalPosition, PhysicalSize, Position, Size};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

pub struct Window {
	pub event_loop: EventLoop<()>,
	pub window: Arc<winit::window::Window>,
}

impl Window {
	pub fn create(width: u32, height: u32, x: i32, y: i32, name: &str) -> Self {
		let event_loop = EventLoop::new();
		let window = WindowBuilder::new()
			.with_inner_size(
				Size::Physical(
					PhysicalSize::new(
						width,
						height
					)
				)
			)
			.with_position(
				Position::Physical(
					PhysicalPosition::new(
						x as i32,
						y as i32
					)
				)
			)
			.with_title(name)
			.with_resizable(false)
			.build(&event_loop).unwrap();

		Self {
			event_loop,
			window: Arc::new(window)
		}
	}
}