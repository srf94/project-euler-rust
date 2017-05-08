fn is_prime(value: isize) -> bool {
    let search_max = (value as f64).sqrt() as isize;
    let mut is_prime = true;

    for n in 2..(search_max + 1) {
        if value % n == 0 {
            is_prime = false;
            break
        }
    }
    is_prime
}


fn primes_under_n(n: isize) -> Vec<isize> {
    let mut primes = vec![2];

	'outer: for i in 3..n {	
		'inner: for prime in &primes {
			if i % prime == 0 { continue 'outer; }
			if prime * prime > i { break 'inner; }
		}
		primes.push(i);
	}
	primes
}


fn get_prime_seq_len(a: isize, b: isize, stored_primes: &Vec<isize>) -> isize {
	// First check we satisfy n=1, 2 conditions
	if !stored_primes.contains(&(1 + a + b)) { return 0; }
	if !stored_primes.contains(&(4 + 2*a + b)) { return 0; }

	let mut n = 3;
	loop {
		let quad = n*n + a*n + b;
		if quad <= 1000 { if !stored_primes.contains(&quad) {break;} }
		else { if !is_prime(quad) {break;} }
		n += 1;
	}

	n
}


fn main() {
	let max_abs_a: isize = 1000;
	let max_abs_b: isize = 1000;

	let mut max_length = 0;
	let mut max_ab_prod = 0;
	let stored_primes = primes_under_n(max_abs_b + 1);

	for a in (-max_abs_a + 1)..max_abs_a {
		for &b in &stored_primes {
			// Positive b case
			let seq_len = get_prime_seq_len(a, b, &stored_primes);
			if seq_len > max_length {
				max_length = seq_len;
				max_ab_prod = a * b;
			}

			// Negative b case
			let seq_len = get_prime_seq_len(a, -b, &stored_primes);
			if seq_len > max_length {
				max_length = seq_len;
				max_ab_prod = a * b;
			}
		}
	}

    println!("Product of coefficents for maximum (a, b) pair: {}", max_ab_prod);
}
