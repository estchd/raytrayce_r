pub mod generator;

#[derive(Clone, Debug)]
pub struct PixelWork {
	x: u32,
	y: u32,
	taken: bool
}

#[derive(Clone, Debug)]
pub struct LineWork {
	x: u32,
	y: u32,
	width: u32,
	current_x: u32,
	reverse: bool
}

#[derive(Clone, Debug)]
pub struct StripeWork {
	x: u32,
	y: u32,
	height: u32,
	current_y: u32,
	reverse: bool
}

#[derive(Clone, Debug)]
pub struct TileWork {
	x: u32,
	y: u32,
	sub_generation_work: Option<Vec<Option<RaytracingWork>>>
}

#[derive(Clone, Debug)]
pub enum RaytracingWork {
	Pixel(PixelWork),
	Line(LineWork),
	Stripe(StripeWork),
	Tile(TileWork)
}

impl RaytracingWork {
	pub fn pixel(x: u32, y: u32) -> Self {
		Self::Pixel(
			PixelWork {
				x,
				y,
				taken: false
			}
		)
	}
	
	pub fn line(x: u32, y: u32, width: u32, reverse: bool) -> Self {
		let work = if reverse {
			LineWork {
				x,
				y,
				width,
				current_x: width,
				reverse
			}
		}
		else {
			LineWork {
				x,
				y,
				width,
				current_x: 1,
				reverse
			}
		};

		Self::Line(work)
	}

	pub fn stripe(x: u32, y: u32, height: u32, reverse: bool) -> Self {
		let work = if reverse {
			StripeWork {
				x,
				y,
				height,
				current_y: height,
				reverse
			}
		}
		else {
			StripeWork {
				x,
				y,
				height,
				current_y: 1,
				reverse
			}
		};

		Self::Stripe(work)
	}

	pub fn tile(x: u32, y: u32, sub_work: Vec<Self>) -> Self {
		let sub_work: Option<Vec<Option<Self>>> = Some(sub_work.into_iter()
			.map(|item| {
				Some(item)
			}).collect());

		let work = TileWork {
			x,
			y,
			sub_generation_work: sub_work
		};

		Self::Tile(work)
	}

	pub fn get_next_work_pixel(&mut self) -> Option<(u32,u32)> {
		match self {
			RaytracingWork::Pixel(work) => {
				if work.taken {
					return None;
				}
				work.taken = true;
				Some((work.x, work.y))
			}
			RaytracingWork::Line(work) => {
				let x = if work.reverse {
					if work.current_x == 0 {
						return None;
					}
					work.current_x = work.current_x - 1;
					work.current_x
				}
				else {
					if work.current_x > work.width {
						return None;
					}
					work.current_x = work.current_x + 1;
					work.current_x - 2
				};


				Some((x + work.x,work.y))
			}
			RaytracingWork::Stripe(work) => {
				let y = if work.reverse {
					if work.current_y == 0 {
						return None;
					}
					work.current_y = work.current_y - 1;
					work.current_y
				}
				else {
					if work.current_y > work.height {
						return None;
					}
					work.current_y = work.current_y + 1;
					work.current_y - 2
				};

				Some((work.x, y + work.y))
			}
			RaytracingWork::Tile(tile_work) => {
				let sub_generation_work = tile_work.sub_generation_work.as_mut()?;

				for sub_work in sub_generation_work.iter_mut() {
					let work = match sub_work {
						None => {
							continue;
						}
						Some(work) => work
					};

					let sub_work_next_pixel = work.get_next_work_pixel();

					match sub_work_next_pixel {
						None => {
							*sub_work = None;
						}
						Some((pixel_x, pixel_y)) => {
							return Some((tile_work.x + pixel_x, tile_work.y + pixel_y))
						}
					}
				}

				return None;
			}
		}
	}

	pub fn to_sub_work(self) -> Vec<RaytracingWork> {
		let mut work = vec![];
		match self {
			RaytracingWork::Pixel(pixel) => {
				work.push(RaytracingWork::Pixel(pixel));
			}
			RaytracingWork::Line(line) => {
				let mut reconstituted = RaytracingWork::Line(line);

				loop {
					let new_work = reconstituted.get_next_work_pixel();
					match new_work {
						None => break,
						Some((x,y)) => {
							work.push(RaytracingWork::pixel(x, y));
						}
					};
				}
			}
			RaytracingWork::Stripe(stripe) => {
				let mut reconstituted = RaytracingWork::Stripe(stripe);

				loop {
					let new_work = reconstituted.get_next_work_pixel();
					match new_work {
						None => break,
						Some((x,y)) => {
							work.push(RaytracingWork::pixel(x,y));
						}
					}
				}
			}
			RaytracingWork::Tile(tile) => {
				let new_work = tile.sub_generation_work;
				let new_work = match new_work {
					None => vec![],
					Some(work) => work,
				};
				let new_work: Vec<RaytracingWork> = new_work.into_iter().flatten().collect();

				for mut new_work in new_work {
					new_work.adjust_for_transparency(tile.x, tile.y);
					work.push(new_work);
				}
			}
		}

		work
	}

	fn adjust_for_transparency(&mut self, x: u32, y: u32) {
		match self {
			RaytracingWork::Pixel(pixel) => {
				pixel.x = pixel.x + x;
				pixel.y = pixel.y + y;
			}
			RaytracingWork::Line(line) => {
				line.x = line.x + x;
				line.y = line.y + y;
			}
			RaytracingWork::Stripe(stripe) => {
				stripe.x = stripe.x + x;
				stripe.y = stripe.y + y;
			}
			RaytracingWork::Tile(tile) => {
				tile.x = tile.x + x;
				tile.y = tile.y + y;
			}
		}
	}
}
