fn divisors_sum(value: i64) -> i64 {
	let search_max = (value as f32).sqrt() as i64 + 1;

	let mut sum = 1;
	for j in 2..search_max {
		if &value % j == 0 { 
			sum += j;
			sum += value / j;
		}
	}

	if (search_max - 1).pow(2) == value { sum -= search_max - 1; }
	sum
}


fn main() {
	let mut max_seqence_len = 0;
	let mut min_value = 0;

	'outer: for start_n in 1..1000000 {
		let mut n = start_n.clone();
		let mut smallest_n = start_n.clone();

		let mut sequence = vec![start_n];
		loop {
			n = divisors_sum(n);

			if n > 1000000 { continue 'outer; }

			if n < smallest_n { smallest_n = n; }

			if sequence.contains(&n) { 
				if n == start_n {
					break;
				} else {
					continue 'outer;
				}
			} else {
				sequence.push(n);
			}

		}

		if sequence.len() > max_seqence_len {
			max_seqence_len = sequence.len();
			min_value = smallest_n;
		}

	}

	println!("Minimum value: {:?}", min_value);
}
