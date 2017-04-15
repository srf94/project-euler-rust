extern crate num;

use num::{BigUint, BigInt, Zero, One};


fn biguint(num: usize) -> BigUint {
	let n = num.to_string();
	BigUint::parse_bytes(n.as_bytes(), 10).unwrap()
}


fn main() {
	let mut total_sum = 0;

	// Easiest to define biguint constants
	let one = biguint(1);
	let ten = biguint(10);
	let twenty = biguint(20);
	let hundred = biguint(100);

	for n in 2..100 {
		// If number is a square we do not continue
		let sqrt = (n as f64).sqrt() as u32;
		if sqrt * sqrt == n { continue; }

		// Calculate the square root expansion, storing u32 digits in 'expansion'
		let mut number = BigUint::parse_bytes(n.to_string().as_bytes(), 10).unwrap();
		let mut top = number;
		let mut p: BigUint = Zero::zero();
		let mut expansion = Vec::new();

		for _ in 0..100 {
			// Greatest x s.t. x(20p + x) <= c
			let mut x = One::one();
			while &x*(&twenty*&p + &x) <= top {
				x = &x + &one;
			}
			x = &x - &one;

			let y = &x*(&twenty*&p + &x);
			let remainder = &top - &y;

			p = &ten*&p + &x;
			top = &hundred*&remainder;

			// Save the next digit as a u32
			expansion.push(x.to_str_radix(10).parse::<u32>().unwrap());
		}
		
		total_sum += expansion.iter().sum::<u32>();
	}

	println!("Total digit sum of first 100: {:?}", total_sum);
}







