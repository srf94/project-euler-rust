fn solve(max_num: i32) -> i32 {
	let numbers = (1..max_num).filter(|i| *i % 3 == 0 || *i % 5 == 0);
	numbers.fold(0, |acc, x| acc + x)
}

fn main() {
    println!("Sum: {}", solve(1000));
}
