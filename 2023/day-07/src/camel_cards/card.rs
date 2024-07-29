use common::parse_struct_error::ParseStructError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card(u8);

impl Card {
	pub fn is_wildcard(&self) -> bool {
		self.0 == 0
	}
}

impl TryFrom<char> for Card {
	type Error = ParseStructError;

	fn try_from(c: char) -> Result<Self, Self::Error> {
		Ok(Card(match c {
			'*' => 0,
			'2'..='9' => c as u8 - '1' as u8,
			'T' => 9,
			'J' => 10,
			'Q' => 11,
			'K' => 12,
			'A' => 13,
			_ => Err(ParseStructError::new::<Self>(c))?,
		}))
	}
}
