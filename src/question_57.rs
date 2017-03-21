extern crate num;

use num::{BigUint, One};


fn single_step(int: &num::BigUint, top: num::BigUint, bottom: num::BigUint) -> (num::BigUint, num::BigUint) {
	// 1/(a + b/c)  ->  c/(ca + b)
	let new_bottom = int * &bottom + top;
	let new_top = bottom;
	(new_top, new_bottom)
}


fn nth_fraction(n: usize) -> (num::BigUint, num::BigUint) {
	let one: BigUint = One::one();
    let plus = &one + &one;
    let mut top: BigUint = One::one();
    let mut bot = &one + &one;

    for _ in 0..n - 1 {
	    let tuple = single_step(&plus, top, bot);
	    top = tuple.0;
	    bot = tuple.1;
	}

    let mut frac = (top, bot);
    frac.0 = &frac.0 + &frac.1;
    frac
}


fn main() {
    /*
	HOW IT WORKS:

    // 1 + 1/(2 + 1/2) - level 1
    let mut plus = 2;
    let mut top = 1;
    let mut bot = 2;

    let (top, bot) = single_step(plus, top, bot);
    top = top + bot;

    println!("{(:?}, {:?})", top, bot);

    // 1 + 1/(2 + 1/(2 + 1/(2 + 1/2))) - level 3
    let mut plus = 2;
    let mut top = 1;
    let mut bot = 2;

    let (top, bot) = single_step(plus, top, bot);
    let (top, bot) = single_step(plus, top, bot);
    let (top, bot) = single_step(plus, top, bot);
    top = top + bot;

    println!("{(:?}, {:?})", top, bot);
    */

    let mut top_heavy_count = 0;

    for n in 1..1001 {
    	let frac = nth_fraction(n);
    	let top_len = frac.0.to_str_radix(10).len();
    	let bot_len = frac.1.to_str_radix(10).len();

    	if top_len > bot_len {
    		top_heavy_count += 1;
    	}
    }

    println!("Top-heavy fraction count: {:?}", top_heavy_count);

}
