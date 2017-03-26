extern crate num;

use num::{BigUint, One};
use std::collections::HashMap;


fn main() {

	let target_permutations = 5;

	let mut cube_length = 1;
	'main_loop: loop {
		// Create a hashmap of sorted bytes of n cubed -> n
		// Eg, if n = 8, n^3 = 512, or 53, 49, 50 in bytes.
		// Sort the bytes to get [49, 50, 53], and store:
		//     [49, 50, 53] -> 8
		let mut digits_to_n = HashMap::new();

		// Need to use BigUInts as numbers get large fast
		let mut one: BigUint = One::one();
		let mut j: BigUint = One::one();

		loop {
			j = &j + &one;
			let usize_j = j.to_str_radix(10).parse::<usize>().unwrap();
			let cube = &j * &j * &j;
			let string = cube.to_str_radix(10);

			// We are only interested in cubes of digit length 'cube_length' here
			if string.len() < cube_length { continue; }
			if string.len() > cube_length { break; }

			let mut bytes = string.into_bytes();
			bytes.sort();

			digits_to_n.entry(bytes).or_insert(Vec::new()).push(usize_j);
		}

		// Iterate though digits_to_n to see if any cubes are n-permutable
		// If they are, save all such primes
		let mut suitable_primes = Vec::new();
		for (key, val) in digits_to_n.iter() {
			if val.len() == target_permutations {
				for prime in val { suitable_primes.push(prime) }
			}
		}

		if suitable_primes.len() > 0 {
			// We know we have some cubes that are n-permutable
			// Find and prime smallest cube

			let smallest_prime = suitable_primes.iter().min().unwrap();
			println!("Smallest such prime: {:?}", smallest_prime);

			let big_int_prime = smallest_prime.to_string().parse::<BigUint>().unwrap();
			let big_int_cube = &big_int_prime * &big_int_prime * &big_int_prime;
			println!("Cube of this prime: {}", big_int_cube.to_str_radix(10));
			break 'main_loop;
		}

		cube_length += 1;
	}

}
