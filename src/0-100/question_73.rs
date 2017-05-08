fn euclid_step(a: isize, b: isize) -> isize {
	if a % b == 0 { return b; }

	euclid_step(b, a % b)
}


fn hcf(a: isize, b: isize) -> isize {
	if a >= b {
		return euclid_step(a, b);
	} 
	euclid_step(b, a)
}


fn main() {

	let min = (1, 3);
	let max = (1, 2);
	let mut count = 0;

	for d in 4..12001 {

		let mut i = 0;  // TODO: Narrow range
		let mut add = false;
		loop {
			i += 1;

			if !add {
				if 3*i > d { add = true; }
			} else {
				if 2*i >= d { break; }
			}

			if add && hcf(i, d) == 1 { count += 1; }
		}
	}

    println!("Total fractions: {:?}", count);
}
