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

	let mut factor = 1;
	let mut total = 0;

	for i in 1..top_lim + 1 {
		let range_top = (4 as u64).pow(i as u32);

		let streak_1 = (range_top - 2) / factor;

		// Need to remove any numbers that have a streak for i+1 or higher
		factor = factor * (i+1) / hcf(i+1, factor);
		let streak_n = streak_1 - (range_top - 2) / factor;
		
		total += streak_n;
	}

	// Minus 1 as the i=1 case counts streak(1) = 1 by mistake
	println!("Sum of P(i, 4^i) for 1 <= i <= 31: {:?}", total - 1);

}
