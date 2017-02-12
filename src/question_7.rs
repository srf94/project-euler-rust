extern crate time;
use time::PreciseTime;

fn is_prime_odd(value: i64) -> bool {
	// Input is assumed to be odd
    let search_max = (value as f64).sqrt() as i64 + 1;

    for n in 1..search_max / 2 {
        if value % (n * 2 + 1) == 0 {
            return false;
        }
    }
    true
}



fn solve(nth: i64) -> i64{
	let mut test_val = 1;
	let mut num_primes = 1;
	
	while num_primes < nth {
		test_val += 2;
		if is_prime_odd(test_val) { num_primes += 1; }
	}
    test_val
}


fn main() {
	let start = PreciseTime::now();
    println!("The 10,001st prime is {}", solve(10001));
	let end = PreciseTime::now();

	println!("Runtime: {} seconds.", start.to(end));
}
