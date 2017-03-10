fn pandigital(a: &str, b: &str, c: &str) -> bool {
	if a.len() + b.len() + c.len() != 9 { return false; }

	for digit in "123456789".chars() {
		if !a.contains(digit) && !b.contains(digit) && !c.contains(digit) {
		    return false;
		}
	}

    true
}


fn main() {
	let mut products: Vec<usize> = Vec::new();

	for a in 1..10 {
	for b in 1111..10000 {
		let c = a * b;
		if pandigital(&a.to_string()[..], 
					  &b.to_string()[..],
					  &c.to_string()[..]) {
			if !products.contains(&c) {
				products.push(c);
			}
		}		
	}}

	for a in 1..100 {
	for b in 111..1000 {
		let c = a * b;
		if pandigital(&a.to_string()[..], 
					  &b.to_string()[..],
					  &c.to_string()[..]) {
			if !products.contains(&c) {
				products.push(c);
			}		
		}		
	}}
	
	let product_sum = products.iter().fold(0, |mut sum, &x| {sum += x; sum});
	println!("Sum of products: {:?}", product_sum);
}

