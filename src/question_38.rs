fn pandigital(int: usize) -> usize {
	let mut n = 1;
	let mut string = "".to_string();

	while string.len() < 9 {
		let prod = n*int;
		let new_string = prod.to_string();
		string.push_str(&new_string);
    	n += 1;
	}
	
	if string.len() > 9 { return 0; }
	
	for digit in "123456789".chars() {
	    if !string.contains(digit) { return 0; }
	}

    string.parse::<usize>().unwrap()
}


fn main() {
	let mut max = 0;

	let iter_max = 100000;
	for int in 1..iter_max {
		let num = pandigital(int);
		if num > max { max = num; }
	}

    println!("Largest pandigital number that can be formed: {}", max);
}
