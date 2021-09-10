use crate::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Game {
	remaining_possible_codes: [bool; NUM_CODES as usize],
	num_remaining_possible_codes: u32,
	pub guesses_made: u32,
	guesses: [Option<Code>; MAX_GUESSES as usize],
	keys: [Option<Key>; MAX_GUESSES as usize]
}

#[wasm_bindgen]
impl Game {
	pub fn new() -> Self {
		Game {
			remaining_possible_codes: [true; NUM_CODES as usize],
			num_remaining_possible_codes: NUM_CODES,
			guesses_made: 0,
			guesses: [None; MAX_GUESSES as usize],
			keys: [None; MAX_GUESSES as usize]
		}
	}

	pub fn reset(&mut self) {
		*self = Game::new();
	}

	// pub fn most_recent_guess(&self) -> Code {
	// 	if self.guesses_made == 0 {
	// 		panic!("No guesses made");
	// 	}

	// 	self.guesses[(self.guesses_made - 1) as usize].expect("Guess array mismatched with total guesses made")
	// }

	pub fn print_num_possible_codes(&self) {
		web_sys::console::log_2(&"Num remaining possible codes: ".into(), &self.num_remaining_possible_codes.into());
	}

	pub fn guesses(&self) -> JsValue {
		JsValue::from_serde(&self.guesses).unwrap()
	}

	pub fn print_possible_codes(&self) {
		for (number, _code) in CodeIterator::new() {
			if !self.remaining_possible_codes[number as usize] {
				continue;
			}
		}
	}

	pub fn rollback(&mut self, num_guesses_rollback: u32) {
		// if it would underflow, make sure it stays at 0
		self.guesses_made = self.guesses_made.checked_sub(num_guesses_rollback).map_or(0, |x| x);

		for i in self.guesses_made..MAX_GUESSES {
			self.guesses[i as usize] = None;
			self.keys[i as usize] = None;
		}

		for (number, _code) in CodeIterator::new() {
			self.remaining_possible_codes[number as usize] = true;
		}
		self.num_remaining_possible_codes = NUM_CODES;

		for i in 0..self.guesses_made {
			self.prune(&self.guesses[i as usize].expect("Unexpected None value in guesses"), &self.keys[i as usize].expect("Unexpected None value in keys"));
		}
	}

	pub fn prune(&mut self, guess: &Code, key: &Key) {
		for (number, code) in CodeIterator::new() {
			if !self.remaining_possible_codes[number as usize] {
				continue;
			}

			if *key != Key::from_code_and_guess(&code, &guess) {
				self.remaining_possible_codes[number as usize] = false;
				self.num_remaining_possible_codes -= 1;
			}
		}
	}

	pub fn prune_most_recent(&mut self) {
		self.prune(&self.guesses[(self.guesses_made - 1) as usize].expect("Unexpected None value in guesses"), &self.keys[(self.guesses_made - 1) as usize].expect("Unexpected None value in keys"));
	}

	pub fn count_pruned_in_place(&self, guess: &Code, key: &Key) -> u32 {
		let mut pruned = 0;

		for (number, code) in CodeIterator::new() {
			if !self.remaining_possible_codes[number as usize] {
				continue;
			}

			if *key != Key::from_code_and_guess(&code, &guess) {
				pruned += 1;
			}
		}

		pruned
	}

	fn lookahead(&self, guess: &Code) -> u32 {
		let mut min_score: u32 = u32::MAX;

		let mut valid_keys = [false; NUM_KEYS as usize];

		for (number, code) in CodeIterator::new() {
			if !self.remaining_possible_codes[number as usize] {
				continue;
			}

			let key = Key::from_code_and_guess(&code, &guess);

			valid_keys[key.to_ordinal() as usize] = true;
		}

		for i in 0..NUM_KEYS {
			if !valid_keys[i as usize] {
				continue;
			}

			let score = self.count_pruned_in_place(&guess, &Key::from_ordinal(i));

			if score < min_score {
				min_score = score;
			}
		}

		min_score
	}

	pub fn next_guess(&mut self) -> Code {
		if self.guesses_made == 0 {
			self.guesses[self.guesses_made as usize] = Some(Code([0, 0, 1, 1]));
		} else {
			if self.num_remaining_possible_codes == 1 {
				let mut code = Code([0, 0, 0, 0]);

				for (number, c) in CodeIterator::new() {
					if self.remaining_possible_codes[number as usize] {
						code = c;
					}
				}

				self.guesses[self.guesses_made as usize] = Some(code);
			} else {
				let mut best_next_guess = (Code::from_ordinal(0), 0);

				for (_number, code) in CodeIterator::new() {
					let next_guess = code;

					let score = self.lookahead(&next_guess);

					if score > best_next_guess.1 {
						best_next_guess = (next_guess, score);
					}
				}

				self.guesses[self.guesses_made as usize] = Some(best_next_guess.0);
			}
		}

		self.guesses_made += 1;
		self.guesses[(self.guesses_made - 1) as usize].expect("Guess array mismatched with total guesses made")
	}

	pub fn user_input(&mut self, key: Key) {
		self.keys[(self.guesses_made - 1) as usize] = Some(key);
		self.prune_most_recent();
	}
}

pub mod prelude {
	pub use super::Game;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn game_one_possibility() {
		let mut game = Game::new();
		let code = Code([2, 1, 5, 5]);
		let key = Key { red: 4, white: 0 };

		game.prune(&code, &key);

		assert_eq!(game.num_remaining_possible_codes, 1);
	}
}