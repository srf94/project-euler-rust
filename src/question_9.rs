fn find_ab(abc_sum: usize) -> (usize, usize){
    let target = abc_sum * abc_sum / 2;

    for a in 1..abc_sum {
    	for b in a..abc_sum {
    		if a * (abc_sum - b) + abc_sum * b == target {
    		    return (a, b)
    		}	
    	}
    }
	(0, 0)
}

fn main() {
    let abc_sum = 1000;	
    let ab_tup = find_ab(abc_sum);
    
    let a = ab_tup.0;
    let b = ab_tup.1;
    let c = abc_sum - a - b;
    
    let prod = a * b * c;
    
	println!("Product of a, b, c is: {}", prod);

}
