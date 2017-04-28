use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn read_input() -> String {
    // Create a path to the desired file
    let path = Path::new("../question_99_text.txt");
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
	/*
	For positive integers a,b,c,d the following are equal:
	  a ** b > c ** d
	  b * log(a) > d * log(c)

	This allows us to compare values easily
	*/
	let mut max_val_seen = 0 as f64;
	let mut max_line_num = 0;

	let input = read_input();

	for (line, pair) in input.lines().enumerate() {
		let a_b: Vec<&str> = pair.split(",").collect();
		let a: f64 = a_b[0].parse().unwrap();
		let b: f64 = a_b[1].parse().unwrap();

		let val = b * a.log(10 as f64);
		if val > max_val_seen {
			max_val_seen = val;
			max_line_num = line;
		}
	}

	println!("Max line: {:?}, Max value of b*log(a): {:?}", max_line_num, max_val_seen);
}
