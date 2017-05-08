use std::cmp;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn import_file() -> String {
    // Create a path to the desired file
    let path = Path::new("../question_81_text.txt");
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
    let input = import_file();

    // Parse data into Vector of Vectors (2-D array)
    let line_iterator = input.lines();
    let mut data: Vec<Vec<usize>> = Vec::new();
    for line in line_iterator {
    	data.push(line.split(",").map(|s| s.parse::<usize>().unwrap()).collect());
    }
    let square_size = data.len();

    /* Add the minimum value seen so far to the value in each square.
    Work down the square from to-left to bottom right in the following pattern:
    0  2  5  9  14
    1  4  8  13 18
    3  7  12 17 22
    6  11 16 21 24
    10 15 20 23 25
    */
    for cur_pos_sum in 1..2*square_size {
	    for x in 0..cur_pos_sum + 1 {
	    	let y = cur_pos_sum - x;
	    	if x >= square_size || y >= square_size { continue; }

	    	let mut new_val = data[x][y];

	    	if x == 0 {
	    		new_val += data[x][y - 1];
	    	} else if y == 0 {
	    		new_val += data[x - 1][y];
	    	} else {
	    		new_val += cmp::min(data[x][y - 1], data[x - 1][y]);
	    	}

	    	data[x][y] = new_val;
	    }
	}

	println!("Minimum path sum: {:?}", data[square_size - 1][square_size - 1]);
}
