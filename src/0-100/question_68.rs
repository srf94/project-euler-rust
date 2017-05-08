fn all_permutations(vec: Vec<usize>) -> Vec<Vec<usize>> {
	if vec.len() == 1 { return vec![vec.to_vec()]; }
  
	let mut permus = Vec::new();

	for i in 0..vec.len() {
		let first = &vec[i..i+1].to_vec();
		let remaining_digits = [&vec[..i], &vec[i+1..]].concat();

		let inner_perms = all_permutations(remaining_digits);
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

	let joined = format!("{}{}{}{}{}", substrings[0].to_string(), 
								   	   substrings[1].to_string(), 
									   substrings[2].to_string(), 
									   substrings[3].to_string(), 
									   substrings[4].to_string()).parse::<usize>().unwrap();

	joined
}


fn main() {
	// Inner numbers: 1-5
	// Outer numbers: 6-10

	let mut max_num = 0;
	let inner = vec![1, 2, 3, 4, 5];
	let outer = vec![6, 7, 8, 9, 10];

	let all_inner = all_permutations(inner);
	let all_outer = all_permutations(outer);

	for inner in &all_inner {
		'return_here: for outer in &all_outer {
			let target = outer[0] + inner[0] + inner[1];
			for i in 1..5 {
				if target != outer[i] + inner[i] + inner[(i + 1) % 5] {
					continue 'return_here;
				}
			}

			let joined_max = create_string(inner.to_vec(), outer.to_vec());
			if joined_max > max_num {
				max_num = joined_max;
			}
		}
	}

	println!("Maxiumum 16-digit string: {:?}", max_num);
}
