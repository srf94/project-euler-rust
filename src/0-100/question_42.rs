use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Nicer way to setup hashmap
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);


fn word_value(word: &str) -> usize {
	let letter_map = map!{ 
		"A" => 1,
        "B" => 2,
        "C" => 3,
        "D" => 4,
        "E" => 5,
        "F" => 6,
        "G" => 7,
        "H" => 8,
        "I" => 9,
        "J" => 10,
        "K" => 11,
        "L" => 12,
        "M" => 13,
        "N" => 14,
        "O" => 15,
        "P" => 16,
        "Q" => 17,
        "R" => 18,
        "S" => 19,
        "T" => 20,
        "U" => 21,
        "V" => 22,
        "W" => 23,
        "X" => 24,
        "Y" => 25,
        "Z" => 26
	};

	let mut sum = 0;
	let chopped_word = &word[1..&word.len() - 1];

	for c in chopped_word.chars() {
		if !letter_map.contains_key::<str>(&c.to_string()) { continue; }
		
		let value = letter_map.get::<str>(&c.to_string()).unwrap();
		sum += *value as usize;
	}

	sum
}


fn get_triangles(split: &Vec<&str>) -> Vec<usize> {
	let mut max_word_len = 0;
	for word in split {
		if word.len() > max_word_len { max_word_len = word.len(); }
	}
	let max_word_value = max_word_len * 26;

	let mut n = 1;
	let mut triangles = Vec::new();
	while n*(n+1) / 2 <= 416 {
		triangles.push( n*(n+1) / 2 );
		n += 1;
	}

	triangles
}


fn main() {
    // Create a path to the desired file
    let path = Path::new("../question_42_text.txt");
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

	let mut split = input.split(",").collect::<Vec<&str>>();

	// Get all triangle numbers the words could possible match
	let triangles = get_triangles(&split);

	let mut final_sum = 0;
	for word in split {
		if triangles.contains(&word_value(word)) { final_sum += 1; }
	}
	
	println!("Number of triangle words: {}", final_sum);
}
