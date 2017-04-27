extern crate num;

use num::{BigUint, BigInt, One};


fn main() {
	let exponent = 7830457;

	let mut value: i64 = 1;
	for _ in 0..exponent {
		value *= 2;
		value = value % 10000000000;
	}

	value *= 28433;
	value += 1;
	value = value % 10000000000;

	println!("Last 10 digits: {:?}", value);
}
