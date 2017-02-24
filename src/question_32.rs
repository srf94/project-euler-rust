// 100 * 100 = 10000
// 1000 * 10 = 10000
// 99 * 99 < 10000 < 100000
// Must be 5 / 4 split, eg ab * cde = wxyz

use std::collections::HashSet;


fn get_all_perms (digits: Vec<usize>) -> Vec<usize> {
	let mut perms = Vec::new();
	for digit in digits { 
		let mut digits_copy = digits;
		digits_copy.remove(digit);
		
		let mut sub_perms = get_all_perms(digits_copy);
		for sub_perm in sub_perms {
			
		}
	}
	
	rh_digits
}


fn get_rh_digits (lh_digits: &Vec<usize>) -> Vec<usize> {
	let mut rh_digits = Vec::new();
	for i in 1..10 { 
		if !lh_digits.contains(&i) { rh_digits.push(i); }
	}
	rh_digits
}


fn one_four_case (lh_digits: Vec<usize>) -> usize {
	let rh_digits = get_rh_digits(&lh_digits);
	
	for (i, digit) in lh_digits.iter().enumerate() {
		
		
    	println!("{}, {}", i, digit);
	}
	
	0
}


fn main() {
	// let mut allowed_products = HashSet::new();
	let var = vec![1,2,3,4,5];
	
    println!("{}", one_four_case(var));

}

