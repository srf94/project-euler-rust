
fn is_palindrome(value: i32) -> bool{
    let string = format!("{}", value); 
	let reverse = string.chars().rev().collect::<String>();
	
	string == reverse
}


fn largest_palindrome(num_digits: i32) -> i32{
    let top_cap = (10 as i32).pow(num_digits as u32);
    let bottom_cap = (10 as i32).pow((num_digits - 1) as u32);

	let mut largest_palin = 0;

    for i in (bottom_cap..top_cap).rev() {
        for j in (i..top_cap).rev() {
            let prod = i * j;
            if prod > largest_palin {
                if is_palindrome(prod) {
                    largest_palin = prod;
                }
            }
        }
    }

    largest_palin
}


fn main() {
    println!("{}", largest_palindrome(3));
}
