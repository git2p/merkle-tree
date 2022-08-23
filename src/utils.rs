//TODO: recognize cpu architechture and add in group of 4 or 8 bytes
pub fn hash_adder(left: Vec<u8>, right: Vec<u8>) -> Vec<u8> {
	let mut res: Vec<u8> = vec![];
	let mut carry = false;

	let (left, right) = if right.len() > left.len() {
		(right, left)
	} else {
		(left, right)
	};

	for (i, v) in left.iter().rev().enumerate() {
		let mut sum = if carry { 1u8 } else { 0u8 };

		if right.len() > i {
			let r = right[right.len() - i - 1];

			let s = v.overflowing_add(r);

			sum += s.0;

			carry = s.1;
		} else {
			sum += v;

			carry = false;
		}
		res.insert(0, sum);
	}

	if carry {
		res.insert(0, 1);
	}

	res
}

#[cfg(test)]
mod hash_adder {
	use super::*;

	#[test]
	fn add() {
		let left = 120500u64;
		let right = 200500360u64;

		let expected = left + right;

		let result = hash_adder(left.to_be_bytes().to_vec(), right.to_be_bytes().to_vec());

		assert_eq!(expected.to_be_bytes().to_vec(), result);
	}

	#[test]
	fn add_different_sizes() {
		let left = 120500u128;
		let right = 200500360u32;

		let expected = left + right as u128;

		let result = hash_adder(left.to_be_bytes().to_vec(), right.to_be_bytes().to_vec());

		assert_eq!(expected.to_be_bytes().to_vec(), result);
	}

	#[test]
	fn add_different_sizes_invert() {
		let left = 120500u32;
		let right = 200500360u128;

		let expected = left as u128 + right;

		let result = hash_adder(left.to_be_bytes().to_vec(), right.to_be_bytes().to_vec());

		assert_eq!(expected.to_be_bytes().to_vec(), result);
	}
}
