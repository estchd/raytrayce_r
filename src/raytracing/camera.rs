use crate::raytracing::ray::Ray;
use crate::raytracing::vector_3d::Vec3;

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Camera {
	pub position: Vec3,
	pub look_direction: Vec3,
	pub up_direction: Vec3,
	pub field_of_view: f64,
	pub aspect_ratio: f64,
	pub focus_distance: f64,
	pub aperture: f64,
}

impl Camera {
	pub fn cast_ray(&self, s: f64, t: f64) -> Ray {
		let theta = self.field_of_view.to_radians();
		let height = (theta / 2.0).tan();

		let viewport_height = 2.0 * height;
		let viewport_width = self.aspect_ratio * viewport_height;

		let w = (self.position - self.look_direction).normalized();
		let u = Vec3::cross(self.up_direction, w).normalized();
		let v = Vec3::cross(w,u);

		let origin = self.position;
		let horizontal = u * viewport_width * self.focus_distance;
		let vertical = -v * viewport_height * self.focus_distance;
		let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * self.focus_distance;

		let lens_radius = self.aperture / 2.0;

		let rd = Vec3::random_in_unit_disk() * lens_radius;

		let offset = u * rd.x + v * rd.y;

		let direction = lower_left_corner + horizontal * s  + vertical * t - origin;

		Ray {
			origin: origin + offset,
			direction
		}
	}
}