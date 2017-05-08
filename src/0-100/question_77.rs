use std::collections::HashMap;


fn is_prime(int: isize) -> bool {
	let max = (int as f64).sqrt() as isize + 1;

	for i in 2..max {
		if int % i == 0 { return false; }
	}
	true
}


fn fnc(sum: isize, mut largest: isize, 
	mut saved_values: &mut HashMap<(isize, isize), isize>, 
	primes: &Vec<isize>) -> isize {

	if largest <= 1 { return 0; }
	if sum == 0 { return 1; }
	if sum < 0 { return 0; }

	// Round largest down to nearest prime
	if !primes.contains(&largest) {
		let mut ii = 0 as isize;
		for prime in primes {
			if prime <= &largest && prime > &ii {
				ii = *prime;
			}
		}
		largest = ii;
	}

	let mut part_1: isize;
	let mut part_2: isize;

	if saved_values.contains_key(&(sum, largest - 1)) {
		part_1 = saved_values.get(&(sum, largest - 1)).unwrap().clone();
	} else {
		part_1 = fnc(sum, largest - 1, &mut saved_values, &primes);
	}

	if saved_values.contains_key(&(sum - largest, largest)) {
		part_2 = saved_values.get(&(sum - largest, largest)).unwrap().clone();
	} else {
		part_2 = fnc(sum - largest, largest, &mut saved_values, &primes);
	}

	let result = part_1 + part_2;

	if !saved_values.contains_key(&(sum, largest)) {
		saved_values.insert((sum, largest), result);
	}

	result
}



fn main() {
	let mut primes = Vec::new();
	let mut n = 2;

	loop {
		if is_prime(n) {
			primes.push(n as isize);
		}

		let mut saved_values: HashMap<(isize, isize), isize> = HashMap::new();

	    let num_ways = fnc(n, n, &mut saved_values, &primes);
	    if num_ways >= 5000 {
	    	println!("First value that can be written in 5000 ways: {:?}", n);
	    	break;
	    }

		n += 1;
	}

}
