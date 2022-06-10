use std::fmt::{Display, Formatter};
use image::Rgba;
use crate::raytracing::pixel::{MutPixel, Pixel};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32
}

impl Color {
	pub fn new() -> Self {
		Self {
			r: 0.0,
			g: 0.0,
			b: 0.0,
			a: 0.0
		}
	}

	pub fn create(r: f32, g: f32, b: f32, a: f32) -> Self {
		Self {
			r,
			g,
			b,
			a
		}
	}
}

impl Default for Color {
	fn default() -> Self {
		Self::new()
	}
}

impl Display for Color {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let r = self.r.clamp(0.0, 1.0);
		let g = self.g.clamp(0.0, 1.0);
		let b = self.b.clamp(0.0, 1.0);
		let a = self.a.clamp(0.0, 1.0);

		let r = (r * 255.0).round();
		let g = (g * 255.0).round();
		let b = (b * 255.0).round();
		let a = (a * 255.0).round();

		let r = r.clamp(0.0, 255.0);
		let g = g.clamp(0.0, 255.0);
		let b = b.clamp(0.0, 255.0);
		let a = a.clamp(0.0, 255.0);

		let r = r as u8;
		let g = g as u8;
		let b = b as u8;
		let a = a as u8;

		write!(f, "{} {} {} {}", r, g, b, a)
	}
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