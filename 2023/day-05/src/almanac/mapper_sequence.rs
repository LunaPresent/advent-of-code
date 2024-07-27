use std::str::FromStr;

use common::parse_struct_error::ParseStructError;

use super::mapper::Mapper;

#[derive(Debug)]
pub struct MapperSequence {
	mappers: Vec<Mapper>,
}

impl MapperSequence {
	pub fn flatten(&self) -> Mapper {
		self.mappers
			.iter()
			.fold(Mapper { mappings: vec![] }, |a, b| a.combine(b))
	}
}

impl FromStr for MapperSequence {
	type Err = ParseStructError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mappers = s
			.split("\n\n")
			.map(|s| s.parse())
			.collect::<Result<_, _>>()
			.map_err(|e| ParseStructError::new::<Self>(s).because(e))?;

		Ok(MapperSequence { mappers })
	}
}
