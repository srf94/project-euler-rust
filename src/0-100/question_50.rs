fn is_prime(num: usize) -> bool {
	let search_max = (num as f64).sqrt() as usize + 1;
	
	for i in 2..search_max {
		if num % i == 0 { return false; }
	}
	true
}


fn sum_sequence(primes: &Vec<usize>, sequence_len: usize) -> usize{
	let prime_len = primes.len();
	let max_value = 1000000 / sequence_len;
	
	for (count, &prime) in primes.iter().enumerate() {
		if prime > 1000000 / sequence_len {
			let mut max_value = count;
			break;
		}
	}
	
	let mut sum = 0;
	for &j in &primes[0..sequence_len] {
		sum += j;
	}
	
	for i in 0..prime_len - sequence_len {
		if i > max_value { return 0; }
	
		if primes.contains(&sum) { return sum }

		sum -= primes[i];
		sum += primes[i+sequence_len];
	}

	return 0
}


fn main() {
	let mut primes = vec![2];
	for i in 3..1000000 {
		if is_prime(i) { primes.push(i); }
	}

	let mut largest_sum = 0;
	for sequence_len in 532..999 {

		let result = sum_sequence(&primes, sequence_len);
		if result > largest_sum {
			largest_sum = result;
		}
	}
	
	println!("Largest consecutive prime sum: {:?}", largest_sum);
}
