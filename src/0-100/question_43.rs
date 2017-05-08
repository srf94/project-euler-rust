fn _get_perms(digits: Vec<String>) -> Vec<String> {
	if digits.len() == 1 { return digits; }

	let mut output = Vec::new();

	for i in 0..digits.len() {
		let current_perms = _get_perms([&digits[0..i], &digits[i+1..]].concat());

		let ref base = digits[i];
		for perm in current_perms {
			output.push(format!("{}{}", base, perm));
		}
	}
	output
}


fn get_all_perms(num: &str) -> Vec<String> {
	let digits: Vec<_> = num.chars().map(|d| d.to_digit(10).unwrap().to_string()).collect();

	let mut output = Vec::new();
	for str_digits in _get_perms(digits) {
		output.push( str_digits );
	}
	output
}


fn satisfy_conditions(number: &str) -> bool {
	if &number[1..4].parse::<usize>().unwrap() % 2 != 0 { return false; }
	if &number[2..5].parse::<usize>().unwrap() % 3 != 0 { return false; }
	if &number[3..6].parse::<usize>().unwrap() % 5 != 0 { return false; }
	if &number[4..7].parse::<usize>().unwrap() % 7 != 0 { return false; }
	if &number[5..8].parse::<usize>().unwrap() % 11 != 0 { return false; }
	if &number[6..9].parse::<usize>().unwrap() % 13 != 0 { return false; }
	if &number[7..10].parse::<usize>().unwrap() % 17 != 0 { return false; }
	true
}


fn main() {
	let all_pandigital_perms = get_all_perms("0123456789");

	let mut sum = 0;
	for perm in all_pandigital_perms {
		if satisfy_conditions(&perm) { sum += perm.parse::<usize>().unwrap() }
	}
	
    println!("Sum of pandigital primes with property: {}", sum);
}
