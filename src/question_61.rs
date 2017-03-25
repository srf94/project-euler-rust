
struct Remaining {
	squr: bool,
	pent: bool,
	hex: bool,
	hept: bool,
	oct: bool
}


struct Vectors {
	// tris: Vec<String>,
	squares: Vec<String>,
	pents: Vec<String>,
	hexs: Vec<String>,
	hepts: Vec<String>,
	octs: Vec<String>
}


fn generate(f: &Fn(usize) -> usize) -> Vec<String> {
	let mut vector = Vec::new();
	let mut n = 1;

	loop {
		let number = f(n);
		if number >= 10000 { return vector; }
		if number >= 1000 { vector.push(number.to_string()); }
		n += 1;
	}
}

fn triangle(n: usize) -> usize { n * (n + 1) / 2 }

fn square(n: usize) -> usize { n * n }

fn pentagonal(n: usize) -> usize { n * (3*n - 1) / 2 }

fn hexagonal(n: usize) -> usize { n * (2*n - 1) }

fn heptagonal(n: usize) -> usize { n * (5*n - 3) / 2 }

fn octagonal(n: usize) -> usize { n * (3*n - 2) }


fn recursive(string: &str, mut remaining: &mut Remaining, vectors: &Vectors) -> Vec<Vec<String>> {
    let last_two = &string[2..4];

    if !remaining.squr && !remaining.pent && !remaining.hex && !remaining.hept && !remaining.oct  {
    	return vec![vec![string.to_string()]];
    }

    let mut results = Vec::new();

	if remaining.squr {
		remaining.squr = false;

		for squr in &vectors.squares {
			if &squr[..2] == last_two {
				let result = recursive(squr, &mut remaining, vectors);
				for res in result {
					let mut var = vec![string.to_string()];
					var.extend(res);
					results.push(var);
				}
			}
		}
		remaining.squr = true;
	}

	if remaining.pent {
		remaining.pent = false;
		for pent in &vectors.pents {
			if &pent[..2] == last_two {
				let result = recursive(pent, &mut remaining, vectors);
				for res in result {
					let mut var = vec![string.to_string()];
					var.extend(res);
					results.push(var);
				}
			}
		}
		remaining.pent = true;
	}

	if remaining.hex {
		remaining.hex = false;
		for hex in &vectors.hexs {
			if &hex[..2] == last_two {
				let result = recursive(hex, &mut remaining, vectors);
				for res in result {
					let mut var = vec![string.to_string()];
					var.extend(res);
					results.push(var);
				}			
			}
		}
		remaining.hex = true;
	}

	if remaining.hept {
		remaining.hept = false;
		for hept in &vectors.hepts {
			if &hept[..2] == last_two {
				let result = recursive(hept, &mut remaining, vectors);
				for res in result {
					let mut var = vec![string.to_string()];
					var.extend(res);
					results.push(var);
				}			
			}
		}
		remaining.hept = true;
	}	

	if remaining.oct {
		remaining.oct = false;
		for oct in &vectors.octs {
			if &oct[..2] == last_two {

				let result = recursive(oct, &mut remaining, vectors);
				for res in result {
					let mut var = vec![string.to_string()];
					var.extend(res);
					results.push(var);
				}
			}
		}
		remaining.oct = true;
	}

	results
}


fn main() {

    let vectors = Vectors {
    	squares: generate(&square),
    	pents: generate(&pentagonal),
    	hexs: generate(&hexagonal),
    	hepts: generate(&heptagonal),
    	octs: generate(&octagonal)
    };

    let mut final_results = Vec::new();

    for tri in generate(&triangle) {
    	let mut remaining = Remaining { squr: true, pent: true, hex: true, hept: true, oct: true };
    	let results = recursive(&tri, &mut remaining, &vectors);

    	for result in results {
    		if result.len() == 6 { 
    			if &result[0][0..2] == &result[5][2..4] {
	    			final_results.push(result); 
    			}
    		}
    	}
    }

	println!("Total solutions: {:?}", final_results.len());
	println!("First solution: {:?}", final_results[0]);

	let mut sum = 0;
	for string in &final_results[0] { sum += string.parse::<usize>().unwrap(); }
	println!("Sum of first solution: {:?}", sum);

}





