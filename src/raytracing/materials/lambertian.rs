use crate::{Color, NEAR_ZERO_THRESHOLD, Vec3};
use crate::raytracing::hit_record::HitRecord;
use crate::raytracing::materials::Material;
use crate::raytracing::ray::Ray;

pub struct Lambertian {
	pub albedo: Color
}

impl Lambertian {
	pub fn new() -> Self {
		Self {
			albedo: Color::new()
		}
	}

	pub fn create(albedo: Color) -> Self {
		Self {
			albedo
		}
	}
}

impl Material for Lambertian {
	fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
		let mut scatter_direction = hit_record.normal + Vec3::random_normalized();

		if scatter_direction.near_zero(NEAR_ZERO_THRESHOLD) {
			scatter_direction = hit_record.normal;
		}

		let scattered = Ray::create(hit_record.point, scatter_direction);
		Some((self.albedo, scattered))
	}
}