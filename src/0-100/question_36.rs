use std::num;


fn is_palindromic(string: &str) -> bool {
	string == string.chars().rev().collect::<String>()
}


fn main() {
	let mut sum = 0;

	for num in 1..1000000 {
		let base_10 = &num.to_string();
    	let n: usize = usize::from_str_radix(base_10, 10).unwrap();
		let base_2 = format!("{:b}", n);
		
		if is_palindromic(&base_10) && is_palindromic(&base_2) {
			println!("{}", base_10);
			sum += n;
		}
    }
    
    println!("Sum of palindromes under 1 million: {}", sum);
}
