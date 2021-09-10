use crate::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Key {
	pub red: u8,
	pub white: u8
}

#[wasm_bindgen]
impl Key {
	pub fn new(red: u8, white: u8) -> Self {
		Key {
			red,
			white
		}
	}

	pub fn from_ordinal(num: u32) -> Self {
		// cantor pairing function
		let w: u32 = (((8 * num + 1) as f32).sqrt().floor() as u32 - 1) / 2;

		let red = num as u8 - ((w * w + w) / 2) as u8;
		let white = w as u8 - red;

		Key {
			red,
			white
		}
	}

	pub fn to_ordinal(&self) -> u32 {
		// cantor pairing function
		let w = self.red as u32 + self.white as u32;
		(w + 1) * w / 2 + self.red as u32
	}

	pub fn from_code_and_guess(code: &Code, guess: &Code) -> Self {
		let mut color_counts_code = [0u8; NUM_COLORS as usize];
		let mut color_counts_guess = [0u8; NUM_COLORS as usize];

		let mut color_matches = 0;
		let mut exact_matches = 0;

		for i in 0..NUM_SLOTS {
			color_counts_code[code.0[i as usize] as usize] += 1;
			color_counts_guess[guess.0[i as usize] as usize] += 1;
			if code.0[i as usize] == guess.0[i as usize] {
				exact_matches += 1;
			}
		}

		for i in 0..NUM_COLORS {
			while color_counts_code[i as usize] > 0 && color_counts_guess[i as usize] > 0 {
				color_counts_code[i as usize] -= 1;
				color_counts_guess[i as usize] -= 1;
				color_matches += 1;
			}
		}

		Key { 
			red: exact_matches,
			white: color_matches - exact_matches
		}
	}
}

pub mod prelude {
	pub use super::Key;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn key_ordinals() {
		for red in 0..=4 {
			for white in 0..=4-red {
				let key = Key::new(red, white);
				let recovered = Key::from_ordinal(key.to_ordinal());

				assert_eq!(key, recovered);
			}
		}
	}

	#[test]
	fn key_from_code_and_guess() {
		let test_cases = vec![
			(
				Code([2, 1, 4, 5]),
				Code([5, 2, 5, 4]),
				Key {
					red: 0,
					white: 3
				}
			),
			(
				Code([5, 0, 3, 5]),
				Code([4, 5, 4, 5]),
				Key {
					red: 1,
					white: 1
				}
			),
			(
				Code([3, 5, 2, 4]),
				Code([0, 3, 1, 3]),
				Key {
					red: 0,
					white: 1
				}
			),
			(
				Code([2, 4, 4, 3]),
				Code([2, 4, 4, 3]),
				Key {
					red: 4,
					white: 0
				}
			),
		];

		for &(code, guess, expected) in test_cases.iter() {
			let key = Key::from_code_and_guess(&code, &guess);

			assert_eq!(key, expected);
		}
	}
}