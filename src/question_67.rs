use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
    // Create a path to the desired file
    let path = Path::new("../question_67_text.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut input = String::new();
    match file.read_to_string(&mut input) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => println!("Read input successfully!"),
    }

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





