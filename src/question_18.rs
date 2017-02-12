fn main() {
	let input = "75
	95 64
	17 47 82
	18 35 87 10
	20 04 82 47 65
	19 01 23 75 03 34
	88 02 77 73 07 63 67
	99 65 04 28 06 16 70 92
	41 41 26 56 83 40 80 70 33
	41 48 72 33 47 32 37 16 94 29
	53 71 44 65 25 43 91 52 97 51 14
	70 11 33 28 77 73 17 78 39 68 17 57
	91 71 52 38 17 14 91 43 58 50 27 29 48
	63 66 04 68 89 53 67 30 73 16 69 87 40 31
	04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

	let input_split = input.split_whitespace();

	let mut triangle: Vec<i32> = input_split.map(|x|
		match i32::from_str_radix(x, 10) {
			Ok(v) => v,
			Err(e) => {println!("Bad input: {}", e); 0}
		}
	).collect();	
	
	let mut triangle_number = 3;
	let mut prev_row_start = 0;
	let mut cur_row_start = 1;
	let mut cur_row_end = 2;

	// Instead of looking at individual paths, we carry the largest value
	// possible 'down' the triangle, from the top to bottom row.
	while cur_row_start < triangle.len() {
		// Carry the top number down in the 1 and -1 edge cases
		triangle[cur_row_start] += triangle[prev_row_start];
		triangle[cur_row_end] += triangle[cur_row_start - 1];

		// Add the larger top number to the current row
		for i in 1..(cur_row_end - cur_row_start) {
			if triangle[prev_row_start + i - 1] >= triangle[prev_row_start + i] {
				triangle[cur_row_start + i] += triangle[prev_row_start + i - 1];
			} else {
				triangle[cur_row_start + i] += triangle[prev_row_start + i];
			}
		}
		prev_row_start = cur_row_start;
		cur_row_start = cur_row_end + 1;
		cur_row_end += triangle_number;
		triangle_number += 1;
	}

	let mut max_val = 0;
	for &item in triangle.iter() {
		if item > max_val { max_val = item; }
	}

	println!("Maximum value path: {}", max_val);
}





