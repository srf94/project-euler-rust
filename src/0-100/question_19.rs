fn main() {
	let mut day = 2;
	let mut sum = 0;

	for i in 1..101 {
		println!("{}", i + 1900);

		// Jan
		if day == 6 { sum += 1; }
		day = (day + 31) % 7;

		// Feb
		if day == 6 { sum += 1; }
		if i % 4 == 0 { day = (day + 1) % 7; }

		// March to Dec
		let months = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
		for &month in months.iter() {
			if day == 6 { sum += 1; }
			day = (day + &month) % 7;
		}
	}

    println!("Total number of Sundays on 1st of month: {}", sum);
}
