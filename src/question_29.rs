extern crate num;

use std::collections::HashSet;
use num::BigUint;


fn main() {
	let mut power_hs = HashSet::new();
	
	for a_u8 in 2..101 {
		for b_u8 in 2..101 {
			let a = BigUint::from_bytes_be(&[a_u8 as u8]);
			power_hs.insert(num::pow(a, b_u8));
		}
	}
	println!("Distinct terms in sequence: {}", power_hs.len());
}
