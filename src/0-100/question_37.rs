fn is_prime(number: usize) -> bool {
    let search_max = (number as f64).sqrt() as usize + 1;

    for i in 2..search_max {
        if number % i == 0 { return false; }
    }

    true
}


fn prime_strip(number: &usize, primes: &Vec<usize>, left: bool) -> bool {
    if number.to_string().len() == 1 {
        return primes.contains(&number)
    }

    if left {
        if primes.contains(&number) && prime_strip(&number.to_string()[1..].parse::<usize>().unwrap(), &primes, left){
            return true;
        }
    } else {
        let mut str_num = number.to_string();
        let b = str_num.pop();
        if primes.contains(&number) && prime_strip(&str_num.parse::<usize>().unwrap(), &primes, left){
            return true;
        }
    }
    false
}


fn main() {
    let mut primes = vec![2, 3, 5, 7];

    let mut test_c = 0;
    let mut final_sum = 0;
    let mut final_count = 0;
    let mut number = 11;
    loop {
        if !is_prime(number) { number += 1; continue; }

        primes.push(number);

        if prime_strip(&number, &primes, true) && prime_strip(&number, &primes, false) {
            // We have found one of the required primes
            final_sum += number;
            final_count += 1;
            println!("{}", number);
        }
        if final_count == 11 { break; }

		number += 1;
		//println!("{}", number);

        //test_c += 1;
        //if test_c > 200 { break; }

    }
    println!("Total sum: {}", final_sum)
}