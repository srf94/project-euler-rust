fn is_prime(num: usize) -> bool {
	let search_max = (num as f64).sqrt() as usize + 1;
	
	for i in 2..search_max {
		if num % i == 0 { return false; }
	}
	true
}


fn num_factors(i: usize, primes: &Vec<usize>) -> usize {
	let mut count = 0;
	for prime in primes {
		if prime > &i { return count; }
		if i % prime == 0 { count += 1; }
	}
	count
}


fn main() {
	let target_factors = 4;
	let target_consec_count = 4;

	let mut primes = vec![2];

	let mut i = 2;
	let mut consec_count = 0;
	
	loop {
		i += 1;

		if is_prime(i) {
			primes.push(i);
			consec_count = 0;
			continue;
		}
	
		if num_factors(i, &primes) == target_factors {
			consec_count += 1;
			
			if consec_count == target_consec_count {
				// Reduce i to the first number in sequence
				i -= consec_count - 1;
				break;
			}
		} else {
			consec_count = 0;
		}
	}

    println!("First of the four consecutive numbers: {}", i);
}
