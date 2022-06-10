use image::Rgba;
use crate::pixel::{MutPixel, Pixel};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32
}

impl<'a> From<&Pixel<'a>> for Color {
	fn from(value: &Pixel<'a>) -> Self {
		Self {
			r: value.get_r(),
			g: value.get_g(),
			b: value.get_b(),
			a: value.get_a()
		}
	}
}

impl<'a> From<&MutPixel<'a>> for Color {
	fn from(value: &MutPixel<'a>) -> Self {
		Self {
			r: value.get_r(),
			g: value.get_g(),
			b: value.get_b(),
			a: value.get_a()
		}
	}
}

impl Into<Rgba<f32>> for Color {
	fn into(self) -> Rgba<f32> {
		let values = [self.r, self.g, self.b, self.a];

		Rgba::from(values)
	}
}