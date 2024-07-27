use std::str::FromStr;

use common::parse_struct_error::ParseStructError;

use super::mapper::Mapper;
use super::mapper_sequence::MapperSequence;
use super::range_mapping::RangeMapping;

#[derive(Debug)]
pub struct Almanac {
	seeds: Vec<u64>,
	mapper: Mapper,
}

impl Almanac {
	pub fn lowest_mapped(&self) -> Option<u64> {
		self.seeds.iter().map(|v| self.mapper.map(*v)).min()
	}

	pub fn lowest_mapped_seed_range(&self) -> Option<u64> {
		let seed_ranges = self
			.seeds
			.iter()
			.step_by(2)
			.zip(self.seeds.iter().skip(1).step_by(2))
			.map(|(&start, &range)| RangeMapping {
				source: start,
				destination: start,
				range,
			});

		seed_ranges
			.map(|r| self.mapper.map_range(r).map(|r| r.destination).min())
			.filter_map(|d| d)
			.min()
	}
}

impl FromStr for Almanac {
	type Err = ParseStructError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let seeds_prompt_len = "seeds: ".len();

		let (mut seed_str, mapper_sequence_str) = s
			.split_once("\n\n")
			.ok_or(ParseStructError::new::<Self>(s).because("invalid format"))?;

		seed_str = seed_str.get(seeds_prompt_len..).ok_or(
			ParseStructError::new::<Self>(s)
				.at(s.len())
				.because("string too short"),
		)?;
		let seeds: Vec<u64> = seed_str
			.split(' ')
			.map(|s| s.trim())
			.filter(|s| !s.is_empty())
			.map(|s| s.parse::<u64>())
			.collect::<Result<_, _>>()
			.map_err(|e| {
				ParseStructError::new::<Self>(s)
					.at(seeds_prompt_len)
					.because(e)
			})?;

		let mapper_sequence: MapperSequence = mapper_sequence_str
			.parse()
			.map_err(|e| ParseStructError::new::<Self>(s).because(e))?;

		let mapper = mapper_sequence.flatten();

		Ok(Almanac { seeds, mapper })
	}
}
