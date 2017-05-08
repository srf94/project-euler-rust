extern crate num;

use num::{BigUint, BigInt, One};


fn power(a: num::BigUint, b: usize) -> num::BigUint {
	let mut result = a.clone();
	for _ in 0..(b-1) {
		result = result * &a;
	}

	result
}


fn main() {
	let mut max_digit_sum = 0;

	for b in 1..100 {
		for number in 0..100 {
			let n = number.to_string();
			let mut big_n = BigUint::parse_bytes(n.as_bytes(), 10).unwrap();

			let a_to_b = power(big_n, b).to_str_radix(10);
			let a_to_b_vec = a_to_b.as_bytes();

			let mut sum = 0;
			for &digit in a_to_b_vec { sum += digit as u64 - 48; }

			if sum > max_digit_sum { max_digit_sum = sum; }
	    }
	}

    println!("Max digital sum: {:?}", max_digit_sum);
}


