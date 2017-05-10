fn calc_fn(n: i64, coefs: &Vec<i64>) -> i64 {
	let mut tot = 0;
	for (power, coef) in coefs.iter().enumerate() {
		tot += coef * n.pow(power as u32);
	}
	tot
}

fn mult_matrix(M: &Vec<Vec<f64>>, N: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
	let n = N[0].len();
	let mut out = vec![];
	for (row_loc, row_m) in M.iter().enumerate() {
		let mut row_out = vec![];
		for col_loc in 0..n {
			let mut sum = 0.0 as f64;
			for x in 0..n {
				sum += M[row_loc][x] * N[x][col_loc];
			}
			row_out.push(sum);
		}
		out.push(row_out);
	}

	out
}


fn pivot_matrix(A: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
	// TODO
	A.clone()
}


fn LU_decompose(A: Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
	let n = A[0].len();
	let mut L = vec![vec![0.0 as f64; n]; n];
	let mut U = vec![vec![0.0 as f64; n]; n];

	let P = pivot_matrix(&A);
	let PA = mult_matrix(&P, &A);

	for j in 0..n {
		L[j][j] = 1.0;

		for i in 0..j+1 {
			let mut sum = 0.0 as f64;
			for k in 0..i {
				sum += U[k][j] * L[i][k];
			}
            U[i][j] = PA[i][j] - sum;
		}

		for i in j..n {
			let mut sum = 0.0 as f64;
			for k in 0..j {
				sum += U[k][j] * L[i][k];
			}
            L[i][j] = (PA[i][j] - sum) / U[j][j];
		}

	}

	(L, U)
}



fn main() {
	let target_fn = vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1];
	println!("{:?}", calc_fn(1, &target_fn));
	println!("{:?}", calc_fn(2, &target_fn));
	println!("{:?}", calc_fn(3, &target_fn));
	println!("{:?}", calc_fn(4, &target_fn));

	// OP(1, n)
	// a_0 = f(n) for n = 1
	let A = vec![vec![1]];

	// OP(2, n)
	// a_0 + n*a_1 = f(n) for n = 1, 2
	// (1  1) (a_0) = (f(1))
	// (1  2) (a_1)   (f(2))
	let A = vec![vec![1.0 as f64, 1.0 as f64], vec![1.0 as f64, 2.0 as f64]];

	let B = vec![vec![4.0 as f64, 3.0 as f64], vec![6.0 as f64, 3.0 as f64]];
	// println!("{:?}", B);
	// println!("{:?}", LU_decompose(B));

	mult_matrix(&A, &B);

	let A = vec![vec![1, 1, 1],
	             vec![1, 2, 4],
	             vec![1, 3, 9]];

	// println!("{:?}", A);
	// println!("{:?}", LU_decompose(A));
	// OP(m, n)
	// (1  1  1  ..    1   ) (a_0) = (f(1))
	// (1  2  4  .. 2^(m-1)) (a_1) = (f(2))
	// (1  3  9  .. 3^(m-1)) (...) = (...)
	// (1  4  16 .. 4^(m-1)) (a_m) = (f(m))

	// So I need to find a matrix library (cgmath???) to handle this stuff for me
	// Invert the matrix, find the coefficents.
	// Find the BOP for each and we are done!

}
