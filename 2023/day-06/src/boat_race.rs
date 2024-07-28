#[derive(Debug, Clone, Copy)]
pub struct BoatRace {
	pub time: u64,
	pub record: u64,
}

impl BoatRace {
	fn min_win_press_time(&self) -> u64 {
		((self.time as f64 - ((self.time * self.time - 4 * self.record) as f64).sqrt()) / 2.) as u64
			+ 1
	}

	pub fn time_win_range(&self) -> u64 {
		let min = self.min_win_press_time();
		let max = self.time - min;
		max - min + 1
	}
}
