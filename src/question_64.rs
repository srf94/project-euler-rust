struct RootFraction {
	a: isize,
	b: isize,
	c: isize,
	d: isize,
	sqrt: isize
}


fn invert_cancel_frac(mut frac: RootFraction) -> RootFraction {
	// First, invert the fraction
	frac = RootFraction { a: frac.c, b: frac.d, c: frac.a, d: frac.b, sqrt: frac.sqrt };

	// Now we have some fraction (a*sqrt + b) / (c*sqrt + d)
	// Multiply top and bottom by (c*sqrt - d)
	// Gives (a*c*sqrt*sqrt - b*d + sqrt*(b*c - a*d)) / (c*c*sqrt*sqrt - d*d)

	let new_a = frac.b * frac.c - frac.a * frac.d;
	let new_b = frac.a * frac.c * frac.sqrt - frac.b * frac.d;
	let new_c = 0 as isize;
	let new_d = frac.c * frac.c * frac.sqrt - frac.d * frac.d;

	RootFraction { a: new_a, b: new_b, c: new_c, d: new_d, sqrt: frac.sqrt }
}



fn euclid_step(a: isize, b: isize) -> isize {
	if a % b == 0 { return b; }

	euclid_step(b, a % b)
}


fn gcd(a: isize, b: isize) -> isize {
	if a >= b {
		return euclid_step(a, b);
	} 
	euclid_step(b, a)
}


fn cancel_frac(mut frac: RootFraction) -> RootFraction {
	if frac.c != 0 { return frac; }

	let factorise_top = gcd(frac.a, frac.b);
	let gcd = gcd(factorise_top, frac.d);

	frac.a = frac.a / &gcd;
	frac.b = frac.b / &gcd;
	frac.d = frac.d / &gcd;
	frac
}


fn split_frac(mut frac: RootFraction) -> (isize, RootFraction) {
	// Split fraction into an integer and fractional part
	// Want largest integer part possible where the fractional part is greater than 0

	if frac.c != 0 {
		panic!("Fraction.c was non-zero! a: {:?}, b: {:?}, c: {:?}, d: {:?}, sqrt: {:?}", 
			frac.a, frac.b, frac.c, frac.d, frac.sqrt);
	}

	let mut i = 0;
	while frac.sqrt - (i+1) * (i+1) > 0 { i += 1 }

	let new_int = (frac.b + i) / frac.d;
	let new_b = -i + (frac.b + i - new_int * frac.d);
	frac.b = new_b;

	(new_int, frac)
}


fn odd_period(num: isize) -> bool {
	let sqrt = (num as f64).sqrt() as isize;
	if sqrt * sqrt == num { return false; }

	let mut frac = RootFraction { 
		a: 1 as isize, 
		b: 0 as isize, 
		c: 0 as isize, 
		d: 1 as isize, 
		sqrt: num as isize 
	};

	let mut vector = Vec::new();

	'outer: loop {
	    let tuple = split_frac(frac);
	    let int = tuple.0;
	    frac = tuple.1;

	    let new_entry = (int, frac.a, frac.b, frac.c, frac.d);
	    if vector.contains(&new_entry) {
	    	// Find first day this entry occurs

	    	for (i, tuple) in vector.iter().enumerate() {
	    		if tuple == &new_entry {
	    			let period = vector.len() - i;
	    			if period % 2 != 0 { return true; }
	    			return false;
	    		}
	    	}
	    }
	    vector.push(new_entry);

    	frac = invert_cancel_frac(frac);

    	frac = cancel_frac(frac);
	    // println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}, sqrt: {:?}", frac.a, frac.b, frac.c, frac.d, frac.sqrt);
    }
}


fn main() {
	// Decided to crete a more engineered solution than necessary to get used to using structs
	// Code may come in handy for any future continued fractions questions!

	let mut odd_periods = 0;

	for num in 2..10001 {
		if odd_period(num) { odd_periods += 1; }
	}
	println!("Odd periods: {:?}", odd_periods);
}
