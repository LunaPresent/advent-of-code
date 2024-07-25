use std::{
	env,
	error::Error,
	io::{self, Read},
};

use engine::Engine;

mod engine;

fn main() -> Result<(), Box<dyn Error>> {
	let mut buf = Vec::<u8>::new();
	io::stdin().read_to_end(&mut buf)?;
	let s = String::from_utf8(buf)?;
	let engine: Engine = s.parse()?;

	let val = match env::args().find(|x| x == "part2") {
		None => engine.sum_of_part_numbers(),
		Some(..) => engine.sum_of_gear_ratios(),
	};
	println!("{}", val);

	Ok(())
}
