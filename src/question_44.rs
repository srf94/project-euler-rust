fn main() {
	let max_size = 10000;
	let mut pents = Vec::new();
	for n in 1..max_size {
		pents.push(n * (3*n - 1) / 2);
	}

	for i in 2100..max_size-1 {
		let pent_i = pents[i];
		for j in 0..i {
			let pent_j = pents[j];

			if !pents.contains(&(pent_i - pent_j)) { continue; }
			if !pents.contains(&(pent_i + pent_j)) { continue; }
    		println!("Difference: {}", pent_i - pent_j);
		}
		if i % 100 == 0 {
    		println!("Checkin: {}", i);
		}
	}
}
