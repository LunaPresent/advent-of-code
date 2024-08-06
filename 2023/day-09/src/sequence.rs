use std::{iter, num::ParseIntError, str::FromStr};

use common::parse_struct_error::ParseStructError;

#[derive(Debug)]
pub struct Sequence {
	coefficients: Vec<f64>,
	len: usize,
}

impl Sequence {
	pub fn len(&self) -> usize {
		self.len
	}

	pub fn nth(&self, n: i128) -> i128 {
		self.coefficients
			.iter()
			.enumerate()
			.map(|(exp, c)| c * n.pow(exp as u32) as f64)
			.sum::<f64>()
			.round() as i128
	}
}

impl FromStr for Sequence {
	type Err = ParseStructError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut sequence: Vec<i128> = s
			.split(' ')
			.map(|s| Ok::<i128, ParseIntError>(s.parse::<i128>()?))
			.collect::<Result<_, _>>()
			.map_err(|e| ParseStructError::new::<Self>(s).because(e))?;

		let mut i: usize = 0;
		let degree: usize = loop {
			let init: i128 = sequence[i];
			if sequence.iter().skip(i).all(|&n| n == init) {
				break i;
			}
			// leave the values required for solving the equations in 0..=degree
			// and use the rest of the array for the next difference row
			i += 1;
			sequence
				.iter_mut()
				.skip(i)
				.scan(init, |prev, n| {
					let diff = *n - *prev;
					*prev = *n;
					Some((n, diff))
				})
				.for_each(|(e, n)| *e = n);
		};

		let mut eqs: Vec<i128> = (0..=degree)
			.flat_map(|x| {
				iter::once(1).chain((1..=degree).map(move |exp| (x as i128).pow(exp as u32)))
			})
			.collect();
		for i in 1..=degree {
			for j in (i..=degree).rev() {
				for k in j * (degree + 1)..(j + 1) * (degree + 1) {
					eqs[k] = &eqs[k] - &eqs[k - (degree + 1)];
				}
			}
		}

		let mut coefficients: Vec<f64> = iter::repeat(0.).take(degree + 1).collect();

		for i in (0..=degree).rev() {
			let y = sequence[i] as f64; // constant expression value of the equation
			let n_i = i * (degree + 2); // index of the n_th coefficient's multiplier in the equation
			let n = eqs[n_i] as f64;
			let cs = coefficients[i + 1..degree + 1].iter();
			let ms = eqs[(n_i + 1)..((i + 1) * (degree + 1))].iter();
			coefficients[i] = (y - cs.zip(ms).map(|(&c, &m)| c * m as f64).sum::<f64>()) / n;
		}

		Ok(Sequence {
			coefficients,
			len: sequence.len(),
		})
	}
}
