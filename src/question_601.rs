fn factorial(n: u64) -> u64 {
	if n == 1 { return 1; }
	n * factorial(n-1)
}


fn euclid_step(a: u64, b: u64) -> u64 {
	if a % b == 0 { return b; }

	euclid_step(b, a % b)
}


fn hcf(a: u64, b: u64) -> u64 {
	if a >= b {
		return euclid_step(a, b);
	} 
	euclid_step(b, a)
}

fn main() {
	/*
	streak(n) = 'first positive integer that does not divide n-1'
	streak(13) = 5, as 1,2,3,4 divide 13-1=12; but 5 does not.

	Therefore P(5, 4^4) = Number of times 1*2*3*4 is found in [1, 4^4-1]

	Pick some range a < n < b, and some i.
	Then P(i, range) = Number of n s.t. 1, 2, 3..., i-1 divide n, but i does not
	*/


	let i = 6 as u64;
	let range = 10 as u64;
	// Need minimal numbers s.t. 1...6 divide fac
	let fac = 1*2*2*3*5;
	let num_in_range = range.pow(6 as u32) / fac;

	// Need to adjust for when fac + 1 divide our number
	let act_num = num_in_range * 6 / 7;

	println!("{:?}", hcf(fac, i+1));
	println!("{:?}", fac);
	println!("{:?}", act_num);
}
