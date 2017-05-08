fn main() {
	let n = 1001;  // Where n=5 for a 5x5 spiral etc
	
	let n_spirals = (n - 1) / 2;
	let final_int = 1 + 4 * (n_spirals * (n_spirals + 1));
	
	let mut sum = 1;
	let mut count = 1;
	let mut side_length = 2;
	let mut side_count = 0;
	
	for i in 2..(final_int + 1) {
		// Are at the end of a side
		if count == 0 { 
			sum +=i;
			side_count += 1;
		}

		count += 1;
		count = count % side_length;

		// After 4 sides we need to work with the next side length
		if side_count == 4 {
			side_length += 2;
			side_count = 0;
		}
	}
	
	println!("{}", sum);
}
