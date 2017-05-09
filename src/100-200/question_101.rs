fn calc_fn(n: i64, coefs: &Vec<i64>) -> i64 {
	let mut tot = 0;
	for (power, coef) in coefs.iter().enumerate() {
		tot += coef * n.pow(power as u32);
	}
	tot
}


fn main() {
	let target_fn = vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1];
	println!("{:?}", calc_fn(1, &target_fn));
	println!("{:?}", calc_fn(2, &target_fn));
	println!("{:?}", calc_fn(3, &target_fn));
	println!("{:?}", calc_fn(4, &target_fn));

	// OP(1, n)
	// a_0 = f(n) for n = 1
	let o_1 = [target_fn[0]];

	// OP(2, n)
	// a_0 + n*a_1 = f(n) for n = 1, 2
	// (1  1) (a_0) = (f(1))
	// (1  2) (a_1)   (f(2))

	// OP(m, n)
	// (1  1  1  ..    1   ) (a_0) = (f(1))
	// (1  2  4  .. 2^(m-1)) (a_1) = (f(2))
	// (1  3  9  .. 3^(m-1)) (...) = (...)
	// (1  4  16 .. 4^(m-1)) (a_m) = (f(m))

	// So I need to find a matrix library (cgmath???) to handle this stuff for me
	// Invert the matrix, find the coefficents.
	// Find the BOP for each and we are done!

}
