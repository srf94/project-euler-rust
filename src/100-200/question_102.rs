use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


#[derive(Eq, PartialEq, Debug)]
struct Point {
	x: i32,
	y: i32,
}


fn same_side(line_tup: (&Point, &Point), a: &Point) -> bool {
	let origin = Point { x: 0, y: 0 };

	let line_1 = line_tup.0;
	let line_2 = line_tup.1;

	// This method makes no sense if line_1 and line_2 are the same
	if line_1 == line_2 {
		panic!("Method 'same_side' was run with a line of length 0. 
			    Start/end point: ({}, {}) ", line_1.x, line_1.y)
	}

	// Deal with infinite gradient case first
	if line_1.x == line_2.x {
		let a_side = a.x < line_1.x;
		let o_side = origin.x < line_1.x;
		return a_side == o_side
	}

	// Calculate the gradient of the line
	// Use gradient to calculate value of line at point a and the origin
	let gradient = (line_2.y - line_1.y) as f64 / (line_2.x - line_1.x) as f64;
	let a_val = line_1.y as f64 + (a.x - line_1.x) as f64 * gradient;
	let o_val = line_1.y as f64 + (origin.x - line_1.x) as f64 * gradient;

	// Need point a and the origin to be on the same side of the line
	// Doesn't matter which side - just needs to be the same side
	let a_side = a_val < (a.y as f64);
	let o_side = o_val < (origin.y as f64);

	a_side == o_side
}

fn read_input() -> String {
    // Create a path to the desired file
    let path = Path::new("../question_102_text.txt");
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
    input
}



fn main() {
	let input = read_input();

	let mut triangle_sum = 0;
	for raw_triangle in input.lines() {
		let raw_points: Vec<i32> = raw_triangle.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

		let A = Point { x: raw_points[0], y: raw_points[1] };
		let B = Point { x: raw_points[2], y: raw_points[3] };
		let C = Point { x: raw_points[4], y: raw_points[5] };

		let contains_origin = same_side((&A, &B), &C) && 
		                      same_side((&C, &A), &B) && 
		                      same_side((&B, &C), &A);

		if contains_origin {
			triangle_sum += 1;
		}
	}

	println!("{:?} triangels contain the origin", triangle_sum);
}
