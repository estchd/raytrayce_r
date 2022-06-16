use std::sync::Arc;
use crate::raytracing::hit_record::HitRecord;
use crate::raytracing::hittable::Hittable;
use crate::raytracing::materials::Material;
use crate::raytracing::ray::Ray;
use crate::raytracing::vector_3d::Vec3;

pub struct Sphere {
	pub center: Vec3,
	pub radius: f64,
	pub material:Arc<dyn Material + Send + Sync>
}

impl Sphere {

	pub fn create(center: Vec3, radius: f64, material: Arc<dyn Material + Send + Sync>) -> Self {
		Self {
			center,
			radius,
			material
		}
	}
}

impl Hittable for Sphere {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let origin_center = ray.origin - self.center;
		let a = ray.direction.length_squared();
		let half_b = Vec3::dot(origin_center, ray.direction);
		let c = origin_center.length_squared() - self.radius.powi(2);

		let discriminant = half_b.powi(2) - (a * c);
		if discriminant < 0.0 {
			return None;
		}

		let sqrt_discriminant = discriminant.sqrt();

		let mut root = (-half_b - sqrt_discriminant) / a;
		if root < t_min || t_max < root {
			root = (-half_b + sqrt_discriminant) / a;
			if root < t_min || t_max < root {
				return None;
			}
		}

		let hit_point = ray.at(root);

		let mut hit_record = HitRecord {
			point: hit_point,
			normal: (hit_point - self.center) / self.radius,
			material: Some(self.material.clone()),
			front_face: true,
			t: root
		};

		hit_record.set_face_normal(ray, hit_record.normal);

		return Some(hit_record);
	}
}