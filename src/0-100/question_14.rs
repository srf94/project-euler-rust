use std::collections::HashMap;

fn next_collatz_step(value: i64) -> i64 {
	if value % 2 == 0 { return value / 2; }
	3*value + 1
}


fn main() {

    let mut largest_value: i64 = 0;
    let mut largest_key: i64 = 0;

    for i in 1..1000000 {
    	if i % 1000 == 0 { 
    		println!("{}", i);
    	}

		let mut count = 0;
		let mut value = i;

		while value > 1 {
			value = next_collatz_step(value);
			count += 1;
		}
		
		if count > largest_value { 
			largest_value = count;
    		largest_key = i;
		}		
    }
	println!("Key: {}, Value: {}", largest_key, largest_value)
}
