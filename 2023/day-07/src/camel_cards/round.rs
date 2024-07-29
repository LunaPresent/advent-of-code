use std::str::FromStr;

use common::parse_struct_error::ParseStructError;

use super::hand::Hand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Round {
	hand: Hand,
	bid: u64,
}

impl Round {
	pub fn bid(&self) -> u64 {
		self.bid
	}
}

impl PartialOrd for Round {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Round {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.hand.cmp(&other.hand).then(self.bid.cmp(&other.bid))
	}
}

impl FromStr for Round {
	type Err = ParseStructError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (hand_str, bid_str) = s.split_once(' ').ok_or(ParseStructError::new::<Self>(s))?;
		let hand = hand_str.parse()?;
		let bid = bid_str
			.parse()
			.map_err(|e| ParseStructError::new::<Self>(s).because(e))?;

		Ok(Round { hand, bid })
	}
}
