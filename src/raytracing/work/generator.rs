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

#[derive(Debug, Clone)]
pub enum GenerationMode {
	PixelRandom,
	Line {
		reverse_order_horizontal: bool,
		random_order_vertical: bool
	},
	PixelLine {
		reverse_order_horizontal: bool,
		random_order_vertical: bool
	},
	Stripe {
		random_order_horizontal: bool,
		reverse_order_vertical: bool
	},
	PixelStripe {
		random_order_horizontal: bool,
		reverse_order_vertical: bool
	},
	LineFirstTile {
		tile_width: u32,
		tile_height: u32,
		horizontal_order: TileAxisOrder,
		vertical_order: TileAxisOrder,
		sub_generation_mode: Box<GenerationMode>
	},
	StripeFirstTile {
		tile_width: u32,
		tile_height: u32,
		horizontal_order: TileAxisOrder,
		vertical_order: TileAxisOrder,
		sub_generation_mode: Box<GenerationMode>
	},
	LineFirstPixelTile {
		tile_width: u32,
		tile_height: u32,
		horizontal_order: TileAxisOrder,
		vertical_order: TileAxisOrder,
		sub_generation_mode: Box<GenerationMode>
	},
	StripeFirstPixelTile {
		tile_width: u32,
		tile_height: u32,
		horizontal_order: TileAxisOrder,
		vertical_order: TileAxisOrder,
		sub_generation_mode: Box<GenerationMode>
	}
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

