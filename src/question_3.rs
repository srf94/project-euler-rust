fn is_prime(value: i64) -> bool{
    let search_max = (value as f64).sqrt() as i64 + 1;
    let mut is_prime = true;

    for n in 2..search_max {
        if value % n == 0 {
            is_prime = false;
            break
        }
    }

    is_prime
}


fn get_next_prime(value: i64) -> i64{
    let mut prime = value;
    
    loop{
        prime += 1;
        if is_prime(prime) {break}
    }
    prime
}


fn largest_prime_factor(target: i64) -> i64{
    let mut current_target = target;
    let mut prime = 2;
    let mut largest_prime = 1;
    
    while current_target > 1 {
        if current_target % prime == 0 {
            current_target /= prime;
            largest_prime = prime;
    	} else {
    	    prime = get_next_prime(prime);
    	}
    }
    largest_prime as i64
}

fn main() {
    println!("Largest factor: {}", largest_prime_factor(600851475143));
}

