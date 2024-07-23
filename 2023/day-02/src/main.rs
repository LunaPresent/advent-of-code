use std::io::{self, BufRead, BufReader};

use cube_set::CubeSet;
use game::Game;
mod cube_set;
mod game;

fn main() -> io::Result<()> {
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

fn sum_of_valid_ids(cube_pool: CubeSet) -> io::Result<u32> {
	let mut sum = 0;
	for line in BufReader::new(io::stdin()).lines() {
		let game: Game = line?.parse().map_err(|e| io::Error::other(e))?;
		if game.test_possible(cube_pool) {
			sum += game.id;
		}
	}
	Ok(sum)
}

fn sum_of_minimum_set_powers() -> io::Result<u32> {
	let mut sum = 0;
	for line in BufReader::new(io::stdin()).lines() {
		let game: Game = line?.parse().map_err(|e| io::Error::other(e))?;
		sum += game.minimum_set().power();
	}
	Ok(sum)
}
