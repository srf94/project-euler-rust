fn is_prime(int: usize) -> bool {
	let max = (int as f64).sqrt() as usize + 1;

	for i in 2..max {
		if int % i == 0 { return false; }
	}
	true
}


fn primes_below_N(N: usize) -> Vec<f64> {
	let mut primes = vec![2 as f64];
	for i in 1..(N/2 + 1) {
		let n = 2*i + 1;
		if is_prime(n) { primes.push(n as f64); }
	}

	primes
}


fn totient(primes: &Vec<f64>, n: usize) -> usize {
	let mut n = n as f64;
	let mut totient = n;

	for &prime in primes {
		if prime > n { break; }
		if n % prime == 0 as f64 { 
			totient = totient * (1.0 - 1.0/prime); 
		}
	}

	totient as usize
}


fn are_permu(a: &usize, b: &usize) -> bool {
	let mut a_str = a.to_string();
	let mut b_str = b.to_string();

	if a_str.len() != b_str.len() { return false; }

	let mut a_bytes = a_str.into_bytes();
	let mut b_bytes = b_str.into_bytes();
	a_bytes.sort();
	b_bytes.sort();

	a_bytes == b_bytes
}


fn main() {
	let N = 10000000;
	let primes = primes_below_N(N);

	let mut max_n = 0;
	let mut max_ratio = 100 as f64;

	for n in 1000000..N {
		if n % 10000 == 0 { println!("{:?}",n ); }
		let tot = totient(&primes, n);

		if are_permu(&n, &tot) {
			let ratio = (n as f64) / (tot as f64);
			if ratio < max_ratio { 
				max_n = n;
				max_ratio = ratio; 
				println!("{:?}, {:?}", n, ratio);
			}
		}
	}

	println!("N: {:?}, ratio: {:?}", max_n, max_ratio);
}
