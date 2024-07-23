use std::{any::type_name, error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseStructError {
	struct_name: String,
	from_string: String,
}

impl ParseStructError {
	pub fn new<T>(from_string: &str) -> Self {
		Self {
			struct_name: type_name::<T>().to_owned(),
			from_string: from_string.to_owned(),
		}
	}
}

impl Error for ParseStructError {}

impl Display for ParseStructError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"error while parsing {} from string \"{}\"",
			self.struct_name, self.from_string
		)
	}
}
