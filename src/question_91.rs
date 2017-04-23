fn squared(tup: (i64, i64)) -> i64 {
	tup.0 * tup.0 + tup.1 * tup.1
}


fn is_valid(p: (i64, i64), q: (i64, i64)) -> bool {
	let len_1 = squared(p);
	let len_2 = squared(q);
	let len_3 = squared((p.0 - q.0, p.1 - q.1));

	if len_1 > len_2 && len_1 > len_3 {
		return len_1 == len_2 + len_3
	} else if len_2 > len_1 && len_2 > len_3 {
		return len_2 == len_1 + len_3
	} else {
		return len_3 == len_1 + len_2
	}
}


fn main() {
	// This should be one greater than the actual length
	// So here 51 not 50
	let len_sides = 51;
	let mut triangle_count = 0;

	for loc_1 in 1..&len_sides*&len_sides {
		let py = loc_1/len_sides;
		let p = (loc_1 - py * &len_sides, py);

		for loc_2 in loc_1+1..&len_sides*&len_sides {
			let qy = loc_2/len_sides;
			let q = (loc_2 - qy * &len_sides, qy);

			if is_valid(p, q) { triangle_count += 1; }

		}
	}

	println!("Right-angled triangles found: {:?}", triangle_count);
}
