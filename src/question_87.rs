use std::collections::HashSet;


fn is_prime(int: usize) -> bool {
	let max = (int as f64).sqrt() as usize + 1;

	for i in 2..max {
		if int % i == 0 { return false; }
	}
	true
}


fn primes_under_n(n: usize) -> Vec<usize> {
	let mut primes = vec![2];
	for i in 3..n+1 {
		if is_prime(i) {
			primes.push(i);
		}
	}
	primes
}


fn main() {
	/*
	Consider all sums of the form a^2 + b^3 + c^4 (a, b, c all prime)
	As 7071^2 < 50,000,000 < 7072^2, a <= 7071
	As 368^2 < 50,000,000 < 369^2, b <= 368
	As 84^2 < 50,000,000 < 85^2, c <= 84

	In total this gives us 1,500,000 possible sums. 
	Note different a,b,c may sum to the same value.
	*/
	let upper_limit = 50000000;

	let primes_a = primes_under_n(7072);
	let primes_b = primes_under_n(369);
	let primes_c = primes_under_n(84);

	let mut created_ints = HashSet::new();

	for a in &primes_a {
	for b in &primes_b {
	for c in &primes_c {
		let new_int = a*a + b*b*b + c*c*c*c;
		if new_int >= upper_limit { continue; }

		if created_ints.contains(&new_int) { continue; }

		created_ints.insert(new_int);
	}}}

	println!("Number of integers that can be expressed this way: {:?}", created_ints.len());

}






