use rand::prelude::SliceRandom;
use rand::thread_rng;
use workers_pool::WorkersPool;
use crate::gui::{GUIModeSettings, GUIModeTree};
use crate::raytracing::work::RaytracingWork;
use crate::RaytracingWorker;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TileAxisOrder {
	Forward,
	Reverse,
	Random
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TileDimensionOrder {
	LineFirst {
		horizontal_order: TileAxisOrder,
		vertical_order: TileAxisOrder
	},
	StripeFirst {
		horizontal_order: TileAxisOrder,
		vertical_order: TileAxisOrder
	},
	Random
}

#[derive(Debug, Clone)]
pub enum GenerationMode {
	PixelRandom,
	Line {
		reverse_order_horizontal: bool,
		random_order_vertical: bool,
		transparent: bool
	},
	Stripe {
		random_order_horizontal: bool,
		reverse_order_vertical: bool,
		transparent: bool
	},
	Tile {
		tile_width: u32,
		tile_height: u32,
		transparent: bool,
		dimension_order: TileDimensionOrder,
		sub_generation_mode: Box<GenerationMode>
	},
}

impl GenerationMode {
	pub fn to_work(&self, width: u32, height: u32) -> Vec<RaytracingWork> {
		let mut rand = thread_rng();

		let mut work = vec![];

		match self {
			GenerationMode::PixelRandom => {
				let mut coords: Vec<(u32,u32)> = Vec::with_capacity((width * height) as usize);

				for y in 0..height {
					for x in 0..width {
						coords.push((x,y));
					}
				}

				coords.shuffle(&mut rand);

				for (new_x,new_y) in coords {
					let new_work = RaytracingWork::pixel(new_x, new_y);
					work.push(new_work);
				}
			}
			GenerationMode::Line { reverse_order_horizontal, random_order_vertical, transparent } => {
				let mut line_order: Vec<u32> = (0..height).collect();

				if *random_order_vertical {
					line_order.shuffle(&mut rand);
				}

				for new_y in line_order {

					let new_work = RaytracingWork::line(0, new_y, width, *reverse_order_horizontal);

					if *transparent {
						let mut new_work = new_work.to_sub_work();
						work.append(&mut new_work);
					}
					else {
						work.push(new_work);
					}
				}
			}
			GenerationMode::Stripe { random_order_horizontal, reverse_order_vertical, transparent } => {
				let mut stripe_order: Vec<u32> = (0..width).collect();

				if *random_order_horizontal {
					stripe_order.shuffle(&mut rand);
				}

				for new_x in stripe_order {
					let new_work = RaytracingWork::stripe(new_x, 0, height, *reverse_order_vertical);
					if *transparent {
						let mut new_work = new_work.to_sub_work();
						work.append(&mut new_work);
					}
					else {
						work.push(new_work);
					}
				}
			}
			GenerationMode::Tile {
				tile_width,
				tile_height,
				transparent,
				dimension_order,
				sub_generation_mode
			} => {
				let mut horizontal_count = width / tile_width;
				let mut vertical_count = height / tile_height;

				let right_border = width % tile_width != 0;
				let bottom_border = height % tile_height != 0;

				let last_tile_width = tile_width + (width % tile_width);
				let last_tile_height = tile_height + (height % tile_height);

				if right_border {
					horizontal_count += 1;
				}
				if bottom_border {
					vertical_count += 1;
				}

				let mut coordinates: Vec<(u32,u32)> = Vec::with_capacity((vertical_count * horizontal_count) as usize);

				match dimension_order {
					TileDimensionOrder::LineFirst { horizontal_order, vertical_order } => {
						let vertical_order = Self::axis_order_range_to_values(0, vertical_count, *vertical_order);

						for y in vertical_order {
							let horizontal_order = Self::axis_order_range_to_values(0, horizontal_count,*horizontal_order);

							for x in horizontal_order {
								coordinates.push((x,y));
							}
						}
					}
					TileDimensionOrder::StripeFirst { horizontal_order, vertical_order } => {
						let horizontal_order = Self::axis_order_range_to_values(0, horizontal_count, *horizontal_order);

						for x in horizontal_order {
							let vertical_order = Self::axis_order_range_to_values(0, vertical_count, *vertical_order);

							for y in vertical_order {
								coordinates.push((x,y));
							}
						}
					}
					TileDimensionOrder::Random => {
						for y in 0..vertical_count {
							for x in 0..horizontal_count {
								coordinates.push((x,y));
							}
						}
						coordinates.shuffle(&mut rand);
					}
				};

				for (new_x, new_y) in coordinates {

					let width = if right_border && new_x == (horizontal_count - 1) {
						last_tile_width
					}
					else {
						*tile_width
					};

					let height = if bottom_border && new_y == (vertical_count - 1) {
						last_tile_height
					}
					else {
						*tile_height
					};

					let new_work = sub_generation_mode.to_work(width, height);
					let new_work = RaytracingWork::tile(new_x * tile_width, new_y * tile_height, new_work);

					if *transparent {
						let mut new_work = new_work.to_sub_work();
						work.append(&mut new_work);
					}
					else {

						work.push(new_work);
					}
				}
			}
		}

		work
	}

	pub fn from_gui_mode_tree(tree: &GUIModeTree) -> Self {
		match &tree.settings {
			GUIModeSettings::PixelRandom => {
				Self::PixelRandom
			}
			GUIModeSettings::Line { reverse_order_horizontal, random_order_vertical, transparent } => {
				Self::Line {
					reverse_order_horizontal: *reverse_order_horizontal,
					random_order_vertical: *random_order_vertical,
					transparent: *transparent
				}
			}
			GUIModeSettings::Stripe { random_order_horizontal, reverse_order_vertical, transparent } => {
				Self::Stripe {
					random_order_horizontal: *random_order_horizontal,
					reverse_order_vertical: *reverse_order_vertical,
					transparent: *transparent
				}
			}
			GUIModeSettings::Tile { tile_width, tile_height, dimension_order, horizontal_order, vertical_order, transparent } => {
				let dimension_order = Self::indices_to_tile_dimension_order(*horizontal_order, *vertical_order, *dimension_order);

				let sub_tree = match &tree.sub_tree {
					None => panic!(),
					Some(sub_tree) => sub_tree
				};

				let sub_generation_mode = Self::from_gui_mode_tree(sub_tree);

				Self::Tile {
					tile_width: *tile_width,
					tile_height: *tile_height,
					dimension_order,
					transparent: *transparent,
					sub_generation_mode: Box::new(sub_generation_mode)
				}
			}
		}
	}

	fn indices_to_tile_dimension_order(horizontal_order: usize, vertical_order: usize, dimension_order: usize) -> TileDimensionOrder {
		let horizontal_order = Self::index_to_tile_axis_order(horizontal_order);
		let vertical_order = Self::index_to_tile_axis_order(vertical_order);

		match dimension_order {
			0 => {
				TileDimensionOrder::LineFirst {
					horizontal_order,
					vertical_order
				}
			}
			1 => {
				TileDimensionOrder::StripeFirst {
					horizontal_order,
					vertical_order
				}
			}
			_ => {
				TileDimensionOrder::Random
			}
		}
	}

	fn index_to_tile_axis_order(index: usize) -> TileAxisOrder {
		match index {
			1 => TileAxisOrder::Reverse,
			2 => TileAxisOrder::Random,
			_ => TileAxisOrder::Forward
		}
	}

	fn axis_order_range_to_values(start: u32, end: u32, order: TileAxisOrder) -> Vec<u32> {
		let mut rand = thread_rng();

		match order {
			TileAxisOrder::Forward => {
				(start..end).collect()
			}
			TileAxisOrder::Reverse => {
				(start..end).rev().collect()
			}
			TileAxisOrder::Random => {
				let mut values: Vec<u32> = (start..end).collect();
				values.shuffle(&mut rand);
				values
			}
		}
	}
}

pub struct RaytracingWorkGenerator {
	pub width: u32,
	pub height: u32,
	pub generation_mode: GenerationMode
}

impl RaytracingWorkGenerator {
	pub fn generate(&self, pool: &mut WorkersPool<RaytracingWorker>) -> Result<(),()> {
		let work = self.generation_mode.to_work(self.width, self.height);
		for work in work {
			pool.add_work(work)?;
		}
		Ok(())
	}
}