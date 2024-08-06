mod sequence;

use std::{
	env,
	error::Error,
	io::{self, BufRead, BufReader},
};

use sequence::Sequence;

fn main() -> Result<(), Box<dyn Error>> {
	let map_seq = match env::args().find(|x| x == "part2") {
		None => |seq: &Sequence| seq.nth(seq.len() as i128),
		Some(..) => |seq: &Sequence| seq.nth(-1),
	};

	let val = BufReader::new(io::stdin())
		.lines()
		.map(|s| Ok::<_, Box<dyn Error>>(s?.parse::<Sequence>()?))
		.map(|seq| {
			let seq = seq?;
			Ok::<_, Box<dyn Error>>(map_seq(&seq))
		})
		.reduce(|a, b| Ok(a? + b?))
		.unwrap_or(Ok(0))?;

	println!("{}", val);

	Ok(())
}
