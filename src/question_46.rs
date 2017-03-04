fn is_prime(target: usize) -> bool {
	let search_max = (target as f64).sqrt() as usize + 1;
	
	for i in 2..search_max { if target % i == 0 { return false; } }
	true
}


fn main() {
	let mut primes = vec![2];
	let mut two_squares = vec![2];
	
	let mut N = 0;
	loop {
		N += 1;
		let mut n = 2*N + 1;
		if is_prime(n) {
			primes.push(n);
			continue;
		}
		if n >= two_squares[two_squares.len()-1] { 
			let len = two_squares.len();
			two_squares.push(2 * (len+1) * (len+1));
		}
	
		let mut condition_holds = false;
		for prime in &primes {
			for two_square in &two_squares {
				if n == prime + two_square {
					condition_holds = true;
				}
				
			}
		}
		if !condition_holds {
			println!("Conjecture not true for: {}", n);
			break
		}
	}

}
