extern crate num;

use num::{BigUint, One};

fn main() {
	let mut prod: BigUint = One::one();

    for _ in 0..1000 {
		prod = &prod + &prod;
	}

    println!("{}", prod);
	
	let string = prod.to_str_radix(10);
	let digits = string.as_bytes();
	
	let mut sum = 0;
	for &ii in digits.iter() {
		sum += (ii - 48) as usize;
	}

    println!("{}", sum);
}
