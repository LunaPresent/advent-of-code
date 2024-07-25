use std::{
	env,
	error::Error,
	io::{self, BufRead, BufReader},
};

use scratchcard::Scratchcard;

mod scratchcard;

struct ScratchcardResult {
	copies: u32,
	win_count: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
	let part = match env::args().find(|x| x == "part2") {
		None => 1,
		Some(_) => 2,
	};

	let mut val: u32 = 0;
	let mut results: Vec<ScratchcardResult> = vec![];
	let mut max_win_count = 0;
	for line in BufReader::new(io::stdin()).lines() {
		let win_count: u32 = line?.parse::<Scratchcard>()?.win_count();
		match part {
			1 => {
				val += match win_count {
					0 => 0,
					n => 2_u32.pow(n - 1),
				};
			}
			2 => {
				let mut copies = 1;
				for i in 1..=max_win_count {
					if i as usize > results.len() {
						break;
					}
					let r = &results[results.len() - i as usize];
					if r.win_count >= i {
						copies += r.copies;
					}
				}
				results.push(ScratchcardResult { copies, win_count });
				if win_count > max_win_count {
					max_win_count = win_count;
				}
				val += copies;
			}
			_ => (),
		}
	}

	println!("{}", val);

	Ok(())
}
