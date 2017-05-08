fn main() {
	let target: i64 = 2000000;
	let mut closest = target;
	let mut result = (0, 0);

	let mut m: i64 = 2;

 	while m < 1000 {

		let mut n: i64 = 2;

		let mut diff: i64 = -target;
		while diff < 0 {
			let m_axis_options = m*m - m*(m-1) / 2;
			let n_axis_options = n*n - n*(n-1) / 2;

			let total_options = m_axis_options * n_axis_options;
			diff = total_options - target;
			let dist = diff.abs();

			if dist < closest {
				result = (m, n);
				closest = dist;
			}

			n += 1;
		}

		m += 1;
	}
	println!("{:?}", closest);
	println!("{:?}", result);

}
