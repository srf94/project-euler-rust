
fn solve(limit: i64) -> i64 {
	let sum_squares = (1..limit+1).fold(0, |sum, x| sum + x*x);
	
	let sum = (1..limit+1).fold(0, |sum, x| sum + x);
	let squared_sum = sum * sum;
	
	squared_sum - sum_squares
}


fn main() {
    println!("Difference between sums: {}", solve(100));
}
