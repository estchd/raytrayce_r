use crate::raytracing::vector_3d::Vec3;

pub fn refract(direction: Vec3, normal: Vec3, etai_over_etat: f64) -> Vec3 {
	let cos_theta = f64::min(Vec3::dot(-direction, normal), 1.0);
	let out_perpendicular = (direction + normal * cos_theta) * etai_over_etat;
	let out_parallel = normal * -(1.0 - out_perpendicular.length_squared()).abs().sqrt();

	out_perpendicular + out_parallel
}

pub fn reflect(direction: Vec3, normal: Vec3) -> Vec3 {
	direction - normal * Vec3::dot(direction, normal)  * 2.0
}

pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
	let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
	let r0 = r0.powi(2);

	r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}