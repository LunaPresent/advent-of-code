use std::{
	fmt::{Debug, Display},
	ops::{Add, Sub},
	str::FromStr,
};

use common::parse_struct_error::ParseStructError;

#[derive(Debug, Clone, Copy)]
pub struct RangeMapping<T> {
	source: T,
	destination: T,
	range: T,
}

impl<T: Ord + Sub<Output = T> + Add<Output = T> + Copy> RangeMapping<T> {
	pub fn try_map(&self, value: T) -> Option<T> {
		if value >= self.source && value < self.source + self.range {
			return Some(value - self.source + self.destination);
		}
		None
	}
}

impl<T: FromStr<Err = impl Display + 'static> + Copy> FromStr for RangeMapping<T> {
	type Err = ParseStructError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let numstrs = s.split(' ');
		let (destination, source, range) = match &numstrs
			.map(|ns| ns.parse::<T>())
			.collect::<Result<Vec<_>, _>>()
			.map_err(|e| ParseStructError::new::<RangeMapping<T>>(s).because(e))?[..]
		{
			&[destination, source, range] => Ok((destination, source, range)),
			_ => Err(ParseStructError::new::<RangeMapping<T>>(s)),
		}?;

		Ok(RangeMapping {
			source,
			destination,
			range,
		})
	}
}
