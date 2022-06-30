use std::num::NonZeroU32;
use std::sync::Arc;
use crate::raytracing::color::Color;
use crate::raytracing::raytracer::{Raytracer, RaytracerSettings, RaytracerState};
use crate::raytracing::scene::RaytracingScene;
use crate::raytracing::texture::{Texture, TextureWrapMode};

pub struct GPURaytracer {
	texture: Texture,
	settings: RaytracerSettings,
	scene: Arc<RaytracingScene>
}

impl GPURaytracer {
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
			scene
		}
	}
}

impl Raytracer for GPURaytracer {
	fn start_rendering(&mut self) {
		todo!()
	}

	fn stop_rendering(&mut self) {
		todo!()
	}

	fn restart_rendering(&mut self) {
		todo!()
	}

	fn pause_rendering(&mut self) {
		todo!()
	}

	fn continue_rendering(&mut self) {
		todo!()
	}

	fn update(&mut self) {
		todo!()
	}

	fn change_settings(&mut self, new_settings: RaytracerSettings) {
		self.settings = new_settings;
	}

	fn set_scene(&mut self, scene: Arc<RaytracingScene>) {
		self.scene = scene;
	}

	fn get_state(&self) -> RaytracerState {
		todo!()
	}

	fn get_current_texture(&self) -> &Texture {
		todo!()
	}
}