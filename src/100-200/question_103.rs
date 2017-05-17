use std::collections::HashSet;


fn check_set(mut elements: Vec<u32>) -> bool {
	// Check condition one: S(B) > S(C) if |B| > |C|
	for i in 2..(n/2 + 1) {
		let smallest_sum: u32 = elements[0..i].iter().sum();
		let largest_sum: u32 = elements[n..n-i].iter().sum();
		if smallest_sum < largest_sum { return false; }
	}

	// Check condition two: Sum of subsets are never equal
	let n = elements.len();
	elements.sort();
	
	// Straight away - need all numbers to be distinct
	let set: HashSet<_> = elements.drain(..).collect();
	if set.len() < n { return false; }

	// TODO!


	true
}



fn main() {

	let starting_guess = vec![11, 18, 19, 20, 22, 25];

	println!("{:?}", check_set(vec![2, 3, 4]));

    println!("Hello, world!");
}
