use std::num::NonZeroU32;
use std::sync::Arc;
use workers_pool::WorkersPool;
use crate::raytracing::color::Color;
use crate::raytracing::raytracer::{Raytracer, RaytracerSettings, RaytracerState};
use crate::raytracing::{RaytracingContext, RaytracingWorker};
use crate::raytracing::scene::RaytracingScene;
use crate::raytracing::texture::{Texture, TextureWrapMode};
use crate::raytracing::work::generator::{RaytracingWorkGenerator};

pub struct CPURaytracer {
	texture: Texture,
	settings: RaytracerSettings,
	scene: Arc<RaytracingScene>,
	current_workers: Option<WorkersPool<RaytracingWorker>>,
	state: RaytracerState,
}

impl CPURaytracer {
	pub fn new(settings: RaytracerSettings, scene: Arc<RaytracingScene>) -> Self {
		let texture = Texture::new(
			NonZeroU32::new(settings.width).unwrap(),
			NonZeroU32::new(settings.height).unwrap(),
			TextureWrapMode::Clamp,
			Color::create(0.0,0.0,0.0,0.0)
		);

		Self {
			texture,
			settings,
			scene,
			current_workers: None,
			state: RaytracerState::Created
		}
	}
}

impl Raytracer for CPURaytracer {
	fn start_rendering(&mut self) {
		self.stop_rendering();

		self.texture.clear(Color::create(0.0,0.0,0.0,1.0));

		let context = RaytracingContext {
			image_width: self.settings.width,
			image_height: self.settings.height,
			samples_per_pixel: self.settings.samples_per_pixel,
			max_bounces: self.settings.max_bounces,
			scene: self.scene.clone()
		};

		let mut new_workers = WorkersPool::new(context);

		let generator = RaytracingWorkGenerator {
			width: self.settings.width,
			height: self.settings.height,
			generation_mode: self.settings.generation_mode.clone()
		};

		generator.generate(&mut new_workers).unwrap();

		self.state = RaytracerState::Running {
			commissioned: self.settings.width as usize * self.settings.height as usize,
			completed: 0
		};
		self.current_workers = Some(new_workers);
	}

	fn stop_rendering(&mut self) {
		self.current_workers = None;
		self.state = RaytracerState::Stopped;
	}

	fn restart_rendering(&mut self) {
		self.stop_rendering();
		self.start_rendering();
	}

	fn pause_rendering(&mut self) {
		todo!()
	}

	fn continue_rendering(&mut self) {
		todo!()
	}

	fn update(&mut self) {
		match self.state {
			RaytracerState::Created => {}
			RaytracerState::Running { commissioned, mut completed } => {
				let workers = self.current_workers.as_mut().unwrap();
				let results = workers.collect_finished().unwrap();

				for result in results {
					self.texture.set_pixel(result.x, result.y, result.pixel_color);
					completed += 1;
				}

				if completed >= commissioned {
					self.state = RaytracerState::Finished {
						commissioned,
						completed
					};
				}
				else {
					self.state = RaytracerState::Running {
						commissioned,
						completed
					};
				}
			}
			RaytracerState::Paused { .. } => {}
			RaytracerState::Finished { .. } => {}
			RaytracerState::Stopped => {}
		}


	}

	fn change_settings(&mut self, new_settings: RaytracerSettings) {
		self.settings = new_settings;
	}

	fn set_scene(&mut self, scene: Arc<RaytracingScene>) {
		self.scene = scene;
	}

	fn get_state(&self) -> RaytracerState {
		self.state
	}

	fn get_current_texture(&self) -> &Texture {
		&self.texture
	}
}