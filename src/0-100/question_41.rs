fn _get_perms(digits: Vec<String>) -> Vec<String> {
	if digits.len() == 1 { return digits; }

	let mut output = Vec::new();

	for i in 0..digits.len() {
		let ref base = digits[i];
		for perm in _get_perms([&digits[0..i], &digits[i+1..]].concat()) {
			output.push(format!("{}{}", base, perm));
		}
	}
	output
}


fn get_all_perms(num: &str) -> Vec<usize> {
	let digits: Vec<_> = num.chars().map(|d| d.to_digit(10).unwrap().to_string()).collect();

	let mut output = Vec::new();
	for str_digits in _get_perms(digits) {
		output.push( str_digits.parse::<usize>().unwrap() );
	}
	output
}


fn is_prime(num: usize) -> bool {
	let search_max = (num as f64).sqrt() as usize + 1;
	
	for i in 2..search_max {
		if num % i == 0 { return false; }
	}
	true
}


fn main() {
	let mut largest_prime = 0;
    let mut current_digits = "123456789";

	while largest_prime == 0 {
    	for number in get_all_perms(current_digits) {
    		if number < largest_prime { continue; }
	    	if is_prime(number) { largest_prime = number; }
	    }
	    
	    current_digits = &current_digits[..current_digits.len() - 1]
    }
    
    println!("Largest pandigital prime: {}", largest_prime)
}

