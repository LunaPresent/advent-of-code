use std::str::FromStr;

use super::cube_set::CubeSet;

use common::parse_struct_error::ParseStructError;

#[derive(Default, Clone)]
pub struct Game {
	pub id: u32,
	pub rolls: Vec<CubeSet>,
}

impl FromStr for Game {
	type Err = ParseStructError;

	// definitely easier to solve with a regex, however, I'm trying to learn new things and this is more fun
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// cut off "Game "
		let mut subs: &str = s
			.get(5..)
			.ok_or(ParseStructError::new::<Game>(s).at(s.len()))?;

		let i: usize = subs.find(": ").ok_or(
			ParseStructError::new::<Game>(s)
				.at(s.len())
				.because("\": \" not found"),
		)?;
		let game_id = subs[..i] // this is fine cause I just used find to get i
			.parse::<u32>()
			.map_err(|e| ParseStructError::new::<Game>(s).at(5).because(e))?;
		subs = &subs[i + 2..];
		// the whole "Game <n>: " header is now stripped off

		let mut game_rolls = Vec::<CubeSet>::new();
		while let Some(i) = subs.find("; ") {
			game_rolls.push(subs[..i].parse::<CubeSet>().map_err(|e| {
				ParseStructError::new::<Game>(s)
					.at(s.len() - subs.len())
					.because(e)
			})?);
			subs = &subs[i + 2..];
		}
		game_rolls.push(subs.parse::<CubeSet>().map_err(|e| {
			ParseStructError::new::<Game>(s)
				.at(s.len() - subs.len())
				.because(e)
		})?);

		Ok(Game {
			id: game_id,
			rolls: game_rolls,
		})
	}
}

impl Game {
	pub fn test_possible(&self, cube_pool: CubeSet) -> bool {
		for roll in self.rolls.iter() {
			if roll.red > cube_pool.red
				|| roll.blue > cube_pool.blue
				|| roll.green > cube_pool.green
			{
				return false;
			}
		}
		true
	}

	pub fn minimum_set(&self) -> CubeSet {
		let mut cs = CubeSet {
			red: 0,
			green: 0,
			blue: 0,
		};

		for roll in self.rolls.iter() {
			if roll.red > cs.red {
				cs.red = roll.red;
			}
			if roll.green > cs.green {
				cs.green = roll.green;
			}
			if roll.blue > cs.blue {
				cs.blue = roll.blue;
			}
		}

		cs
	}
}
