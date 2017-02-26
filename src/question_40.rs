extern crate num;

use std::ops::Mul;
use num::{BigUint, One};

fn main() {

	let mut n = 0;
	let mut count = 0;
	let mut digits = Vec::new();
	//digits.push(1);
	
	let locations_wanted = [1, 10, 100, 1000, 10000, 100000, 1000000];
	
	//while count <= 20 {
	while count <= 1000000 {
		n += 1;
		for i in 0..n.to_string().len() {
			count += 1;

			//let var = &n.to_string();
			//let var2 = &var.chars()[0];

	    	//println!("n: {}, n slice: {:?}", n, );

			if locations_wanted.contains(&count) {
				//let var: () = &n.to_string()[i..i+1].parse::<usize>().unwrap();
	    		println!("To push: {}", &n.to_string()[i..i+1]);
				//product *= n;
				//let to_push = n.to_string();
				//let to_push_2 = to_push[i..i+1];
				digits.push(&n.to_string()[i..i+1].parse::<usize>().unwrap());
			}
		}
	}

    //println!("Product: {}", product);
    println!("Digits: {:?}", digits);

	let mut product: BigUint = One::one();
	for digit in digits {
    	println!("Product: {}", product);
		//product = product * digit;
		//let var = BigUint::from_usize(&digit);
		//product = product.mul(&digit);
	}
	
}
