use std::collections::{HashMap, HashSet};

fn products_of_n(n: usize, min_val: usize) -> Vec<Vec<usize>> {
	if n == 1 {
		return vec![vec![]];
	}

	let mut to_return = Vec::new();
	for i in min_val..n+1 {
		if n % i == 0 {
			for mut sub_result in products_of_n(n/i, i) {
				sub_result.push(i);
				to_return.push(sub_result);
			}
		}
	}
	to_return
}


fn main() {
	let mut min_ps_numbers = HashMap::new();

	let mut n = 2;
	while min_ps_numbers.len() < 20000 {

		for product_vec in products_of_n(n, 2) {
			let num_ones = n - product_vec.iter().sum::<usize>();
			let set_len = num_ones + product_vec.len();
			if !min_ps_numbers.contains_key(&set_len) {
				min_ps_numbers.insert(set_len, n);
			}
		}
		n += 1;
	}

	let mut ps_numbers_set = HashSet::new();
	for i in 2..12001 {
		let val = *min_ps_numbers.get(&i).unwrap();
		ps_numbers_set.insert(val);
	}

	println!("Sum of minimal product-sum numbers: {:?}", ps_numbers_set.iter().sum::<usize>());
}
