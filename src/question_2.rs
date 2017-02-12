fn solve(ceiling: i32) -> i32 {
    let mut done = false;
    
    let mut a = 0;
    let mut b = 1;
    let mut f: i32;
    let mut sum = 0;
    
    while !done {
        f = a + b;

    	if f > ceiling {
    	    done = true;
    	} else {
    	    if f % 2 == 0 {
    	        sum = sum + f;
    	    }
    	    a = b;
    	    b = f;
    	}
    }

    sum
}

fn main() {
    let ceiling = 4000000;
    println!("Sum: {}", solve(ceiling));
}

