fn main() {

	let mut n = 0;
	let mut count = 0;
	let mut digits = Vec::new();
	
	let locations_wanted = [1, 10, 100, 1000, 10000, 100000, 1000000];
	
	while count <= 1000000 {
		n += 1;
		for i in 0..n.to_string().len() {
			count += 1;

			if locations_wanted.contains(&count) {
				// Push the ith digit of n
				digits.push(n.to_string()[i..i+1].parse::<usize>().unwrap());
			}
		}
	}
    
    let prod = digits.iter().fold(1, |mut prod, &x| {prod *= x; prod});
    println!("Product: {}", prod);
}
