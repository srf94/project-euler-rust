use std::collections::HashSet;

fn sum_proper_div(value: usize) -> usize {
	let search_max = (value as f32).sqrt() as usize + 1;

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


fn abundant_under_n(limit: usize) -> Vec<usize>{
	let mut abundant = Vec::new();

	for i in 2..limit {
		if i < sum_proper_div(i) { abundant.push(i); }
	}

	abundant
}


fn main() {
	let range = 28123;
	let abundant = abundant_under_n(range);

    let mut ab_sums = HashSet::new();
	for elem_i in &abundant {
		for elem_j in &abundant {
			ab_sums.insert(elem_i + elem_j);
		}
	}

	// If a number is not in ab_sums it fails the condition
	let mut sum = 0;
	for i in 1..(range + 1) {
		if !ab_sums.contains(&i) { sum += i; }
	}

	println!("Sum of all non-abundant-sum numbers: {}", sum);
}
