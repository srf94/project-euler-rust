extern crate num;

use num::{BigUint, BigInt, One};


fn main() {
	let one = One::one();
	let two = &one + &one;
	let three = &one + &one + &one;

	let mut tri_n: BigUint = One::one();
	let mut pent_n: BigUint = One::one();
	let mut hex_n: BigUint = One::one();

	let mut pent = (&pent_n * (&three * &pent_n - &one)) / &two;
	let mut hex = &hex_n * (&two * &hex_n - &one);

	for k in 1..100000 {
		let mut tri: BigUint = &tri_n * (&tri_n + &one) / &two;

		while pent < tri {
			pent_n = &pent_n + &one;
			pent = (&pent_n * (&three * &pent_n - &one)) / &two;
		}

		while hex < tri {
			hex_n = &hex_n + &one;
			hex = &hex_n * (&two * &hex_n - &one);
		}
		
		if pent == tri && hex == tri {
			println!("Match! {}", tri);
		}

		tri_n = &tri_n + &one;
	}


}
