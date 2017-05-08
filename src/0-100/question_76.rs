use std::collections::HashMap;


fn fnc(sum: isize, largest: isize, mut saved_values: &mut HashMap<(isize, isize), isize>) -> isize {

	if largest == 0 { return 0; }
	if largest == 1 { return 1; }
	if sum == 0 { return 1; }
	if sum < 0 { return 0; }

	let mut part_1: isize;
	let mut part_2: isize;

	if saved_values.contains_key(&(sum, largest - 1)) {
		part_1 = saved_values.get(&(sum, largest - 1)).unwrap().clone();
	} else {
		part_1 = fnc(sum, largest - 1, &mut saved_values);
	}

	if saved_values.contains_key(&(sum - largest, largest)) {
		part_2 = saved_values.get(&(sum - largest, largest)).unwrap().clone();
	} else {
		part_2 = fnc(sum - largest, largest, &mut saved_values);
	}

	let result = part_1 + part_2;

	if !saved_values.contains_key(&(sum, largest)) {
		saved_values.insert((sum, largest), result);
	}

	// println!("{:?}", saved_values);

	result
}



fn main() {

	let mut saved_values: HashMap<(isize, isize), isize> = HashMap::new();

    println!("{:?}", fnc(100, 100, &mut saved_values) - 1);

}
