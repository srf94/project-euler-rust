use std::cmp;

fn is_prime(int: usize) -> bool {
	let max = (int as f64).sqrt() as usize + 1;

	for i in 2..max {
		if int % i == 0 { return false; }
	}
	true
}


fn primes_under_max(max: usize) -> Vec<usize> {
	let mut primes = vec![2];

	for i in 3..max {
		if is_prime(i) { primes.push(i); }
	}

	primes
}



fn primes_paired(primes: &Vec<usize>, prime_1: &usize, prime_2: &usize) -> bool {
	let joined_str = format!("{}{}", prime_1.to_string(), prime_2.to_string());
	let joined_int = joined_str.parse::<usize>().unwrap();

	if joined_int < 100000 { if !primes.contains(&joined_int) { return false; } }
	else { if !is_prime(joined_int) { return false; } }

	let joined_str = format!("{}{}", prime_2.to_string(), prime_1.to_string());
	let joined_int = joined_str.parse::<usize>().unwrap();

	if joined_int < 100000 { if !primes.contains(&joined_int) { return false; } }
	else { if !is_prime(joined_int) { return false; } }
	
	true
}


fn prime_tuples(primes: &Vec<usize>, max: usize) -> Vec<(&usize, &usize)> {

	// Need to keep prime_1, prime_2 under max to ensure prime_1 + prime_2 < upper_bound(primes)
	// Know primes is increasing in size, so once prime > max we can break the iteration
	
	let mut prime_tuples = Vec::new();

	'outer: for prime_1 in primes {
		if prime_1 > &max { break 'outer; }

		'inner: for prime_2 in primes {
			if prime_2 < prime_1 { continue; }
			if prime_2 > &max { break 'inner; }

			if primes_paired(primes, prime_1, prime_2) {
				prime_tuples.push((prime_1, prime_2));
			}

			// if !primes.contains(&concat(prime_1, prime_2)) { continue; }
			// if !primes.contains(&concat(prime_2, prime_1)) { continue; }
			// prime_tuples.push((prime_1, prime_2));
			// println!("{:?}, {:?}", prime_1, prime_2);
		}
	}

	prime_tuples
}


fn main() {
	let max = 10000;
	let max_prime_size = 100000;

	// First, get a list of primes less than the max
	let primes = primes_under_max(max_prime_size);

	// Create a list of tuples - sorted (smaller, larger) - that concat to form a new prime
	let prime_tuples = prime_tuples(&primes, max);

	// Create 3-cycles!
	let mut three_cycles = Vec::new();

	for tuple_1 in &prime_tuples {
	for tuple_2 in &prime_tuples {
		if tuple_2.0 < tuple_1.0 {
			continue;
		} else if tuple_2.0 == tuple_1.0 {
			// Both (3, x)
			let min = cmp::min(tuple_1.1, tuple_2.1);
			let max = cmp::max(tuple_1.1, tuple_2.1);
			let new = (min, max);
			if prime_tuples.contains(&new) {
				three_cycles.push((tuple_1.0, min, max));
			}
		} else {
			break;
		}
	}}

	three_cycles.sort();
	three_cycles.dedup();


	// Create 4-cycles
	let mut four_cycles = Vec::new();

	for triple_1 in &three_cycles {
	for triple_2 in &three_cycles {
		if triple_1.0 != triple_2.0 || triple_1.1 != triple_2.1 { continue; }

		let i = triple_1.2;
		let j = triple_2.2;
		if primes_paired(&primes, &i, &j) {
			let min = cmp::min(i, j);
			let max = cmp::max(i, j);
			let tup = (triple_1.0, triple_1.1, min, max);
			four_cycles.push(tup);
		} 
	}}

	four_cycles.sort();
	four_cycles.dedup();


	// Create 5-cycles
	let mut five_cycles = Vec::new();

	for quad_1 in &four_cycles {
	for quad_2 in &four_cycles {
		if quad_1.0 != quad_2.0 || quad_1.1 != quad_2.1 || quad_1.2 != quad_2.2 { continue; }

		let i = quad_1.3;
		let j = quad_2.3;
		if primes_paired(&primes, &i, &j) {
			let min = cmp::min(i, j);
			let max = cmp::max(i, j);
			let tup = (quad_1.0, quad_1.1, quad_1.2, min, max);
			five_cycles.push(tup);
		} 
	}}

	five_cycles.sort();
	five_cycles.dedup();

    println!("Five cycles: {:?}", five_cycles);
}







