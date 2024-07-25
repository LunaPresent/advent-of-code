use std::str::FromStr;

use common::parse_struct_error::ParseStructError;

pub struct Scratchcard {
	winning_numbers: Vec<u32>,
	drawn_numbers: Vec<u32>,
}

impl Scratchcard {
	pub fn win_count(&self) -> u32 {
		let mut count = 0;
		let mut win_iter = self.winning_numbers.iter().peekable();
		let mut draw_iter = self.drawn_numbers.iter().peekable();

		while let (Some(&&win), Some(&&draw)) = (win_iter.peek(), draw_iter.peek()) {
			if win <= draw {
				win_iter.next();
			}
			if win >= draw {
				draw_iter.next();
			}
			if win == draw {
				count += 1;
			}
		}

		count
	}
}

impl FromStr for Scratchcard {
	type Err = ParseStructError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let subs = s
			.find(':')
			.ok_or(ParseStructError::new::<Scratchcard>(s).at(s.len()))
			.and_then(|i| {
				s.get(i + 1..)
					.ok_or(ParseStructError::new::<Scratchcard>(s).at(s.len()))
			})?;

		let (left, right) = subs
			.split_once('|')
			.ok_or(ParseStructError::new::<Scratchcard>(s).at(s.len()))?;

		fn parse_num_vec(subs: &str, s: &str) -> Result<Vec<u32>, <Scratchcard as FromStr>::Err> {
			let spl = subs.split(' ').filter(|subs| !subs.is_empty());
			let mut v: Vec<u32> = vec![];
			for subs in spl {
				v.push(
					subs.parse()
						.map_err(|e| ParseStructError::new::<Scratchcard>(s).because(e))?,
				);
			}
			v.sort();
			Ok(v)
		}

		let winning_numbers = parse_num_vec(left, s)?;
		let drawn_numbers = parse_num_vec(right, s)?;

		Ok(Scratchcard {
			winning_numbers,
			drawn_numbers,
		})
	}
}
