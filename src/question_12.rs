fn number_factors (value: i64) -> i64 {
	let search_max = (value as f64).sqrt() as i64;

	let mut count = 2;
	for i in 2..(search_max + 1) {
		if value % i == 0 {
			count += 2;
		}
	}

	if value == search_max * search_max { count -= 1; }
	
	count
}

fn main() {
	let mut triangle = 1;
	let mut i = 1;
	while true {
		if number_factors(triangle) > 500 { break; }
		i += 1;
		triangle += i;
    }
	println!("{}", triangle);
}

