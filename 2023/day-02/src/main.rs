use std::{
	error::Error,
	io::{self, BufRead, BufReader},
};

use cube_set::CubeSet;
use game::Game;
mod cube_set;
mod game;

fn main() -> Result<(), Box<dyn Error>> {
	let cube_pool = CubeSet {
		red: 12,
		green: 13,
		blue: 14,
	};

	let val = match std::env::args().find(|x| x == "part2") {
		None => sum_of_valid_ids(cube_pool),
		Some(..) => sum_of_minimum_set_powers(),
	}?;

	println!("{}", val);

	Ok(())
}

fn sum_of_valid_ids(cube_pool: CubeSet) -> Result<u32, Box<dyn Error>> {
	let mut sum = 0;
	for line in BufReader::new(io::stdin()).lines() {
		let game: Game = line?.parse()?;
		if game.test_possible(cube_pool) {
			sum += game.id;
		}
	}
	Ok(sum)
}

fn sum_of_minimum_set_powers() -> Result<u32, Box<dyn Error>> {
	let mut sum = 0;
	for line in BufReader::new(io::stdin()).lines() {
		let game: Game = line?.parse()?;
		sum += game.minimum_set().power();
	}
	Ok(sum)
}
