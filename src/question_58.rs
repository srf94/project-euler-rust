fn is_prime(int: usize) -> bool {
	let max = (int as f64).sqrt() as usize + 1;

	for i in 2..max {
		if int % i == 0 { return false; }
	}
	true
}


fn main() {

	let mut n = 1;
	let mut total_diag = 1;  // Centerpiece 1 is not prime
	let mut prime_diag = 0;

	let mut length = 1;
	loop {
		// Total side length = 2*length + 1
		// Step length = 2*length

		for _ in 0..4 {
			n += 2*length;
			// println!("{}", n);

			total_diag += 1;
			if is_prime(n) { prime_diag += 1; }
		}

		if total_diag > 10*prime_diag {
			break;
		}

		length += 1;
	}

    println!("Total diag: {}", total_diag);
    println!("Prime diag: {}", prime_diag);
    println!("Length: {}", 2*length + 1);
}
