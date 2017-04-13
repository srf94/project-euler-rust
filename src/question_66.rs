fn is_square(x: usize) -> bool {
	let sqrt = (x as f64).sqrt() as usize;
	sqrt * sqrt == x
}


fn euclid_step(a: isize, b: isize) -> isize {
	if a % b == 0 { return b; }

	euclid_step(b, a % b)
}


fn gcd(a: isize, b: isize) -> isize {
	if a >= b {
		return euclid_step(a, b);
	} 
	euclid_step(b, a)
}


fn main() {
	let mut max_x = 0;
	let mut max_D = 0;

	for D in 2..1001 {
		if D % 50 == 0 { println!("{:?}", D); }

		// If D is square move on to the next one
		if is_square(D) {
			continue;
		}

		let mut y = 1;
		loop {
			let x_squ = 1 + D*y*y;

			// let sqrt = (x_squ as f64).sqrt() as usize;

			if is_square(x_squ) {
				let x = (x_squ as f64).sqrt() as usize;
				if x > max_x {
					println!("D: {}, x: {:?}, y: {}", D, x, y);
					max_x = x;
					max_D = D;
				}
				break;
			}
			y += 1;
		}
	}

	println!("D that produces largest x: {:?}", max_D);
}
