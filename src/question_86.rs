fn square_path(a: usize, b: usize) -> (bool, usize) {
	let n = a*a + b*b;
	if (n as f64) < (0 as f64) { 
		println!("{:?}", n);
	}

	let sqrt = (n as f64).sqrt() as usize;

	(sqrt*sqrt == n, n)
}


fn main() {
	let M = 1817;
	let mut M = 1;
	let mut count = 0;

	while count < 1000000 {

		// Only need to consider cuboids we have not yet seen.
		// These are all cuboids with at least one side of length M.
		let side_1 = M;
		for side_2 in 1..M+1 {
		for side_3 in side_2..M+1 {
			let mut shortest_path = (false, M*M*5);

			for path_tuple in [square_path(side_1 + side_2, side_3),
							   square_path(side_3 + side_1, side_2),
							   square_path(side_2 + side_3, side_1)].iter() {
				// If the length of the path is the shortest seen so far
				if path_tuple.1 < shortest_path.1 {
					// If the path is of integer length record 'true'. Else record 'false'.
					if path_tuple.0 { shortest_path = (true, path_tuple.1); }
					else { shortest_path = (false, path_tuple.1); }
				}
			}

			if shortest_path.0 {{ count += 1; }}

		}}

		M += 1;
	}

	println!("Least value of M: {:?}", M - 1);
}
