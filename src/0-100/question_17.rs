fn main() {
	// Sum of 1-9 inclusive
	// one + two + three + four + five + six + seven + eight + nine
	let units = 3 + 3 + 5 + 4 + 4 + 3 + 5 + 5 + 4;
	
	// Sum of 10-19 inclusive
	// ten + eleven + twelve + thirteen + fourteen + fifteen + sixteen + 
	// seventeen + eighteen + nineteen
	let teens = 3 + 6 + 6 + 8 + 8 + 7 + 7 + 9 + 8 + 8;
	
	// Sum of 20, 30, .., 90
	// twenty + thirty + forty + fifty + sixty + seventy + eighty + ninety
	let tens = 6 + 6 + 5 + 5 + 5 + 7 + 6 + 6;
	
	// Sum of 1-99 inclusive
	let double_digits = 9 * units + teens + 10 * tens;
	
	// Sum of 1-999 inclusive
	let hundred = 7;
	let and = 3;
	let triple_digits = 10 * double_digits + 100 * units + 900 * hundred + 891 * and;
		
	// Sum of 1-1000 inclusive
	let final_total = triple_digits + 3 + 8;
	
	println!("Final sum: {}", final_total)
}
