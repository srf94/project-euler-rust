fn factorial (int: u64) -> u64 {
	if int == 0 || int == 1 { return 1; }
	int * factorial(int - 1)
}


fn main() {
	let mut answer_sum = 0;
    
    // 2540160 = 7*factorial(9); the max possible curious number
    // 8 digit curious numbers are not possible as 8*factorial(9) has 7 digits
	for i in 3..2540161 {
    	let digits: Vec<_> = i.to_string().chars().map(|d| d.to_digit(10).unwrap() as u64).collect();
    	
    	let fac_sum = digits.iter().fold(0, |mut sum, &x| {sum += factorial(x); sum});
    	if fac_sum == i { answer_sum += i; }
    }
    println!("Sum of all curious numbers: {}", answer_sum);
}
