extern crate num;

use num::{BigUint, One};


fn power(a: num::BigUint, b: usize) -> num::BigUint {
	let mut result = a.clone();
	for _ in 0..(b-1) {
		result = result * &a;
	}

	result
}


fn main() {
	let mut power_count = 0;
	for target_power in 1..200 {

		// Iterate through all numbers i until i ^ target_power > target_power 
		// (when the condition can no longer hold)
		let mut i = 0;
		loop {
			i += 1;
			let big_i = i.to_string().parse::<BigUint>().unwrap();
			let res_str = power(big_i, target_power).to_str_radix(10);

			if res_str.len() < target_power { continue; }
			if res_str.len() == target_power { power_count += 1; }
			if res_str.len() > target_power { break; }
		}
	}

	println!("Total count: {:?}", power_count);
}
