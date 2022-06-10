use std::num::{NonZeroU32};
use crate::color::Color;
use crate::pixel::{MutPixel, Pixel};
use arrayref::array_ref;
use arrayref::array_mut_ref;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TextureWrapMode {
	None,
	Clamp,
	Repeat,
	Mirror,
}

#[derive(Clone)]
pub struct Texture {
	width: NonZeroU32,
	height: NonZeroU32,
	values: Vec<f32>,
	wrap_mode: TextureWrapMode
}

impl Texture {
	pub fn new(width: NonZeroU32, height: NonZeroU32, wrap_mode: TextureWrapMode, fill_color: Color) -> Self {
		let entry_count = width.get() * height.get();

		let fill_values = [fill_color.r, fill_color.g, fill_color.b, fill_color.a];

		let values = fill_values.repeat(entry_count as usize);

		Self {
			width,
			height,
			values,
			wrap_mode
		}
	}

	pub fn get_width(&self) -> NonZeroU32 {
		self.width
	}

	pub fn get_height(&self) -> NonZeroU32 {
		self.height
	}

	pub fn get_wrap_mode(&self) -> TextureWrapMode {
		self.wrap_mode
	}

	pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
		let wrapped_coordinates = self.get_wrapped_coordinates(x,y);
		let (wrapped_x, wrapped_y) = match wrapped_coordinates {
			None => {
				return;
			}
			Some((x,y)) => {
				(x,y)
			}
		};

		let index = self.get_index_from_coordinates(wrapped_x,wrapped_y);

		self.values[index] = color.r;
		self.values[index + 1] = color.g;
		self.values[index + 2] = color.b;
		self.values[index + 3] = color.a;
	}

	pub fn get_pixel(&self, x: u32, y: u32) -> Option<Pixel> {
		let (wrapped_x, wrapped_y) = self.get_wrapped_coordinates(x,y)?;

		let index = self.get_index_from_coordinates(wrapped_x,wrapped_y);

		let values = array_ref![&self.values, index, 4];

		Some(values.into())
	}

	pub fn get_pixel_mut(&mut self, x: u32, y: u32) -> Option<MutPixel> {
		let (wrapped_x, wrapped_y) = self.get_wrapped_coordinates(x,y)?;

		let index = self.get_index_from_coordinates(wrapped_x, wrapped_y);

		let values = array_mut_ref![&mut self.values, index, 4];

		Some(values.into())
	}

	pub fn get_raw(&self) -> &[f32] {
		&self.values
	}

	pub fn get_raw_mut(&mut self) -> &mut [f32] {
		&mut self.values
	}

	fn get_wrapped_coordinates(&self, x: u32, y: u32) -> Option<(u32,u32)> {
		let wrapped_x = get_wrapped_coordinate(x, self.width, self.wrap_mode);
		let wrapped_y = get_wrapped_coordinate(y, self.height, self.wrap_mode);

		wrapped_x.zip(wrapped_y)
	}

	fn get_index_from_coordinates(&self, x: u32, y: u32) -> usize {
		((y as usize * self.width.get() as usize) + x as usize) * 4
	}
}

fn get_wrapped_coordinate(coordinate: u32, bound: NonZeroU32, wrap_mode: TextureWrapMode) -> Option<u32> {
	if coordinate < bound.get() {
		return Some(coordinate);
	}

	match wrap_mode {
		TextureWrapMode::None => {
			None
		}
		TextureWrapMode::Clamp => {
			Some(bound.get() - 1)
		}
		TextureWrapMode::Repeat => {
			Some(coordinate % bound)
		}
		TextureWrapMode::Mirror => {
			let coordinate = coordinate % (bound.get() * 2);
			if coordinate >= bound.get() {
				Some((bound.get() * 2) - coordinate)
			}
			else {
				Some(coordinate)
			}
		}
	}
}