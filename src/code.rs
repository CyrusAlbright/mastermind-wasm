use crate::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Code(pub(crate) [u8; NUM_SLOTS as usize]);

#[wasm_bindgen]
impl Code {
	pub fn new(value: &JsValue) -> Self {
		let array: [u8; NUM_SLOTS as usize] = value.into_serde().expect("Input to Code constructor has to be array of non-negative integers");

		Code(array)
	}

	pub fn values(&self) -> JsValue {
		JsValue::from_serde(&self.0).expect("Failed to serialize Code")
	}

	pub fn from_ordinal(num: u32) -> Self {
		// separate number into its digits in base n
		let mut code = Code([0; NUM_SLOTS as usize]);
		let mut remainder = num;

		for i in (0..NUM_SLOTS).rev() {
			code.0[i as usize] = (remainder % NUM_COLORS as u32) as u8;
			remainder = remainder / NUM_COLORS as u32;
		}

		code
	}

	pub fn to_ordinal(&self) -> u32 {
		// turn digits in base n to number
		let mut sum = 0;

		for i in 0..NUM_SLOTS as u32 {
			sum += (NUM_COLORS as u32).pow(NUM_SLOTS as u32 - 1 - i) * self.0[i as usize] as u32;
		}

		sum
	}

	pub fn to_string(&self) -> String {
		format!("{}{}{}{}", self.0[0], self.0[1], self.0[2], self.0[3])
	}
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct CodeIterator {
	number: u32,
	code: Code
}

impl CodeIterator {
	pub fn new() -> Self {
		CodeIterator {
			number: 0,
			code: Code([0; NUM_SLOTS as usize])
		}
	}
}

impl Iterator for CodeIterator {
	type Item = (u32, Code);

	fn next(&mut self) -> Option<Self::Item> {
		if self.code.0[0] >= NUM_COLORS {
			return None;
		}

		let current = (self.number, self.code);

		self.number += 1;

		let mut index: usize = (NUM_SLOTS - 1) as usize;
		self.code.0[index] += 1;
				
		while self.code.0[index] >= NUM_COLORS && index > 0 {
			self.code.0[index] = 0;
			index -= 1;
			self.code.0[index] += 1;
		}

		Some(current)
	}
}

pub mod prelude {
	pub use super::Code;
	pub(crate) use super::CodeIterator;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn code_ordinals() {
		for (_number, code) in CodeIterator::new() {
			let recovered = Code::from_ordinal(code.to_ordinal());

			assert_eq!(code, recovered);
		}
	}

	#[test]
	fn code_iterator() {
		let test_cases = vec![
			(
				Code([3, 4, 0, 3]),
				Some(Code([3, 4, 0, 4]))
			),
			(
				Code([1, 3, 5, 5]),
				Some(Code([1, 4, 0, 0]))
			),
			(
				Code([5, 5, 5, 5]),
				None
			)
		];

		for &(code, expected_next) in test_cases.iter() {
			let mut code_iterator = CodeIterator {
				number: code.to_ordinal(),
				code
			};

			code_iterator.next();
			let next = code_iterator.next();
			let expected_next = expected_next.map(|c| (code.to_ordinal() + 1, c));
			assert_eq!(next, expected_next);
		}
	}
}