
fn euclid_step(a: i64, b: i64) -> i64 {
	if a % b == 0 { return b; }
	
	euclid_step(b, a % b)
}


fn gcd(a: i64, b: i64) -> i64 {
	if a >= b {
		return euclid_step(a, b);
	} else {	
		return euclid_step(b, a);
	}
}


fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}


fn solve(num: i64, max: i64) -> i64 {
    if num == max { return num; }
    lcm(num, solve(num + 1, max))
}


fn main() {
    println!("Smallest divisible number: {}", solve(1, 20))
}
