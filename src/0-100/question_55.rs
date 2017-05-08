extern crate num;

use num::{BigUint, BigInt, One};


fn is_palindromic(int: &BigUint) -> bool {
	let str = int.to_str_radix(10 as u32).chars().collect::<String>();
	let rev = int.to_str_radix(10 as u32).chars().rev().collect::<String>();

	str == rev
}


fn rev_n_add(int: &BigUint) -> BigUint {
	let rev_str = int.to_str_radix(10 as u32).chars().rev().collect::<String>();
	let rev_int = BigUint::parse_bytes(rev_str.as_bytes(), 10).unwrap();

	int + rev_int
}


fn main() {
	let max = 10000;
	let mut count = 0;
		
	for number in 0..max {
		let n = number.to_string();
		let mut big_n = BigUint::parse_bytes(n.as_bytes(), 10).unwrap();

		for _ in 0..100 {
			big_n = rev_n_add(&big_n);
			if is_palindromic(&big_n) {
				count += 1;
    			break;
			}
		}
	}

    println!("Number of Lychrel numbers: {}", max - count);
}