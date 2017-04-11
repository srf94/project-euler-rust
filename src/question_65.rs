extern crate num;

use num::{BigUint, BigInt, One};


fn main() {
	let nth_convergent = 100;
	let one: BigUint= One::one();

	// Set up sequence of convergents: [2, 1, 2, 1, 1, 4, 1, ... 1, 2k, 1, ... ]
	let mut convergent: Vec<BigUint> = vec![&one + &one];
	let mut even_num = 2;
	let mut counter = 0;
	while convergent.len() < nth_convergent {
		if counter % 3 == 1 {
			let n = even_num.to_string();
			let big_n = BigUint::parse_bytes(n.as_bytes(), 10).unwrap();
			convergent.push(big_n);
			even_num += 2;
		} else {
			convergent.push(One::one());
		}
		counter += 1;
	}

	let mut top = One::one();
	let mut bottom = convergent[convergent.len()-1].clone();

	// Add successive fractions, starting from the 'bottom'
	for (count, cc) in convergent.iter().rev().enumerate() {
		if count == 0 { continue; }
		if count == convergent.len() - 1 {
			top = &top + &bottom * &convergent[0];
			break;
		}

		top = &top + &bottom * cc;
		let mid = top;
		top = bottom;
		bottom = mid;
	}

	println!("Convergent {:?}:", nth_convergent);
	println!("top: {:?}", top);
	println!("bottom: {:?}", bottom);
	println!("");

	// Convert the BigInt to a vector of bytes
	let top_str = top.to_str_radix(10);
	let top_bytes = top_str.as_bytes();
	let mut byte_sum = top_bytes.iter().fold(0, |sum, x| sum + (*x as usize));

	// Need to subtract 48 for every byte!
	byte_sum -= 48 * top_bytes.len();

	println!("Sum of top digits: {:?}", byte_sum);

}


















