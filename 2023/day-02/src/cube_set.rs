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
		colours.push(s.get(head..).ok_or(ParseStructError::new::<CubeSet>(s))?);

		let (mut red, mut green, mut blue) = (0, 0, 0);
		for s in colours {
			let i: usize = s.find(" ").ok_or(ParseStructError::new::<CubeSet>(s))?;
			let n: u32 = s[..i]
				.parse()
				.map_err(|_| ParseStructError::new::<CubeSet>(s))?;
			let colour: &str = &s[i + 1..];

			match colour {
				"red" => red = n,
				"green" => green = n,
				"blue" => blue = n,
				_ => return Err(ParseStructError::new::<CubeSet>(s)),
			};
		}

		Ok(CubeSet {
			red: red,
			green: green,
			blue: blue,
		})
	}
}

impl CubeSet {
	pub fn power(&self) -> u32 {
		self.red * self.green * self.blue
	}
}
