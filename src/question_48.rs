fn last_10_digits_power(value: i64) -> i64 {
	let mut power = value;
	for _ in 1..value {
		power = (power * value) % 10000000000;
	}
	power
}


fn main() {
	let mut ten_digit_sum: i64 = 0;
	
	for i in 1..1001 {
		ten_digit_sum += last_10_digits_power(i);
		ten_digit_sum = ten_digit_sum % 10000000000;
	}

    println!("Last ten digit sum: {}", ten_digit_sum);
}
