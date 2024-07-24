use std::str::FromStr;

use common::parse_struct_error::ParseStructError;

#[derive(Clone, Copy)]
pub struct CubeSet {
	pub red: u32,
	pub green: u32,
	pub blue: u32,
}

impl FromStr for CubeSet {
	type Err = ParseStructError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut colours = Vec::<&str>::with_capacity(3);
		let mut head = 0;
		while let Some(i) = s.get(head..).and_then(|s| s.find(", ")) {
			colours.push(&s[head..head + i]);
			head = head + i + 2;
		}
		colours.push(&s[head..]);

		let (mut red, mut green, mut blue) = (0, 0, 0);
		for c in colours {
			let i: usize = c
				.find(" ")
				.ok_or(ParseStructError::new::<CubeSet>(s).at(head))?;
			let n: u32 = c[..i]
				.parse()
				.map_err(|e| ParseStructError::new::<CubeSet>(s).at(head).because(e))?;
			let colour: &str = &c[i + 1..];

			match colour {
				"red" => red = n,
				"green" => green = n,
				"blue" => blue = n,
				_ => {
					return Err(ParseStructError::new::<CubeSet>(s)
						.at(i + 1)
						.because("invalid colour"))
				}
			};
		}

		Ok(CubeSet { red, green, blue })
	}
}

impl CubeSet {
	pub fn power(&self) -> u32 {
		self.red * self.green * self.blue
	}
}
