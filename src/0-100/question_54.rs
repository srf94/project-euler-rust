use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;

// Nicer way to setup hashset
macro_rules! set(
    { $($key:expr),+ } => {
        {
            let mut m = ::std::collections::HashSet::new();
            $(
                m.insert($key);
            )+
            m
        }
     };
);


/*
High Card: Highest value card.
One Pair: Two cards of the same value.
Two Pairs: Two different pairs.
Three of a Kind: Three cards of the same value.
Straight: All cards are consecutive values.
Flush: All cards of the same suit.
Full House: Three of a kind and a pair.
Four of a Kind: Four cards of the same value.
Straight Flush: All cards are consecutive values of same suit.
Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
*/


fn high_card(input: Vec<&str>) -> Vec<usize> {
	let mut values = Vec::new();
	for card in &input { 
		let mut value = &card[..1];
		if &value == &"A" { value = "14"; }
		if &value == &"K" { value = "13"; }
		if &value == &"Q" { value = "12"; }
		if &value == &"J" { value = "11"; }
		if &value == &"T" { value = "10"; }
		values.push(value.parse::<usize>().unwrap());
	}

	values.sort();
	values
}


fn one_pair(input: Vec<&str>) -> (bool, &usize) {
	let mut values = Vec::new();
	for card in &input { values.push(&card[..1].parse::<usize>().unwrap()); }
	
	values.sort();
	for i in 0..4 {
		if values[i] == values[i+1] { return (true, &values[i]); }
	}

	(false, &0)
}


fn two_pairs(input: Vec<&str>) -> (bool, &usize) {
	let mut suits = Vec::new();
	for card in &input { suits.push(&card[1..].parse::<usize>().unwrap()) }
	suits.sort();

	if suits[0] == suits[1] || suits[2] == suits[3] || suits[1] != suits[2] {
		if suits[1] > suits[2] { return (true, suits[1]); }
		return (true, &suits[2]);
	}
	
	if suits[0] == suits[1] || suits[3] == suits[4] || suits[1] != suits[3] {
		if suits[1] > suits[3] { return (true, suits[1]); }
		return (true, &suits[3]);
	}
	
	if suits[1] == suits[2] || suits[3] == suits[4] || suits[2] != suits[3] {
		if suits[2] > suits[3] { return (true, suits[2]); }
		return (true, &suits[3]);
	}

	(false, &0)
}


fn three_of_a_kind(input: Vec<&str>) -> (bool, &usize) {
	let mut suits = Vec::new();
	for card in &input { suits.push(&card[1..]) }
	suits.sort();

	if suits[0] == suits[2] || suits[1] == suits[3] || suits[2] == suits[4] {
		return (true, &suits[2].parse::<usize>().unwrap());
	}

	(false, &0)
}


fn straight(input: Vec<&str>) -> (bool, &usize) {
	let mut hand_values = HashSet::new();
	for card in &input { hand_values.insert(&card[..1]); }

	let order = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];
	for i in 0..order.len() - 4 {
		let mut values = HashSet::new();
		for &value in &order[i..i+5] { values.insert(value); }

		if values == hand_values { 
			return (true, &hand_values.iter().max().unwrap().parse::<usize>().unwrap()); 
		}
	}
	
	(false, &0)
}


fn flush(input: Vec<&str>) -> (bool, &usize) {
	let mut suits = HashSet::new();
	for card in &input { suits.insert(&card[1..].parse::<usize>().unwrap()); }
	if suits.len() > 1 { return (false, &0); }

	let mut hand_values = Vec::new();
	for card in &input { hand_values.push(&card[..1].parse::<usize>().unwrap()); }

	(true, hand_values.iter().max().unwrap())
}


fn full_house(input: Vec<&str>) -> (bool, &usize) {
	let mut hand_values = Vec::new();
	for card in &input { hand_values.push(&card[..1].parse::<usize>().unwrap()); }

	let mut suits = Vec::new();
	for card in &input { suits.push(&card[1..].parse::<usize>().unwrap()) }
	suits.sort();
	
	if suits[0] == suits[1] && suits[2] == suits[4] && suits[0] != suits[4] {
		return (true, hand_values.iter().max().unwrap());
	}
	
	if suits[0] == suits[2] && suits[3] == suits[4] && suits[0] != suits[4] {
		let var = hand_values.iter().max().unwrap();
		return (true, var);
	}

	(false, &0)
}


