use std::str::FromStr;

use common::parse_struct_error::ParseStructError;

#[derive(Debug, Clone, Copy)]
pub struct RangeMapping {
	pub(super) source: u64,
	pub(super) destination: u64,
	pub(super) range: u64,
}

impl RangeMapping {
	pub fn try_map(&self, value: u64) -> Option<u64> {
		if value >= self.source && value < self.source + self.range {
			return Some(value - self.source + self.destination);
		}
		None
	}

	pub fn intersect(&self, next: &Self) -> Option<RangeMapping> {
		let start = u64::max(self.destination, next.source);
		let end = u64::min(self.destination + self.range, next.source + next.range);
		if end <= start {
			return None;
		}
		Some(RangeMapping {
			destination: start - next.source + next.destination,
			source: start - self.destination + self.source,
			range: end - start,
		})
	}
}

impl FromStr for RangeMapping {
	type Err = ParseStructError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let numstrs = s.split(' ');
		let (destination, source, range) = match &numstrs
			.map(|ns| ns.parse::<u64>())
			.collect::<Result<Vec<_>, _>>()
			.map_err(|e| ParseStructError::new::<Self>(s).because(e))?[..]
		{
			&[destination, source, range] => Ok((destination, source, range)),
			_ => Err(ParseStructError::new::<Self>(s)),
		}?;

		Ok(RangeMapping {
			source,
			destination,
			range,
		})
	}
}
