pub mod sphere;

use crate::raytracing::hit_record::HitRecord;
use crate::raytracing::ray::Ray;

pub trait Hittable {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}