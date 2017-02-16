extern crate num;

use num::{BigUint, One};

fn main() {
    let mut f_one: BigUint = One::one();
    let mut f_two: BigUint = One::one();

	let mut index = 3;
	loop {
		if f_two.to_str_radix(10).len() >= 1000 { break; }
		
		let sum = &f_one + &f_two;
		f_one = f_two;
		f_two = sum;
		index += 1;	
	}

    println!("Index of first 1000 digit Fibonacci number: {}", index);    
}
