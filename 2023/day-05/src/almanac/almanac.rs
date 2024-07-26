use std::{
	fmt::Display,
	ops::{Add, Sub},
	str::FromStr,
};

use common::parse_struct_error::ParseStructError;

use super::category_map::CategoryMap;

pub struct Almanac<T> {
	category_maps: Vec<CategoryMap<T>>,
}

impl<T: Ord + Sub<Output = T> + Add<Output = T> + Copy> Almanac<T> {
	// pub fn lowest_mapped(&self) -> Option<T> {
	// 	self.seeds.iter().map(|v| self.map_full(*v)).min()
	// }

	pub fn map_full(&self, mut value: T) -> T {
		for category_map in self.category_maps.iter() {
			value = category_map.map(value);
		}
		value
	}
}

impl<T: FromStr<Err = impl Display + 'static> + Copy> FromStr for Almanac<T> {
	type Err = ParseStructError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let category_maps = s
			.split("\n\n")
			.map(|s| s.parse())
			.collect::<Result<_, _>>()
			.map_err(|e| ParseStructError::new::<Almanac<T>>(s).because(e))?;

		Ok(Almanac { category_maps })
	}
}
