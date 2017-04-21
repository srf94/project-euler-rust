use std::collections::{HashMap, HashSet};

fn flip_9(mut v: &Vec<u32>) -> Vec<u32> {
	let mut v_clone = v.clone();
	for (i, n) in v.iter().enumerate() {
		if n == &9 { v_clone[i] = 6; }
	}
	v_clone
}


fn primehash(a: &u32, b: &u32) -> u32 {
	if a <= b {
		return (2 as u32).pow(*a) * (3 as u32).pow(*b);
	}
	(2 as u32).pow(*b) * (3 as u32).pow(*a)
}


fn combinations(elements: Vec<u32>, len: u32) -> Vec<Vec<u32>> {
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


fn main() {
	/* Change all 9's to 6's!
	Square numbers: 01, 04, 06, 16, 25, 36, 46, 64, 81

	To avoid messing around with MORE string permutations, store each 
	number in the form 2^a * 3^b
	*/
	let pairs = [(0, 1), (0, 4), (0, 6), 
				 (1, 6), (2, 5), (3, 6), 
				 (4, 6), (6, 4), (8, 1)];

	let mut good_arrangements = HashSet::new();

	for mut cube_1 in combinations(vec![0,1,2,3,4,5,6,7,8,9], 6) {
	for mut cube_2 in combinations(vec![0,1,2,3,4,5,6,7,8,9], 6) {

		let cube_1_flip = flip_9(&cube_1);
		let cube_2_flip = flip_9(&cube_2);

		let mut hashmap = HashMap::new();
		for pair in &pairs { hashmap.insert(primehash(&pair.0, &pair.1), false); }

		for c1 in &cube_1_flip {
		for c2 in &cube_2_flip {
			if hashmap.contains_key(&primehash(c1, c2)) {
				hashmap.insert(primehash(c1, c2), true);
			}
		}}

		let mut good_arrangement = true;
		for (k, v) in hashmap {
			if !v { good_arrangement = false; }
		}

		if good_arrangement { 
			cube_1.sort();
			cube_2.sort();
			good_arrangements.insert((cube_1.clone(), cube_2.clone()));
		}

	}}

	println!("Number of good arrangemens: {:?}", good_arrangements.len()/2);
}
