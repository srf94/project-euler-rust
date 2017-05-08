fn step(n: u64) -> u64 {
	let str_n = n.to_string();
	let bytes = str_n.into_bytes();

	let mut sum = 0;
	for byte in bytes {
		let num = (byte - 48) as u64;
		sum += num * num;
	}
	sum
}


fn main() {
	let mut total_sum = 0;

	for n in 1..10000001 {
		let mut current_num = n;

		while current_num != 1 && current_num != 89 {
			current_num = step(current_num);
		}

		if current_num == 89 { total_sum += 1 }
	}

	println!("{:?}", total_sum);
}
