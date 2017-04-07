extern crate num;

use num::{BigUint, BigInt, One, Zero};
use std::collections::HashMap;


// fn fnc(sum: isize, largest: isize, mut saved_values: &mut HashMap<(isize, isize), isize>) -> isize {
fn fnc(sum: isize, largest: isize, mut saved_values: &mut HashMap<(isize, isize), BigUint>) -> BigUint {

	// if sum > 976 || largest > 976 {
		// println!("{:?}, {:?}", sum, largest);
		// println!("{:?}", &saved_values.contains_key(&(sum - largest, largest)));
	// }

	let one: BigUint = One::one();
	let zero: BigUint = Zero::zero();

	if largest == 0 { return zero; }
	if largest == 1 { return one; }
	if sum == 0 { return one; }
	if sum < 0 { return zero; }

	let mut part_1: BigUint;
	let mut part_2: BigUint;

	// 978, 455
	// -> 978, 454
	// -> 978-455
	// , 455


	if saved_values.contains_key(&(sum, largest - 1)) {
		part_1 = saved_values.get(&(sum, largest - 1)).unwrap().clone();
	} else {
		part_1 = fnc(sum, largest - 1, &mut saved_values);
	}

	if saved_values.contains_key(&(sum - largest, largest)) {
		part_2 = saved_values.get(&(sum - largest, largest)).unwrap().clone();
	} else {
		part_2 = fnc(sum - largest, largest, &mut saved_values);
	}

	let result = part_1 + part_2;

	if !saved_values.contains_key(&(sum, largest)) {
		saved_values.insert((sum, largest), result.clone());
	}

	result
}



fn main() {
	let target = "100000";

	let mut saved_values: HashMap<(isize, isize), BigUint> = HashMap::new();
	let mut big_target = BigUint::parse_bytes(target.as_bytes(), 10).unwrap();

	// let answer = fnc(600, 600, &mut saved_values);

	let seed = vec![1, 1, 2, 3, 5, 7, 11, 15, 22, 30];

	let i = 10;

	let mut pents = Vec::new();
	let mut k: isize = 1;
	loop {
		let pent = k * (3*k - 1) / 2;
		if pent > i { break; }
		pents.push(pent);

		if k > 0 {
			k = -k;
		} else {
			k = -(k - 1);
		}
	}

	let seed_len = seed.len();

	let mut sum = 0;
	for (i, pent) in pents.iter().enumerate() {
		println!("{:?}", i);
		println!("{:?}", pent);
	}

	// 0, 1 -> +
	// 2, 3 -> -

	// k * (3*k - 1) / 2



	// let mut i = 2;
	// for i in 2..10 {
	// 	let answer = fnc(i, i, &mut saved_values);
	// 	println!("{:?}", answer);

		// if (&answer % &big_target).to_str_radix(10) == "100" {
		// 	println!("{:?}, {}", i, answer.to_str_radix(10));
		// 	break;
		// }
		// // println!("{:?}", saved_values.len());

		// let mut keys_to_remove = Vec::new();
		// for (key, _) in &saved_values {
		// 	if key.0 + key.1 < i-5 {
		// 		keys_to_remove.push(key.clone());
		// 	}
		// }

		// for key in keys_to_remove {
		// 	saved_values.remove(&(key));
		// }

		// i += 1;
		// println!("{:?}", i);
		// println!("{:?}", saved_values.len());
		// break;
	// }
}
