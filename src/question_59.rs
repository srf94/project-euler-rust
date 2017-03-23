use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn read_input() -> Vec<i32> {
    // Create a path to the desired file
    let path = Path::new("../question_59_text.txt");
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

	let mut input_strings = input.split(",").collect::<Vec<&str>>();

	let mut input_ints = Vec::new();
	for string in input_strings {
		input_ints.push(string.parse::<i32>().unwrap());
	}
	input_ints
}


fn main() {
	let mut input_ints = read_input();

	// Input - numbers (bytes).
	// XOR these numbers with some key - eg aa -> bytes -> binary
	// Then these numbers should be in their original format
	// Now search for 'and' and other sensible words

    // From 97 to 122 inclusive (bytes of a to z)
    for ii in 97..123 {
    for jj in 97..123 {
    for kk in 97..123 {

    let mut key = vec![ii, jj, kk];

    let mut loc = 0;
	let mut ascii_vec = Vec::new();

    for &digit in &input_ints {
    	ascii_vec.push(digit as u8 ^ key[loc]);
    	loc = (loc + 1) % 3;
    }

    let mut sum = 0;
    for &ascii in &ascii_vec {
    	sum += ascii as usize;
    }
    // Once we have found the correct key:
    // println!("ASCII value sum: {:?}", sum);

    let mut final_str = String::from_utf8(ascii_vec).unwrap();
    if final_str.contains(&"and") && final_str.contains(&"the") {
    	// Print out possible candidates for the correct code here
    	// Manually verify the correct code by looking at the output
	    println!("{:?}", final_str);
	    println!("{:?}, {:?}, {:?}", ii, jj, kk);
	}

	}}}

	// After the search, we find that the following values work for the code:
    // let ii = 103;
    // let jj = 111;
    // let kk = 100;
    // Which translates to a passcode of: god
}
