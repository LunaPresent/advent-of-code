use std::{
	any::type_name,
	error::Error,
	fmt::{Debug, Display},
};

pub struct ParseStructError {
	struct_name: String,
	from_string: String,
	at_char: Option<usize>,
	reason: Option<Box<dyn Display>>,
}

impl ParseStructError {
	pub fn new<T>(from_string: impl ToString) -> Self {
		Self {
			struct_name: type_name::<T>().to_owned(),
			from_string: from_string.to_string(),
			at_char: None,
			reason: None,
		}
	}

	pub fn at(mut self, char_idx: usize) -> Self {
		self.at_char = Some(char_idx);
		self
	}

	pub fn because<T: Display + 'static>(mut self, reason: T) -> Self {
		self.reason = Some(Box::new(reason));
		self
	}
}

impl Error for ParseStructError {}

impl Debug for ParseStructError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self, f)
	}
}

impl Display for ParseStructError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "error while parsing {}", self.struct_name)
			.and_then(|_| match self.at_char {
				Some(i) => write!(
					f,
					" at index {} in string\n\"{}\"\n {}^{}",
					if (i) < self.from_string.len() {
						i
					} else {
						self.from_string.len()
					},
					self.from_string,
					"~".repeat(if (i) < self.from_string.len() {
						i
					} else {
						self.from_string.len()
					}),
					"~".repeat(if (i) < self.from_string.len() {
						self.from_string.len() - i - 1
					} else {
						0
					})
				),
				None => write!(f, " from string \"{}\"", self.from_string),
			})
			.and_then(|_| match &self.reason {
				Some(reason) => write!(f, "\nReason: {}", &**reason),
				None => Ok(()),
			})
	}
}
