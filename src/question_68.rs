fn all_permutations(vec: Vec<usize>) -> Vec<Vec<usize>> {
	if vec.len() == 1 { return vec![vec.to_vec()]; }
  
	let mut permus = Vec::new();

	for i in 0..vec.len() {
		let first = &vec[i..i+1].to_vec();
		let remaining_digits = [&vec[..i], &vec[i+1..]].concat();

		let mut inner_perms = all_permutations(remaining_digits);
		for perm in inner_perms {
			 let combined = [&first[..], &perm[..]].concat();
			 permus.push(combined);
		}
	}

	return permus
}


fn create_string(inner: Vec<usize>, outer: Vec<usize>) -> usize {
	// let mut substrings: Vec<String> = Vec::new();
	let mut substrings: Vec<usize> = Vec::new();
	let mut seen_6 = false;

	for i in 0..10 {
		if outer[i % 5] != 6 && !seen_6 { continue; }
		seen_6 = true;
		let substring = format!("{}{}{}", &outer[i % 5].to_string(), 
										  &inner[i % 5].to_string(), 
										  &inner[(i + 1) % 5].to_string());

		substrings.push(substring.parse::<usize>().unwrap());
		if substrings.len() == 5 { break; }
	}

	let digit_perms = all_permutations(substrings);

	let mut max_number = 0;
	for perm in digit_perms {
		let joined = format!("{}{}{}{}{}", perm[0].to_string(), 
							  perm[1].to_string(), 
							  perm[2].to_string(), 
							  perm[3].to_string(), 
							  perm[4].to_string()).parse::<usize>().unwrap();
		if joined > max_number {
			max_number = joined;
		}
	}

	max_number
}


fn main() {
	// Inner numbers: 1-5
	// Outer numbers: 6-10

	let mut max_num = 0;
	let target = 16;
	let inner = vec![1, 2, 3, 4, 5];
	let outer = vec![6, 7, 8, 9, 10];

	let all_inner = all_permutations(inner);
	let all_outer = all_permutations(outer);

	for inner in &all_inner {
		for outer in &all_outer {
			for i in 0..5 {
				if target != outer[i] + inner[i] + inner[(i + 1) % 5] {
					break;
				}

				let joined_max = create_string(inner.to_vec(), outer.to_vec());
				if joined_max > max_num {
					max_num = joined_max;
				}
			}

		}
	}

	println!("{:?}", max_num);
}
