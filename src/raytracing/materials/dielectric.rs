use rand::{Rng, thread_rng};
use crate::{Color, Vec3};
use crate::raytracing::hit_record::HitRecord;
use crate::raytracing::materials::{Material};
use crate::raytracing::materials::util::{reflect, reflectance, refract};
use crate::raytracing::ray::Ray;

pub struct Dielectric {
	index_of_refraction: f64
}

impl Dielectric {
	pub fn new() -> Self {
		Self {
			index_of_refraction: 0.0
		}
	}

	pub fn create(index_of_refraction: f64) -> Self {
		Self {
			index_of_refraction
		}
	}
}

impl Material for Dielectric {
	fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
		let refraction_ratio = if hit_record.front_face {
			1.0 / self.index_of_refraction
		}
		else {
			self.index_of_refraction
		};

		let unit_direction = ray.direction.normalized();

		let cos_theta = f64::min(Vec3::dot(-unit_direction, hit_record.normal), 1.0);
		let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

		let cannot_refract = refraction_ratio * sin_theta > 1.0;

		let mut rand = thread_rng();

		let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand.gen_range(0.0..1.0) {
			reflect(unit_direction, hit_record.normal)
		}
		else {
			refract(unit_direction, hit_record.normal, refraction_ratio)
		};

		let attenuation = Color::create(1.0,1.0,1.0,1.0);
		let ray = Ray {
			origin: hit_record.point,
			direction
		};

		Some((attenuation, ray))
	}
}