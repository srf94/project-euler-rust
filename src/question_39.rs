fn main() {
	let mut max_perimeter = 0;
	let mut max_perimeter_count = 0;

	for p in 10..1001 {
		let mut sum = 0;
		for b in 1..p {
			let top = p*p - b*p;
			let bottom = 2*p + 2*b;
			if top % bottom == 0 {
				let a = top / bottom;
				let c = p - a - b;
				
				if a == b || b == c || c == a { continue; }
				
				if a + b < c || c + a < b || b + c < a { continue; }
			
				sum += 1;
			}
		}
		if sum > max_perimeter_count {
			max_perimeter = p;
			max_perimeter_count = sum;
		}
	}

    println!("Perimeter that maximises number of solutions: {}", max_perimeter);
}
