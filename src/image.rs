use std::path::Path;
use image::{ImageResult, Rgba, RgbaImage};
use crate::raytracing::texture::Texture;

pub fn save_texture_to_path<P: AsRef<Path>>(path: P, texture: &Texture) -> ImageResult<()> {
	let image = RgbaImage::from_fn(texture.get_width().get(), texture.get_height().get(), |x,y| {
		let pixel = texture.get_pixel(x,y).unwrap();
		let rgba_float = pixel.to_rgba();
		let rgba_float_values = rgba_float.0;

		let r = rgba_float_values[0].clamp(0.0,1.0);
		let g = rgba_float_values[1].clamp(0.0,1.0);
		let b = rgba_float_values[2].clamp(0.0,1.0);
		let a = rgba_float_values[3].clamp(0.0,1.0);

		let r = (r * 255.0).round() as u8;
		let g = (g * 255.0).round() as u8;
		let b = (b * 255.0).round() as u8;
		let a = (a * 255.0).round() as u8;

		let rgba_uint_values = [r,g,b,a];
		let rgba_uint = Rgba::<u8>::from(rgba_uint_values);
		rgba_uint
	});

	image.save(path)
}