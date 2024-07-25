use std::str::FromStr;

use grid::Grid;

use common::parse_struct_error::ParseStructError;

struct InternalNumber {
	pub value: u32,
	pub is_part_number: bool,
}

struct Gear {
	location: usize,
	connections: u32,
	ratio: u32,
}

pub struct Engine {
	internal_numbers: Vec<InternalNumber>,
	symbol_locations: Vec<usize>,
	gears: Vec<Gear>,
	number_index_grid: Grid<Option<usize>>,
}

impl Engine {
	pub fn sum_of_part_numbers(&self) -> u32 {
		let mut sum = 0;
		for n in self.internal_numbers.iter().filter(|n| n.is_part_number) {
			sum += n.value;
		}
		sum
	}

	pub fn sum_of_gear_ratios(&self) -> u32 {
		let mut sum = 0;
		for n in self.gears.iter().filter(|n| n.connections == 2) {
			sum += n.ratio;
		}
		sum
	}

	fn identify_part_numbers(&mut self) {
		for idx in self.symbol_locations.iter() {
			let row = (idx / self.number_index_grid.cols()) as i32;
			let col = (idx % self.number_index_grid.cols()) as i32;

			for row_offset in -1..=1 {
				for col_offset in -1..=1 {
					match self
						.number_index_grid
						.get(row + row_offset, col + col_offset)
					{
						Some(Some(i)) => self.internal_numbers[*i].is_part_number = true,
						_ => (),
					}
				}
			}
		}
	}

	fn count_gear_connections(&mut self) {
		for gear in self.gears.iter_mut() {
			let row = (gear.location / self.number_index_grid.cols()) as i32;
			let col = (gear.location % self.number_index_grid.cols()) as i32;

			let mut last_number_index = self.internal_numbers.len();

			for row_offset in -1..=1 {
				for col_offset in -1..=1 {
					match self
						.number_index_grid
						.get(row + row_offset, col + col_offset)
					{
						Some(Some(i)) if *i != last_number_index => {
							last_number_index = *i;
							gear.connections += 1;
							gear.ratio *= self.internal_numbers[*i].value;
						}
						_ => (),
					}
				}
			}
		}
	}
}

impl FromStr for Engine {
	type Err = ParseStructError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut internal_numbers: Vec<InternalNumber> = vec![];
		let mut symbol_locations: Vec<usize> = vec![];
		let mut flat_grid: Vec<Option<usize>> = vec![];
		let mut gears: Vec<Gear> = vec![];
		let mut grid_width: usize = 0;

		for line in s.lines() {
			let mut number_head: usize = internal_numbers.len();
			grid_width = line.len();

			for c in line.chars() {
				flat_grid.push(match c {
					'.' => {
						number_head = internal_numbers.len();
						None
					}
					'0'..='9' => {
						let digit = c as u32 - '0' as u32;
						if internal_numbers.len() == number_head {
							internal_numbers.push(InternalNumber {
								value: digit,
								is_part_number: false,
							});
						} else {
							let num = internal_numbers.last_mut().unwrap();
							num.value *= 10;
							num.value += digit;
						}
						Some(number_head)
					}
					'*' => {
						number_head = internal_numbers.len();
						symbol_locations.push(flat_grid.len());
						gears.push(Gear {
							location: flat_grid.len(),
							connections: 0,
							ratio: 1,
						});
						None
					}
					_ => {
						number_head = internal_numbers.len();
						symbol_locations.push(flat_grid.len());
						None
					}
				});
			}
		}

		let mut engine = Engine {
			internal_numbers,
			symbol_locations,
			gears,
			number_index_grid: Grid::from_vec(flat_grid, grid_width),
		};
		engine.identify_part_numbers();
		engine.count_gear_connections();

		Ok(engine)
	}
}
