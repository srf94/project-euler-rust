fn same_digits(a: usize, b: usize) -> bool {
	let mut chars_a: Vec<char> = a.to_string().chars().collect();
	chars_a.sort();

	let mut chars_b: Vec<char> = b.to_string().chars().collect();
	chars_b.sort();
	
	for i in 0..chars_a.len() {
		if chars_a[i] == chars_b[i] {
			continue;
		}
		return false;
	}
	true
}


fn main() {
	let mut i = 10;
	let mut ceiling = 100;

	'outer: loop {
		if 6 * i > ceiling {
			ceiling *= 10;
			i = ceiling / 10;
			continue;
		}

		for j in 2..7 {
			if !same_digits(i, i*j) {
				i += 1;
				continue 'outer;
			}
		}

		println!("Smallest number: {}", i);
		break;
	}
}