				for (x,y) in coords {
					let new_work = RaytracingWork::pixel(x,y);
					work.push(new_work);
				}
			}
			GenerationMode::Line { reverse_order_horizontal, random_order_vertical } => {
				let mut line_order: Vec<u32> = (0..height).collect();

				if *random_order_vertical {
					line_order.shuffle(&mut rand);
				}

				for y in line_order {
					let new_work = RaytracingWork::line(y, width, *reverse_order_horizontal);
					work.push(new_work);
				}
			}
			GenerationMode::PixelLine { reverse_order_horizontal, random_order_vertical } => {
				let mut line_order: Vec<u32> = (0..height).collect();

				if *random_order_vertical {
					line_order.shuffle(&mut rand);
				}

				for y in line_order {
					let pixel_order = 0..width;
					let pixel_order: Vec<u32> = if *reverse_order_horizontal {
						pixel_order.rev().collect()
					}
					else {
						pixel_order.collect()
					};

					for x in pixel_order {
						let new_work = RaytracingWork::pixel(x,y);
						work.push(new_work);
					}
				}
			}
			GenerationMode::Stripe { random_order_horizontal, reverse_order_vertical } => {
				let mut stripe_order: Vec<u32> = (0..width).collect();

				if *random_order_horizontal {
					stripe_order.shuffle(&mut rand);
				}

				for x in stripe_order {
					let new_work = RaytracingWork::stripe(x, height, *reverse_order_vertical);
					work.push(new_work);
				}
			}
			GenerationMode::PixelStripe { random_order_horizontal, reverse_order_vertical } => {
				let mut stripe_order: Vec<u32> = (0..width).collect();

				if *random_order_horizontal {
					stripe_order.shuffle(&mut rand);
				}

				for x in stripe_order {
					let pixel_order = 0..height;
					let pixel_order: Vec<u32> = if *reverse_order_vertical {
						pixel_order.rev().collect()
					} else {
						pixel_order.collect()
					};

					for y in pixel_order {
						let new_work = RaytracingWork::pixel(x,y);
						work.push(new_work);
					}
				}
			}
			GenerationMode::LineFirstTile { tile_width, tile_height, horizontal_order, vertical_order, sub_generation_mode } => {
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

				let vertical_range = 0..vertical_count;

				let vertical_order: Vec<u32> = match vertical_order {
					TileAxisOrder::Forward => {
						vertical_range.collect()
					}
					TileAxisOrder::Reverse => {
						vertical_range.rev().collect()
					}
					TileAxisOrder::Random => {
						let mut order: Vec<u32> = vertical_range.collect();
						order.shuffle(&mut rand);
						order
					}
				};

				for y in vertical_order {
					let horizontal_range = 0..horizontal_count;
					let horizontal_order: Vec<u32> = match horizontal_order {
						TileAxisOrder::Forward => {
							horizontal_range.collect()
						}
						TileAxisOrder::Reverse => {
							horizontal_range.rev().collect()
						}
						TileAxisOrder::Random => {
							let mut order: Vec<u32> = horizontal_range.collect();
							order.shuffle(&mut rand);
							order
						}
					};

					for x in horizontal_order {
						let width = if right_border && x == (horizontal_count - 1) {
							last_tile_width
						}
						else {
							*tile_width
						};

						let height = if bottom_border && y == (vertical_count - 1) {
							last_tile_height
						}
						else {
							*tile_height
						};

						let new_work = sub_generation_mode.to_work(width, height);

						let new_work = RaytracingWork::tile(x * tile_width, y * tile_height, new_work);

						work.push(new_work);
					}
				}
			}
			GenerationMode::StripeFirstTile { tile_width, tile_height, horizontal_order, vertical_order, sub_generation_mode } => {
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

				let horizontal_range = 0..horizontal_count;

				let horizontal_order: Vec<u32> = match horizontal_order {
					TileAxisOrder::Forward => {
						horizontal_range.collect()
					}
					TileAxisOrder::Reverse => {
						horizontal_range.rev().collect()
					}
					TileAxisOrder::Random => {
						let mut order: Vec<u32> = horizontal_range.collect();
						order.shuffle(&mut rand);
						order
					}
				};



				for x in horizontal_order {
					let vertical_range = 0..vertical_count;
					let vertical_order: Vec<u32> = match vertical_order {
						TileAxisOrder::Forward => {
							vertical_range.collect()
						}
						TileAxisOrder::Reverse => {
							vertical_range.rev().collect()
						}
						TileAxisOrder::Random => {
							let mut order: Vec<u32> = vertical_range.collect();
							order.shuffle(&mut rand);
							order
						}
					};

					for y in vertical_order {
						let width = if right_border && x == (horizontal_count - 1) {
							last_tile_width
						}
						else {
							*tile_width
						};

						let height = if bottom_border && y == (vertical_count - 1) {
							last_tile_height
						}
						else {
							*tile_height
						};

						let new_work = sub_generation_mode.to_work(width, height);

						let new_work = RaytracingWork::tile(x * tile_width, y * tile_height, new_work);

						work.push(new_work);
					}
				}
			}
			GenerationMode::LineFirstPixelTile { tile_width, tile_height, horizontal_order, vertical_order, sub_generation_mode } => {
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

				let vertical_range = 0..vertical_count;



				let vertical_order: Vec<u32> = match vertical_order {
					TileAxisOrder::Forward => {
						vertical_range.collect()
					}
					TileAxisOrder::Reverse => {
						vertical_range.rev().collect()
					}
					TileAxisOrder::Random => {
						let mut order: Vec<u32> = vertical_range.collect();
						order.shuffle(&mut rand);
						order
					}
				};

				for y in vertical_order {
					let horizontal_range = 0..horizontal_count;
					let horizontal_order: Vec<u32> = match horizontal_order {
						TileAxisOrder::Forward => {
							horizontal_range.collect()
						}
						TileAxisOrder::Reverse => {
							horizontal_range.rev().collect()
						}
						TileAxisOrder::Random => {
							let mut order: Vec<u32> = horizontal_range.collect();
							order.shuffle(&mut rand);
							order
						}
					};

					for x in horizontal_order {
						let width = if right_border && x == (horizontal_count - 1) {
							last_tile_width
						}
						else {
							*tile_width
						};

						let height = if bottom_border && y == (vertical_count - 1) {
							last_tile_height
						}
						else {
							*tile_height
						};

						let mut new_work = sub_generation_mode.to_work(width, height);

						work.append(&mut new_work);
					}
				}
			}
			GenerationMode::StripeFirstPixelTile { tile_width, tile_height, horizontal_order, vertical_order, sub_generation_mode } => {
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

				let horizontal_range = 0..horizontal_count;
				let horizontal_order: Vec<u32> = match horizontal_order {
					TileAxisOrder::Forward => {
						horizontal_range.collect()
					}
					TileAxisOrder::Reverse => {
						horizontal_range.rev().collect()
					}
					TileAxisOrder::Random => {
						let mut order: Vec<u32> = horizontal_range.collect();
						order.shuffle(&mut rand);
						order
					}
				};

				for x in horizontal_order {
					let vertical_range = 0..vertical_count;
					let vertical_order: Vec<u32> = match vertical_order {
						TileAxisOrder::Forward => {
							vertical_range.collect()
						}
						TileAxisOrder::Reverse => {
							vertical_range.rev().collect()
						}
						TileAxisOrder::Random => {
							let mut order: Vec<u32> = vertical_range.collect();
							order.shuffle(&mut rand);
							order
						}
					};
					for y in vertical_order {
						let width = if right_border && x == (horizontal_count - 1) {
							last_tile_width
						}
						else {
							*tile_width
						};

						let height = if bottom_border && y == (vertical_count - 1) {
							last_tile_height
						}
						else {
							*tile_height
						};

						let mut new_work = sub_generation_mode.to_work(width, height);

						work.append(&mut new_work);
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
			GUIModeSettings::Line { reverse_order_horizontal, random_order_vertical } => {
				Self::Line {
					reverse_order_horizontal: *reverse_order_horizontal,
					random_order_vertical: *random_order_vertical
				}
			}
			GUIModeSettings::PixelLine { reverse_order_horizontal, random_order_vertical } => {
				Self::PixelLine {
					reverse_order_horizontal: *reverse_order_horizontal,
					random_order_vertical: *random_order_vertical
				}
			}
			GUIModeSettings::Stripe { random_order_horizontal, reverse_order_vertical } => {
				Self::Stripe {
					random_order_horizontal: *random_order_horizontal,
					reverse_order_vertical: *reverse_order_vertical
				}
			}
			GUIModeSettings::PixelStripe { random_order_horizontal, reverse_order_vertical } => {
				Self::PixelStripe {
					random_order_horizontal: *random_order_horizontal,
					reverse_order_vertical: *reverse_order_vertical
				}
			}
			GUIModeSettings::LineFirstTile { tile_width, tile_height, horizontal_order, vertical_order } => {
				let horizontal_order = Self::index_to_tile_axis_order(*horizontal_order);
				let vertical_order = Self::index_to_tile_axis_order(*vertical_order);

				let sub_tree = match &tree.sub_tree {
					None => panic!(),
					Some(sub_tree) => sub_tree
				};

				let sub_generation_mode = Self::from_gui_mode_tree(sub_tree);

				Self::LineFirstTile {
					tile_width: *tile_width,
					tile_height: *tile_height,
					horizontal_order,
					vertical_order,
					sub_generation_mode: Box::new(sub_generation_mode)
				}
			}
			GUIModeSettings::StripeFirstTile { tile_width, tile_height, horizontal_order, vertical_order } => {
				let horizontal_order = Self::index_to_tile_axis_order(*horizontal_order);
				let vertical_order = Self::index_to_tile_axis_order(*vertical_order);

				let sub_tree = match &tree.sub_tree {
					None => panic!(),
					Some(sub_tree) => sub_tree
				};

				let sub_generation_mode = Self::from_gui_mode_tree(sub_tree);

				Self::StripeFirstTile {
					tile_width: *tile_width,
					tile_height: *tile_height,
					horizontal_order,
					vertical_order,
					sub_generation_mode: Box::new(sub_generation_mode)
				}
			}
			GUIModeSettings::LineFirstPixelTile { tile_width, tile_height, horizontal_order, vertical_order } => {
				let horizontal_order = Self::index_to_tile_axis_order(*horizontal_order);
				let vertical_order = Self::index_to_tile_axis_order(*vertical_order);

				let sub_tree = match &tree.sub_tree {
					None => panic!(),
					Some(sub_tree) => sub_tree
				};

				let sub_generation_mode = Self::from_gui_mode_tree(sub_tree);

				Self::LineFirstPixelTile {
					tile_width: *tile_width,
					tile_height: *tile_height,
					horizontal_order,
					vertical_order,
					sub_generation_mode: Box::new(sub_generation_mode)
				}
			}
			GUIModeSettings::StripeFirstPixelTile { tile_width, tile_height, horizontal_order, vertical_order } => {
				let horizontal_order = Self::index_to_tile_axis_order(*horizontal_order);
				let vertical_order = Self::index_to_tile_axis_order(*vertical_order);

				let sub_tree = match &tree.sub_tree {
					None => panic!(),
					Some(sub_tree) => sub_tree
				};

				let sub_generation_mode = Self::from_gui_mode_tree(sub_tree);

				Self::StripeFirstPixelTile {
					tile_width: *tile_width,
					tile_height: *tile_height,
					horizontal_order,
					vertical_order,
					sub_generation_mode: Box::new(sub_generation_mode)
				}
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