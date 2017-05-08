fn main() {
	/*
	Length is l1, sides are a, b, c
	a^2 + b^2 = c^2 = (l - a - b)^2
	a^2 + b^2 = l^2 + a^2 + b^2 + 2ab - 2al - 2bl
	0 = l^2 + 2(ab - al - bl + l^2) - 2l^2
	l^2 = 2(a - l)(b - l)
	a = l + l^2/2(b - l)

	WLOG, a <= b <= c, so we only need to search for sides up to a <= l/3
	*/

	let mut exactly_one_count = 0;

	// for l in 10..1500001 {
	for l in 3..1500001 {
		if l % 10000 == 0 {
			println!("{:?}", l);
		}

		let mut triples = Vec::new();
		'inner: for b in 1..(l/2) {
			// Check to ensure the division below can proceed without remainder
			// If not we must have a triangle with non-integer sides, and can skip
			if (l*l) % (2*(l-b)) != 0 {
				continue 'inner;
			}

			let a = l - (l*l) / (2*(l-b));
			let c = l - a - b;
			let sum = &a + &b + &c;
			let sq_a = &a * &a;
			let sq_b = &b * &b;
			let sq_c = &c * &c;

			let triangle = a+b>c && a+c>b && b+c>a;

			if sum == l && sq_a + sq_b == sq_c && triangle {
				let mut tuple = vec![a, b, c];
				tuple.sort();
				if !triples.contains(&tuple) {
					triples.push(tuple);
				}
			}
			if triples.len() > 1 { break 'inner }
		}

		if triples.len() == 1 { exactly_one_count += 1; }
	}

	println!("Lengths with exactly one right-angled triangle: {:?}", exactly_one_count);
}
