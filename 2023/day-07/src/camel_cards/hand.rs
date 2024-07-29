use std::{cmp::Reverse, str::FromStr};

use common::parse_struct_error::ParseStructError;

use super::{card::Card, hand_type::HandType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hand([Card; 5]);

impl Hand {
	pub fn hand_type(&self) -> HandType {
		let mut counts = [0; 5];
		self.0.iter().filter(|c| !c.is_wildcard()).for_each(|a| {
			counts[self
				.0
				.iter()
				.enumerate()
				.find(|(_i, b)| *a == **b)
				.unwrap()
				.0] += 1;
		});
		counts.sort_unstable_by_key(|c| Reverse(*c));
		counts[0] += self.0.iter().filter(|c| c.is_wildcard()).count();

		match counts {
			[5, ..] => HandType::FiveOfAKind,
			[4, ..] => HandType::FourOfAKind,
			[3, 2, ..] => HandType::FullHouse,
			[3, 1, ..] => HandType::ThreeOfAKind,
			[2, 2, ..] => HandType::TwoPair,
			[2, 1, ..] => HandType::OnePair,
			_ => HandType::HighCard,
		}
	}
}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Hand {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.hand_type()
			.cmp(&other.hand_type())
			.then(self.0.cmp(&other.0))
	}
}

impl FromStr for Hand {
	type Err = ParseStructError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Hand(
			s.chars()
				.map(|c| Card::try_from(c))
				.collect::<Result<Vec<_>, _>>()?
				.try_into()
				.map_err(|_| ParseStructError::new::<Self>(s))?,
		))
	}
}
