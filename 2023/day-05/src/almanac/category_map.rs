use std::{
	fmt::Display,
	ops::{Add, Sub},
	str::FromStr,
};

use common::parse_struct_error::ParseStructError;

use super::range_mapping::RangeMapping;

pub struct CategoryMap<T> {
	mappings: Vec<RangeMapping<T>>,
}

impl<T: Ord + Sub<Output = T> + Add<Output = T> + Copy> CategoryMap<T> {
	pub fn map(&self, value: T) -> T {
		match self.mappings.iter().filter_map(|v| v.try_map(value)).next() {
			Some(v) => v,
			None => value,
		}
	}
}

impl<T: FromStr<Err = impl Display + 'static> + Copy> FromStr for CategoryMap<T> {
	type Err = ParseStructError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut mappings = vec![];

		for line in s.lines().skip(1) {
			mappings.push(line.parse()?);
		}

		Ok(CategoryMap { mappings })
	}
}
