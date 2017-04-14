use std::collections::HashMap;


fn is_prime(value: usize) -> bool {
    let search_max = (value as f64).sqrt() as usize + 1;

    for n in 2..search_max {
        if value % n == 0 { return false; }
    }

    true
}


fn primes_between_m_n_inclusive(m: usize, n: usize) -> Vec<String> {
	let mut primes = Vec::new();

	for i in m..n+1 {
		if is_prime(i) { primes.push(i.to_string()); }
	}

	primes
}


fn missing_digit_perms(start: usize, end: usize, depth: usize) -> Vec<Vec<usize>> {
	/* Recursively create vector of 'missing digits'
	EG: If we are considering primes of length 4, missing_digits would be:
	vec![ [0], [1], [2], [3], [0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3] ]
	*/
	let mut perms = Vec::new();

	for digit in start..end {
		perms.push(vec![digit]);

		if depth > 1 {
			for vec in missing_digit_perms(digit + 1, end, depth - 1) {
				perms.push([&vec[..], &vec![digit][..]].concat());
			}
		}
	}

	perms
}


fn find_families(missing_digits: &Vec<usize>, num_digits: usize, primes: &Vec<String>) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
	let mut digits_to_count = HashMap::new();
	let mut digits_to_min_prime: HashMap<usize, usize> = HashMap::new();

	'outer: for prime_str in primes {
		// We have some prime_str (eg "56003") and some missing digits (say [2, 3])

		// First, check that all the starred digits of print_str are the same.
		// This is true in our example, as digits 2 and 3 are both 0
		let mut starred_digit = "na";
		for digit_loc in 0..num_digits {
			if missing_digits.contains(&digit_loc) {

				let l = digit_loc as usize;
				let str_digit = &prime_str[l..l+1];

				if starred_digit == "na" {
					// starred_digit will now be a string of a digit, eg "0"
					starred_digit = str_digit;
				} else {
					if starred_digit != str_digit {
						// The prime MUST have matching starred digits.
						//If not, we continue to the next one.
						continue 'outer
					}
				}
			}
		}

		// Now remove the missing digits from the string (eg 56**3 -> 563)
		let mut starless_key_vec = Vec::new();
		for digit in 0..num_digits {
			if !missing_digits.contains(&digit) {
				let l = digit as usize;
				starless_key_vec.push(&prime_str[l..l+1]);
			}
		}
		let starless_key: usize = starless_key_vec.into_iter().collect::<String>().parse().unwrap();

		let prime_int: usize = prime_str.parse().unwrap();

		// Count the number of times we have this specifc starless_key for this set of missing_digits
		let count = digits_to_count.entry(starless_key).or_insert(0);
		 *count += 1;

		// Store the smallest prime that generates this starless_key
		if digits_to_min_prime.contains_key(&starless_key) { 
			let var = *digits_to_min_prime.get(&starless_key).unwrap();
			if prime_int < var {
				digits_to_min_prime.insert(starless_key, prime_int);
			}
		} else {
			digits_to_min_prime.insert(starless_key, prime_int);
		}
	}

	(digits_to_count, digits_to_min_prime)
}


fn main() {
	// Rough idea:
	// Get all primes of length n (eg =5)
	// Select some set of stars, say 1, 2
	// Create a hashmap of remaining chars: num times found across all primes length n
	// If we get >=8 we win!
	// Select minimum suitable prime (there may be more than 1 such family - if so pick min)

	let prime_family_len = 8;

	'outer: for num_digits in 2..9 {
		let power = (10 as usize).pow(num_digits - 1);
		let primes = primes_between_m_n_inclusive(power, power*10);

		// Initalise this to a value that is: a) Never prime b) Larger than all primes in 'primes'
		let mut min_prime = power*10;

		for missing_digits in missing_digit_perms(0, num_digits as usize, (num_digits-1) as usize) {
			let tuple = find_families(&missing_digits, num_digits as usize, &primes);
			let digits_to_count = tuple.0;
			let digits_to_min_prime = tuple.1;

			for (k, v) in &digits_to_count {
				if v >= &prime_family_len {
					// Get the minimum prime that satisfies the condition
					let min_prime_for_key = digits_to_min_prime.get(&k).unwrap();

					if min_prime_for_key < &min_prime {
						min_prime = *min_prime_for_key;
					}
				}
			}
		}

		// If we have found any suitable primes we can return a result and stop the program!
		if min_prime < power * 10 {
			println!("Smallest suitable prime: {:?}", min_prime);
			break 'outer
		}
	}
}
