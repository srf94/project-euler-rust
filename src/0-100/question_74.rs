fn factorial(n: usize) -> usize {
	if n == 0 { return 1; }
	return n * factorial(n-1);
}


fn sum_digit_factorial(n: usize) -> usize {
	let digits: Vec<_> = n.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

	let mut sum = 0;
	for digit in digits { sum += factorial(digit as usize); }

	sum
}


fn non_repeating_terms(n: usize) -> usize {
	let mut chain = vec![n];
	let mut next = n;

	loop {
		next = sum_digit_factorial(next);
		if chain.contains(&next) {
			return chain.len();
		} 
		else { chain.push(next); }
	}
}


fn main() {

	let mut sixty_count = 0;
	for n in 1..1000000 {
		if non_repeating_terms(n) == 60 {
			sixty_count += 1;
		}
	}

	println!("{:?}", sixty_count);
}
