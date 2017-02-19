fn main() {
	let mut sum_of_results = 0;

	for i in 2..1000000 {
		let mut power_sum: i32 = 0;
		for digit in (i as i32).to_string().as_bytes().iter() {
			power_sum += ((digit - 48) as i32).pow(5)
		}
		if i == power_sum { sum_of_results += i; }
	}

	println!("Sum of numbers: {}", sum_of_results);
}
