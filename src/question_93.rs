use std::collections::HashSet;

fn get_perms_i32(digits: Vec<i32>) -> Vec<Vec<i32>> {
	if digits.len() == 1 { return vec![digits]; }

	let mut output = Vec::new();
	for i in 0..digits.len() {

		let ref base = digits[i];
		for mut perm in get_perms_i32([&digits[0..i], &digits[i+1..]].concat()) {
			perm.push(*base);
			output.push(perm);
		}
	}
	output
}


fn get_perms_f64(digits: Vec<f64>) -> Vec<Vec<f64>> {
	if digits.len() == 1 { return vec![digits]; }

	let mut output = Vec::new();
	for i in 0..digits.len() {

		let ref base = digits[i];
		for mut perm in get_perms_f64([&digits[0..i], &digits[i+1..]].concat()) {
			perm.push(*base);
			output.push(perm);
		}
	}
	output
}


fn combinations(elements: Vec<i32>, len: i32) -> Vec<Vec<i32>> {
	let mut to_return = Vec::new();
	for (i, elem) in elements.iter().enumerate() {
		if len == 1 { 
			to_return.push(vec![*elem]);
			continue;
		}

		for mut v in combinations(elements[i+1..].to_vec(), len-1) {
			v.push(*elem);
			to_return.push(v);
		}

	}
	to_return
}


fn form_e(c: f64, d: f64) -> Vec<f64> {
	vec![c+d, c-d, d-c, c*d, c/d]
}


fn combine_2(a: f64, b: f64) -> Vec<f64> {
	vec![a+b, a-b, a*b, a/b]
}


fn combine_3(a: f64, b: f64, e: f64) -> Vec<f64> {
	let mut to_ret = Vec::new();

	for i in combine_2(b, e) {
		to_ret.push(a + i);
		to_ret.push(a - i);
		to_ret.push(a * i);
		to_ret.push(a / i);
	}

	for i in combine_2(e, b) {
		to_ret.push(a + i);
		to_ret.push(a - i);
		to_ret.push(a * i);
		to_ret.push(a / i);
	}

	for i in combine_2(a, b) {
		to_ret.push(i + e);
		to_ret.push(i - e);
		to_ret.push(i * e);
		to_ret.push(i / e);
	}

	to_ret
}


fn main() {
	/*
	Form e = c+d or c-d or c*d or c/d. 
	Now we only have three numbers to worry about!
	
	Need to consider the following groupings with all possible operators
	a (b e)
	a (e b)
	(a e) b
	*/
	let mut max_num_formed = 0;
	let mut best_abcd = vec![];

	for mut abcd in combinations(vec![0,1,2,3,4,5,6,7,8,9], 4) {

		let mut num_formed = HashSet::new();
		let abcd_ = abcd.clone();

		for abcd_tup in get_perms_i32(abcd) {
			let a = abcd_tup[0] as f64;
			let b = abcd_tup[1] as f64;
			let c = abcd_tup[2] as f64;
			let d = abcd_tup[3] as f64;

			for e in form_e(c, d) {
				for triple in get_perms_f64(vec![a, b, e]) {
					for num in combine_3(triple[0], triple[1], triple[2]) {

						// Check we have a postive integer
						if ((num as i32) as f64 == num) && (num > 0 as f64) {
							num_formed.insert(num as i32);
						}
					}
				}
			}
		}

		let mut i = 1;
		loop {
			if !num_formed.contains(&i) { break; }
			i += 1;
		}

		if i > max_num_formed {
			max_num_formed = i;
			best_abcd = abcd_;
		}

	}

	best_abcd.sort();
	println!("Best abcd: {:?}", best_abcd);
}
