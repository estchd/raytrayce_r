use image::{Rgba};
use crate::color::Color;

#[derive(Copy, Clone)]
pub struct Pixel<'a> {
	target: &'a [f32;4]
}

impl<'a> Pixel<'a> {
	pub fn get_r(&self) -> f32 {
		self.target[0]
	}

	pub fn get_g(&self) -> f32 {
		self.target[1]
	}

	pub fn get_b(&self) -> f32 {
		self.target[2]
	}

	pub fn get_a(&self) -> f32 {
		self.target[3]
	}

	pub fn to_rgba(&self) -> Rgba<f32> {
		Rgba::from(self.target.clone())
	}
}

impl<'a> From<&'a [f32; 4]> for Pixel<'a> {
	fn from(value: &'a [f32; 4]) -> Self {
		Self {
			target: value
		}
	}
}

pub struct MutPixel<'a> {
	target: &'a mut [f32;4]
}

impl<'a> MutPixel<'a> {
	pub fn get_r(&self) -> f32 {
		self.target[0]
	}

	pub fn set_r(&mut self, value: f32) {
		self.target[0] = value;
	}

	pub fn get_g(&self) -> f32 {
		self.target[1]
	}

	pub fn set_g(&mut self, value: f32) {
		self.target[1] = value;
	}

	pub fn get_b(&self) -> f32 {
		self.target[2]
	}

	pub fn set_b(&mut self, value: f32) {
		self.target[2] = value;
	}

	pub fn get_a(&self) -> f32 {
		self.target[3]
	}

	pub fn set_a(&mut self, value: f32) {
		self.target[3] = value;
	}

	pub fn set_color(&mut self, value: Color) {
		self.set_r(value.r);
		self.set_g(value.g);
		self.set_b(value.b);
		self.set_a(value.a);
	}

	pub fn to_rgba(&self) -> Rgba<f32> {
		Rgba::from(self.target.clone())
	}
}

impl<'a> From<&'a mut [f32;4]> for MutPixel<'a> {
	fn from(value: &'a mut [f32; 4]) -> Self {
		Self {
			target: value
		}
	}
}