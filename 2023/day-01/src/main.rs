use regex::Regex;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
	let mut calibration_total: u32 = 0;

	let calibration_fn: fn(&str) -> u32 = match std::env::args().find(|x| x == "part2") {
		None => calibration_value,
		Some(..) => calibration_value_with_text,
	};

	for line in BufReader::new(io::stdin()).lines() {
		match line {
			Ok(s) => {
				calibration_total += calibration_fn(&s);
			}
			Err(e) => return Err(e),
		}
	}
	println!("{}", calibration_total);
	Ok(())
}

fn calibration_value(code: &str) -> u32 {
	let mut left: char = '0';
	for ch in code.chars() {
		if ch >= '0' && ch <= '9' {
			left = ch;
			break;
		}
	}
	let mut right: char = '0';
	for ch in code.chars().rev() {
		if ch >= '0' && ch <= '9' {
			right = ch;
			break;
		}
	}

	(left as u32 - '0' as u32) * 10 + right as u32 - '0' as u32
}

fn calibration_value_with_text(code: &str) -> u32 {
	let re_left = Regex::new(r".*?(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
	let re_right = Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
	let left: u32 = match re_left.captures(code) {
		Some(c) => parse_number(&c[1]),
		None => 0,
	};
	let right: u32 = match re_right.captures(code) {
		Some(c) => parse_number(&c[1]),
		None => left,
	};

	left * 10 + right
}

fn parse_number(number: &str) -> u32 {
	match number {
		"one" | "1" => 1,
		"two" | "2" => 2,
		"three" | "3" => 3,
		"four" | "4" => 4,
		"five" | "5" => 5,
		"six" | "6" => 6,
		"seven" | "7" => 7,
		"eight" | "8" => 8,
		"nine" | "9" => 9,
		_ => 0,
	}
}
