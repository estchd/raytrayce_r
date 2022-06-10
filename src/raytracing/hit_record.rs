use std::sync::Arc;
use crate::raytracing::materials::Material;
use crate::raytracing::ray::Ray;
use crate::Vec3;

#[derive(Clone)]
pub struct HitRecord {
	pub point: Vec3,
	pub normal: Vec3,
	pub material: Option<Arc<dyn Material + Send + Sync>>,
	pub front_face: bool,
	pub t: f64,
}

impl HitRecord {
	pub fn new() -> Self {
		Self {
			point: Vec3::create(0.0,0.0,0.0),
			normal: Vec3::create(0.0,0.0,0.0),
			material: None,
			front_face: false,
			t: 0.0
		}
	}

	pub fn set_face_normal(&mut self, ray: &Ray, normal: Vec3) {
		self.front_face = Vec3::dot(ray.direction, normal) < 0.0;

		self.normal = if self.front_face {
			normal
		}
		else {
			-normal
		}
	}
}

impl Default for HitRecord {
	fn default() -> Self {
		Self::new()
	}
}
