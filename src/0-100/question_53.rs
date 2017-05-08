use std::collections::HashMap;


fn factorise_int(int: usize) -> HashMap<usize, usize> {
	let mut factors = HashMap::new();
	
	let mut remainder = int;
	let mut i = 2;

	while remainder > 1 {
		if remainder % i == 0 {
			remainder = remainder/i;

			if factors.contains_key(&i) {
				*factors.get_mut(&i).unwrap() += 1;
			} else {
				factors.insert(i, 1);
			}
		} else { i += 1; }
	}

	factors
}


fn factorise_factorial(n: usize) -> HashMap<usize, usize>  {
	let mut factors = HashMap::new();

	for i in 2..n+1 {
		for (factor, count) in factorise_int(i) {
			if factors.contains_key(&factor) {
				*factors.get_mut(&factor).unwrap() += count;
			} else {
				factors.insert(i, count);
			}
		}
	}

	factors
}


fn bigger_than(n: usize, r: usize) -> bool {
	let n_fac = factorise_factorial(n);
	let mut r_fac = factorise_factorial(r);
	let n_r_fac = factorise_factorial(n-r);

	// Join the two denominator vectors
	for (factor, count) in n_r_fac {
		if r_fac.contains_key(&factor) {
			let var = r_fac.get(&factor).unwrap() + count;
			r_fac.insert(factor, var);
		} else {
			r_fac.insert(factor, count);
		}
	}

	let mut prod = 1;
	for (factor, count) in n_fac.iter() {

		if r_fac.contains_key(&factor) {
			let n_fac = n_fac.get(&factor).unwrap();
			let r_fac = r_fac.get(&factor).unwrap();

			if &n_fac > &r_fac {
				for _ in 0..(n_fac - r_fac) {
					prod = prod*factor;
					if prod > 1000000 { return true }
				}
			}
		} else {
			for _ in 0..*count {
				prod = prod*factor;
				if prod > 1000000 { return true }
			}
		}

	}

	false
}


fn main() {
	let mut count = 0;

	for n in 10..101 {
		for r in 1..n {
			if bigger_than(n, r) { count += 1; }
		}
	}

	println!("Number of nCr greater than 1,000,000: {}", count);
}
