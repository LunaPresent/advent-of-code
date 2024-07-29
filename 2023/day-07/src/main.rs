use std::{
	env,
	error::Error,
	io::{self, BufRead, BufReader},
};

use camel_cards::Round;

mod camel_cards;

fn main() -> Result<(), Box<dyn Error>> {
	let lines = BufReader::new(io::stdin()).lines();
	let mapped = match env::args().find(|x| x == "part2") {
		None => Box::new(lines) as Box<dyn Iterator<Item = Result<String, _>>>,
		Some(..) => Box::new(lines.map(|l| match l {
			Ok(s) => Ok(s.replace('J', "*")),
			Err(e) => Err(e),
		})) as Box<dyn Iterator<Item = Result<String, _>>>,
	};

	let mut rounds = mapped
		.map(|l| Ok::<_, Box<dyn Error>>(l?.parse::<Round>()?))
		.collect::<Result<Vec<_>, _>>()?;
	rounds.sort_unstable();

	let val: u64 = rounds
		.iter()
		.enumerate()
		.map(|(i, e)| (i as u64 + 1) * e.bid())
		.sum();

	println!("{}", val);

	Ok(())
}
