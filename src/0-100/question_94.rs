fn integral_area(side: usize, base: usize) -> bool {
	// Assume side is odd - if even this will never work
	let height_squ = side*side - (base*base)/4;
	let area_squ = height_squ * base * base;
	let area_approx = (area_squ as f64).sqrt() as usize;

	area_squ == area_approx*area_approx
}



fn main() {
	let mut total_perimeter = 0;
	let mut side = 0;

	loop {
		side += 1;
		if 3*side - 1 > 1000000000 { break; }

		if side % 2 == 0 { continue }

		if integral_area(side, side + 1) {
			total_perimeter += 3 * side + 1
		}
		if integral_area(side, side - 1) {
			total_perimeter += 3 * side - 1
		}
	}

    println!("{:?}", total_perimeter);
}
