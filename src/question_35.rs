fn get_all_rotations(num: usize) -> Vec<usize> {
	let digits = num.to_string();
	
	let mut rotations = Vec::new();
	for i in 0..digits.len() {
		rotations.push([&digits[i..], &digits[..i]].concat());
	}

	let mut output = Vec::new();
	for str_digits in &rotations {
		output.push( str_digits.parse::<usize>().unwrap() );
	}
	output
}


fn primes_under_n(max: usize) -> Vec<usize> {
	// Special cases of primes with digits 2 or 5
    let mut primes = vec![2, 5];

	'outer: for i in 3..max {
		// We only want to consider numbers with digits 1, 3, 7, 9
		let str_i = i.to_string();
		for digit in ["2", "4", "5", "6", "8", "0"].iter() {
			if str_i.contains(digit) { continue 'outer; }
		}
	
		if i % 100000 == 0 {println!("{}", i);}

    	let search_max = (i as f64).sqrt() as usize + 1;
		'inner: for j in 3..search_max {
			if i % j == 0 { continue 'outer; }
		}
		primes.push(i);
	}
	primes
}


fn main() {
	let primes = primes_under_n(1000000);
	
	let mut sum = 0;
	for &prime in &primes {
		let mut cont = true;
		for perm in get_all_rotations(prime) { 
			if !primes.contains(&perm) { 
			cont = false; break; }
		}
		
		if cont { sum += 1; }
	}
	
    println!("Number of circular primes: {}", sum);
}






