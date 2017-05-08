fn sum_proper_div(value: i32) -> i32 {
	let search_max = (value as f32).sqrt() as i32 + 1;

	let mut sum = 1;
	for j in 2..search_max {
		if &value % j == 0 { 
			sum += j;
			sum += value / j;
		}
	}
	if (search_max - 1).pow(2) == value { sum -= search_max - 1; }
	sum
}

fn main() {
	let mut amicable_sum: i32 = 0;

    for i in 2..10000 {
    	let sum_div = sum_proper_div(i);
    	if sum_div == i { continue; }

    	if sum_proper_div(sum_div) == i {
    		amicable_sum += i;
    	}
    }

    println!("Sum of all amicable numbers: {}", amicable_sum)
}
