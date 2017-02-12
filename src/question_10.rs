extern crate time;
use time::PreciseTime;

fn solve(max: usize) -> usize{
    let mut primes = vec![2];

	'outer: for i in 3..max {
		if i % 100000 == 0 {println!("{}", i);}
	
		'inner: for prime in &primes {
			if i % prime == 0 { continue 'outer; }
		}
		primes.push(i);
	}
	
	let mut sum = 0;
	for prime in primes {
		sum += prime;
	}
	
	sum
}

fn main() {
	let range = 2000000;

	let start = PreciseTime::now();
    println!("Sum of first {} primes: {}", range, solve(range));
	let end = PreciseTime::now();

	println!("Runtime: {} seconds.", start.to(end));
}