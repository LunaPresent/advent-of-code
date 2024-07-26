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

	let (seedstr, categorystr) = s
		.split_once("\n\n")
		.ok_or(io::Error::new(io::ErrorKind::InvalidInput, "split failed"))?;

	//let seeds = parse_seeds(seedstr)?;
	let almanac: Almanac<u64> = categorystr.parse()?;
	let val = match env::args().find(|x| x == "part2") {
		None => {
			let seeds = part1(seedstr);
			seeds
				.map(|v| almanac.map_full(v))
				.min()
				.ok_or(io::Error::new(io::ErrorKind::InvalidInput, "no values"))?
		}

		Some(..) => {
			let seeds = part2(seedstr);
			seeds
				.map(|v| almanac.map_full(v))
				.min()
				.ok_or(io::Error::new(io::ErrorKind::InvalidInput, "no values"))?
		}
	};

	println!("{}", val);

	Ok(())
}

fn part1<'a>(s: &'a str) -> impl Iterator<Item = u64> + 'a {
	let seeds_prompt_len = "seeds: ".len();
	let seedstr = &s[seeds_prompt_len..];
	seedstr
		.split(' ')
		.map(|s| s.trim())
		.filter(|s| !s.is_empty())
		.map(|s| s.parse::<u64>().unwrap())
}

fn part2<'a>(s: &'a str) -> impl Iterator<Item = u64> + 'a {
	let seeds_prompt_len = "seeds: ".len();
	let seedstr = &s[seeds_prompt_len..];
	seedstr
		.split(' ')
		.map(|s| s.trim())
		.filter(|s| !s.is_empty())
		.map(|s| s.parse::<u64>().unwrap())
}
