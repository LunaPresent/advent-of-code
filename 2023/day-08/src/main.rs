mod haunted_wasteland;

use std::{
	env,
	error::Error,
	io::{self, BufRead, BufReader},
};

use haunted_wasteland::{Directions, MapBuilder};

fn main() -> Result<(), Box<dyn Error>> {
	let mut lines = BufReader::new(io::stdin()).lines();
	let directions = lines
		.next()
		.ok_or(io::Error::new(io::ErrorKind::InvalidInput, "no input"))??
		.parse::<Directions>()?;
	let map = lines
		.skip(1)
		.fold(
			Ok(MapBuilder::new()),
			|mb: Result<MapBuilder, Box<dyn Error>>, l| Ok(mb?.add_node(l?.parse()?)),
		)?
		.build()?;

	let val = match env::args().find(|x| x == "part2") {
		None => map.traverse(&directions, |s| s == "AAA", |s| s == "ZZZ"),
		Some(..) => map.traverse(&directions, |s| s.ends_with('A'), |s| s.ends_with('Z')),
	};
	println!("{}", val);

	Ok(())
}
