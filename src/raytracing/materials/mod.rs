pub mod lambertian;
pub mod metal;
pub mod dielectric;
pub mod util;

use crate::Color;
use crate::raytracing::hit_record::HitRecord;
use crate::raytracing::ray::Ray;

pub trait Material {
	fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}
