mod boat_race;

use std::{
	env,
	error::Error,
	io::{self, Read},
	num::ParseIntError,
};

use boat_race::BoatRace;

fn main() -> Result<(), Box<dyn Error>> {
	let mut buf = Vec::<u8>::new();
	io::stdin().read_to_end(&mut buf)?;
	let s = String::from_utf8(buf)?;

	let parse_nums_func = match env::args().find(|x| x == "part2") {
		None => parse_nums_part1,
		Some(..) => parse_nums_part2,
	};

	let mut parsed_lines = s.lines().map(|l| {
		Ok::<Vec<u64>, Box<dyn Error>>(parse_nums_func(
			l.get(
				l.find(':')
					.ok_or(io::Error::new(io::ErrorKind::InvalidInput, "':' not found"))?
					+ 1..,
			)
			.unwrap(),
		)?)
	});
	let (times, records) = (
		parsed_lines.next().ok_or(io::Error::new(
			io::ErrorKind::InvalidInput,
			"not enough lines",
		))??,
		parsed_lines.next().ok_or(io::Error::new(
			io::ErrorKind::InvalidInput,
			"not enough lines",
		))??,
	);
	let boat_races = times
		.iter()
		.zip(records.iter())
		.map(|(&time, &record)| BoatRace { time, record });

	let val: u64 = boat_races
		.map(|boat_race| boat_race.time_win_range())
		.product();
	println!("{}", val);

	Ok(())
}

fn parse_nums_part1(s: &str) -> Result<Vec<u64>, ParseIntError> {
	s.split(' ')
		.map(|s| s.trim())
		.filter(|s| !s.is_empty())
		.map(|s| s.parse::<u64>())
		.collect::<Result<Vec<u64>, ParseIntError>>()
}

fn parse_nums_part2(s: &str) -> Result<Vec<u64>, ParseIntError> {
	let s_keming = s.replace(' ', "");
	Ok(vec![s_keming.parse::<u64>()?])
}
