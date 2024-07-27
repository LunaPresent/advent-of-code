mod almanac;

use std::{
	env,
	error::Error,
	io::{self, Read},
};

use almanac::Almanac;

fn main() -> Result<(), Box<dyn Error>> {
	let mut buf = Vec::<u8>::new();
	io::stdin().read_to_end(&mut buf)?;
	let s = String::from_utf8(buf)?;

	let almanac: Almanac = s.parse()?;

	let val = match env::args().find(|x| x == "part2") {
		None => almanac.lowest_mapped(),
		Some(..) => almanac.lowest_mapped_seed_range(),
	}
	.ok_or(io::Error::new(io::ErrorKind::InvalidInput, "no values"))?;
	println!("{}", val);

	Ok(())
}
