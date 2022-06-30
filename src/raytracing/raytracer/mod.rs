pub mod cpu_raytracer;
pub mod gpu_raytracer;

use std::sync::Arc;
use crate::raytracing::scene::RaytracingScene;
use crate::raytracing::texture::Texture;
use crate::raytracing::work::generator::GenerationMode;

use serde_derive::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RaytracerState {
	Created,
	Running {
		commissioned: usize,
		completed: usize,
	},
	Paused {
		commissioned: usize,
		completed: usize
	},
	Finished {
		commissioned: usize,
		completed: usize
	},
	Stopped,
}

pub trait Raytracer {
	fn start_rendering(&mut self);
	fn stop_rendering(&mut self);
	fn restart_rendering(&mut self);
	fn pause_rendering(&mut self);
	fn continue_rendering(&mut self);

	fn update(&mut self);

	fn change_settings(&mut self, new_settings: RaytracerSettings);
	fn set_scene(&mut self, scene: Arc<RaytracingScene>);

	fn get_state(&self) -> RaytracerState;
	fn get_current_texture(&self) -> &Texture;
}

#[derive(Clone, Debug)]
pub struct RaytracerSettings {
	pub width: u32,
	pub height: u32,
	pub samples_per_pixel: usize,
	pub max_bounces: usize,
	pub generation_mode: GenerationMode
}