fn four_of_a_kind(input: Vec<&str>) -> (bool, &usize) {
	let mut suits = Vec::new();
	for card in &input { suits.push(&card[1..].parse::<usize>().unwrap()) }
	suits.sort();

	if suits[0] == suits[3] || suits[1] == suits[4] {
		return (true, &suits[2]);
	}

	(false, &0)
}


fn straight_flush(input: Vec<&str>) -> (bool, &usize) {
	let mut suits = HashSet::new();
	for card in &input { suits.insert(&card[1..]); }
	if suits.len() > 1 { return (false, &0); }

	let mut hand_values = HashSet::new();
	for card in &input { hand_values.insert(&card[..1]); }

	let order = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];
	for i in 0..order.len() - 4 {
		let mut values = HashSet::new();
		for &value in &order[i..i+5] { values.insert(value); }

		if values == hand_values { return (true, &0); }
	}
	
	(false, &0)
}


fn royal_flush(input: Vec<&str>) -> (bool, &usize) {
	let mut suits = HashSet::new();
	for card in &input { suits.insert(&card[1..]); }
	if suits.len() > 1 { return (false, &0); }

	let mut values = HashSet::new();
	for card in &input { values.insert(&card[..1]); }

	if values == set!("T", "J", "Q", "K", "A") {
		return (true, &0);
	}

	(false, &0)
}


fn one_or_two(input: &Vec<&str>, f: &Fn(Vec<&str>) -> (bool, &usize)) -> usize {
	let (p_1, p11) = f(input[..5].to_vec());
	let (p_2, p22) = f(input[5..].to_vec());
	
	if p_1 && !p_2 { return 1; }
	if !p_1 && p_2 { return 2; }
	if !p_1 && !p_2 { return 0; }

	let p_1_m = high_card(input[..5].to_vec());
	let p_2_m = high_card(input[5..].to_vec());

	for i in 0..5 {
		if p_1_m[4-i] > p_2_m[4-i] { return 1; }
		if p_1_m[4-i] < p_2_m[4-i] { return 2; }
	}

	println!("Should never get here with current rules!");
	0
}


fn controller(input: Vec<&str>) -> usize {

	let mut res = one_or_two(&input, &royal_flush);
	if res != 0 { return res; }

	let mut res = one_or_two(&input, &straight_flush);
	if res != 0 { return res; }

	let mut res = one_or_two(&input, &four_of_a_kind);
	if res != 0 { return res; }

	let mut res = one_or_two(&input, &full_house);
	if res != 0 { return res; }

	let mut res = one_or_two(&input, &flush);
	if res != 0 { return res; }

	let mut res = one_or_two(&input, &straight);
	if res != 0 { return res; }

	let mut res = one_or_two(&input, &three_of_a_kind);
	if res != 0 { return res; }

	let mut res = one_or_two(&input, &two_pairs);
	if res != 0 { return res; }

	let mut res = one_or_two(&input, &one_pair);
	if res != 0 { return res; }

	let p_1_m = high_card(input[..5].to_vec());
	let p_2_m = high_card(input[5..].to_vec());
	for i in 0..5 {
		if p_1_m[4-i] > p_2_m[4-i] { return 1; }
		if p_1_m[4-i] < p_2_m[4-i] { return 2; }
	}

	println!("Should never get here with current rules!");
	0
}


fn main() {
    // Create a path to the desired file
    let path = Path::new("../question_54_text.txt");
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

	let mut count_one = 0;
	let mut count_two = 0;
	let mut hands_vec = input.split("\n").collect::<Vec<&str>>();
	
	for hand in hands_vec {
		let mut hand_cards = hand.split(" ").collect::<Vec<&str>>();
		let result = controller(hand_cards);
		if result == 1 { count_one += 1; }
		if result == 2 { count_two += 1; }
	}
	println!("Player 1 wins: {:?}", count_one);
	println!("Player 2 wins: {:?}", count_two);
	
}
