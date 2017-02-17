use std::collections::HashSet;


fn repeating_length(N: usize) -> usize {
	let mut numerator = 10;
	let mut digit_num = Vec::new();
	
	while numerator != 0 {
		while numerator < N { numerator = numerator * 10; }
		let next_digit = numerator / &N;
		
		if digit_num.contains(&(next_digit, numerator)) {			
			let length = digit_num.len();
			for j in 0..length {
				if digit_num[j] == (next_digit, numerator) {
					return length - j;
				}
			}
		}
       digit_num.push((next_digit, numerator));
       numerator = numerator - next_digit * &N;
	}
	0
}


fn main() {
	let mut longest_cycle_int = 0;
	let mut longest_cycle_length = 0;

    for i in 1..1000 {
    	if repeating_length(i) > longest_cycle_length {
    		longest_cycle_int = i;
    		longest_cycle_length = repeating_length(i);
    	}
    }
    
	println!("Integer with longest recurring cycle: {}", longest_cycle_int)
}
