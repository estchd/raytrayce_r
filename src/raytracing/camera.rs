use crate::raytracing::ray::Ray;
use crate::raytracing::vector_3d::Vec3;

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Camera {
	position: Vec3,
	lower_left_corner: Vec3,
	horizontal: Vec3,
	vertical: Vec3,
	lens_radius: f64,
}

impl Camera {
	pub fn new() -> Self {

		Self::create(
			Vec3::create(0.0,0.0,0.0),
			Vec3::create(0.0,0.0,-1.0),
			Vec3::create(0.0,1.0,0.0),
			90.0,
			1.0,
			1.0,
			0.01
		)
	}

	pub fn create(position: Vec3, look_at_point: Vec3, up_direction: Vec3, vertical_field_of_view: f64, aspect_ratio: f64, focus_distance: f64, aperture: f64) -> Self {
		let theta = vertical_field_of_view.to_radians();
		let height = (theta / 2.0).tan();

		let viewport_height = 2.0 * height;
		let viewport_width = aspect_ratio * viewport_height;

		let w = (position - look_at_point).normalized();
		let u = Vec3::cross(up_direction, w).normalized();
		let v = Vec3::cross(w,u);

		let origin = position;
		let horizontal = u * viewport_width * focus_distance;
		let vertical = -v * viewport_height * focus_distance;
		let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;

		let lens_radius = aperture / 2.0;
		
		Self {
			position,
			lower_left_corner,
			horizontal,
			vertical,
			lens_radius
		}
	}

	pub fn cast_ray(&self, u: f64, v: f64) -> Ray {
		let rd = Vec3::random_in_unit_disk() * self.lens_radius;

		let offset = u * rd.x + v * rd.y;

		let direction = self.lower_left_corner + self.horizontal * u  + self.vertical * v - self.position;

		Ray {
			origin: self.position + offset,
			direction
		}
	}
}