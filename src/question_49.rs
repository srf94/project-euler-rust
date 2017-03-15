fn is_prime(value: usize) -> bool{
    let search_max = (value as f64).sqrt() as usize + 1;

    for n in 2..search_max {
        if value % n == 0 { return false; }
    }

    true
}


fn same_digits(a: usize, b: usize, c: usize,) -> bool {

	let mut chars_a: Vec<char> = a.to_string().chars().collect();
	chars_a.sort();

	let mut chars_b: Vec<char> = b.to_string().chars().collect();
	chars_b.sort();
	
	let mut chars_c: Vec<char> = c.to_string().chars().collect();
	chars_c.sort();

	for i in 0..4 {
		if chars_a[i] == chars_b[i] && chars_b[i] == chars_c[i] {
			continue;
		}
		return false;
	}

	true
}


fn main() {
	let mut primes = Vec::new();
	for i in 1000..10000 {
		if is_prime(i) { primes.push(i); }
	}
	
	let len_primes = primes.len();

	for i in 0..len_primes {
		let prime_i = primes[i];

		for j in i+1..len_primes {
			let prime_j = primes[j];
			let prime_k = 2*prime_j - prime_i;

			if primes.contains(&prime_k) && 
			same_digits(prime_i, prime_j, prime_k) {
				println!("Triple: {}, {}, {}", prime_i, prime_j, prime_k);
			}
		}
	}
}
