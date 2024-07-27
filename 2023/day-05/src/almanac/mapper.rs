use std::{iter::once, str::FromStr};

use common::parse_struct_error::ParseStructError;

use super::range_mapping::RangeMapping;

#[derive(Debug)]
pub struct Mapper {
	pub(super) mappings: Vec<RangeMapping>,
}

impl Mapper {
	pub fn map(&self, value: u64) -> u64 {
		match self.mappings.iter().filter_map(|v| v.try_map(value)).next() {
			Some(v) => v,
			None => value,
		}
	}

	pub fn map_range<'a>(&'a self, range: RangeMapping) -> impl Iterator<Item = RangeMapping> + 'a {
		self.mappings
			.iter()
			.scan(0_u64, |state, e| {
				let pair = Some((
					RangeMapping {
						source: *state,
						destination: *state,
						range: e.source - *state,
					},
					*e,
				));
				*state = e.source + e.range;
				pair
			})
			.flat_map(|(first, next)| once(first).chain(once(next)))
			.chain(once({
				let start = match self.mappings.last() {
					Some(mapping) => mapping.source + mapping.range,
					None => 0_u64,
				};
				RangeMapping {
					source: start,
					destination: start,
					range: u64::MAX - start,
				}
			}))
			.filter_map(move |r| range.intersect(&r))
	}

	pub fn combine(&self, next: &Self) -> Self {
		let mut mappings: Vec<RangeMapping> = vec![];
		let mut start_blank = 0;
		for first in self.mappings.iter() {
			let blank_mapping = RangeMapping {
				source: start_blank,
				destination: start_blank,
				range: first.source - start_blank,
			};
			// combine blank before first with non-blanks of next
			for next in next.mappings.iter() {
				let isec = blank_mapping.intersect(&next);
				match isec {
					Some(isec) => mappings.push(isec),
					None => (),
				};
			}

			// all where first is non-blank
			let iter = next.map_range(*first);
			mappings.extend(iter);

			start_blank = first.source + first.range;
		}
		let last_blank_mapping = RangeMapping {
			source: start_blank,
			destination: start_blank,
			range: u64::MAX - start_blank,
		};
		// combine trailing blank of first with non-blanks of next
		for next in next.mappings.iter() {
			let isec = last_blank_mapping.intersect(&next);
			match isec {
				Some(isec) => mappings.push(isec),
				None => (),
			};
		}

		Mapper { mappings }
	}
}

impl FromStr for Mapper {
	type Err = ParseStructError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut mappings: Vec<RangeMapping> = vec![];

		for line in s.lines().skip(1) {
			mappings.push(line.parse()?);
		}
		mappings.sort_unstable_by_key(|e| e.source);

		Ok(Mapper { mappings })
	}
}
