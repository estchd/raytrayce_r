use crate::{Color, Vec3};
use crate::raytracing::hit_record::HitRecord;
use crate::raytracing::materials::{Material, util};
use crate::raytracing::ray::Ray;

pub struct Metal {
	albedo: Color,
	fuzz: f64
}

impl Metal {
	pub fn new() -> Self {
		Self {
			albedo: Color::new(),
			fuzz: 0.0
		}
	}

	pub fn create(albedo: Color, fuzz: f64) -> Self {
		Self {
			albedo,
			fuzz
		}
	}
}

impl Material for Metal {
	fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
		let reflected = util::reflect(ray.direction.normalized(), hit_record.normal);
		let scattered = Ray {
			origin: hit_record.point,
			direction: reflected + Vec3::random_in_unit_sphere() * self.fuzz
		};

		if Vec3::dot(scattered.direction, hit_record.normal) < 0.0 {
			return None;
		}

		Some((self.albedo, scattered))
	}
}