use std::str::FromStr;

use common::parse_struct_error::ParseStructError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
	Left,
	Right,
}

#[derive(Debug)]
pub struct Directions(Vec<Direction>);

impl Directions {
	pub fn iter(&self) -> impl Iterator<Item = &Direction> {
		self.0.iter().cycle()
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}
}

impl FromStr for Directions {
	type Err = ParseStructError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Directions(
			s.chars()
				.enumerate()
				.map(|(i, c)| match c {
					'L' => Ok(Direction::Left),
					'R' => Ok(Direction::Right),
					_ => Err(ParseStructError::new::<Self>(s)
						.at(i)
						.because("unexpected character")),
				})
				.collect::<Result<Vec<_>, _>>()?,
		))
	}
}
