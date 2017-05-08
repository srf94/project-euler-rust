fn main() {
	let mut output = (0, 0);
	let mut min_diff = 1 as f64;

	for d in 9..1000000 {
		let mut i = (3*d) / 7;

		// Need to adjust due to rounding + perfect division issues 
		if 7*i >= 3*d { i -= 1; }

		let diff = ((3*d - 7*i) as f64) / ((7*d) as f64);

		if diff < min_diff {
			min_diff = diff;
			output = (i, d);
		}
	}

	println!("Fraction: {:?}/{:?}", output.0, output.1);
}

