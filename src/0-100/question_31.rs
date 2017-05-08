const COINS: [usize; 8] = [200, 100, 50, 20, 10, 5, 2, 1];


fn n_ways_rec (target_sum: usize, position: usize) -> usize {
	if position + 1 == COINS.len() { return 1; }

	let mut n_ways = 0;
	let mut pass_on_sum = target_sum;

	loop {
		n_ways += n_ways_rec(pass_on_sum, position + 1);
		
		if pass_on_sum < COINS[position] { break; }
		pass_on_sum = pass_on_sum - COINS[position];
	}
	
	n_ways
}


fn main() {
    println!("Number of ways to make £2.00: {}", n_ways_rec(200, 0));
}
