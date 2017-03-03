fn main() {

	let max_size = 1000;
	let mut pents = Vec::new();
	for n in 1..max_size {
		pents.push( n*(3*n - 1) / 2);
	}

	for i in 0..max_size-1 {
		for j in 0..i {
			if !pents.contains(&(pents[i] - pents[j])) { continue; }
			if !pents.contains(&(pents[i] + pents[j])) { continue; }
    		println!("{}, {}", pents[i], pents[j]);
		}
		if i % 100 == 0 {
    		println!("Checkin: {}", i);
		}
	}

}
