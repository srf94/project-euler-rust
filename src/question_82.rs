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


fn min_a_to_b(x: usize, a: usize, b: usize, data: &Vec<Vec<usize>>) -> usize {
    /* Find minimum route from data[x][y] to [x+1][z].
    Possible routes:
    [x][y] + [x][y+1] + [x][y+2] + ... + [x][z-1] + [x][z] + [x+1][z]
    [x][y] + [x][y+1] + [x][y+2] + ... + [x][z-1] + [x+1][z-1] + [x+1][z]
    ...
	[x][y] + [x+1][y] + [x+1][y+1] + ... + [x+1][z-1] + [x+1][z-1] + [x+1][z]
	*/

	let mut min_sum = 0;

	let mut range = a..b+1;
	let mut first_val = a;
	if a > b {
		range = b..a+1;
		first_val = b;
	}

	for cross_loc in range {
		let mut sum = 0;

		// Sum values in column x, crossing into column x+1 at jump: [x][jump] -> [x+1][jump]
		if a <= b {
			for i in a..cross_loc+1 { sum += data[x][i]; }
			for i in cross_loc..b+1 { sum += data[x+1][i]; }
		} else {
			for i in (cross_loc..a+1).rev() { sum += data[x][i]; }
			for i in (b..cross_loc+1).rev() { sum += data[x+1][i]; } 
		}

		// Deal with first case separetly to initialise min_sum correctly
		if cross_loc == first_val { min_sum = sum; }
		if sum < min_sum { min_sum = sum; }
	}

	min_sum
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

    // Need to flip the orientation of the data. [[0, 1], [2, 3]] -> [[0, 2], [1, 3]]
    let mut fliped: Vec<Vec<usize>> = Vec::new();
	for i in 0..square_size {
		let mut inner = Vec::new();
	    for vec_1 in &data {
	    	inner.push(vec_1[i]);
	    }
	    fliped.push(inner);
	}
	data = fliped;


    for col in 0..square_size-1 {
    	println!("Column {:?}/{:?}", col + 1, square_size - 1);

	    let mut replacement_col = Vec::new();

	    // Find the new value of data[col+1][b] for all b
	    for b in 0..square_size {
		    let mut min_val = 0;

		    // Find the minimum path from data[col][a] -> data[col+1][b] for all a.
		    // This minimim value will become the new value for data[col+1][b]
		    for a in 0..square_size {
		    	if a == 0 {
		    		min_val = min_a_to_b(col, a, b, &data);
		    		continue;
		    	}

				let val = min_a_to_b(col, a, b, &data);
				if val < min_val {
					min_val = val;
				}
		    }
		    replacement_col.push(min_val);
		}

		// Update data[col+1] with the new minimum values
		for (i, entry) in replacement_col.iter().enumerate() {
			data[col+1][i] = *entry;
		}
	}

	println!("Minimum path sum: {:?}", data[square_size-1].iter().min().unwrap());
}
