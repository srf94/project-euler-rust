fn calc_fn(n: f64, coefs: &Vec<f64>) -> f64 {
	// Given a vector representation of f(n), calculate f(n)

	let mut tot = 0.0 as f64;
	for (power, coef) in coefs.iter().enumerate() {
		tot += (*coef as f64) * n.powi(power as i32);
	}
	tot
}


fn mult_matrix(M: &Vec<Vec<f64>>, N: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
	// Multiply two matricies with dimensions axb and bxc

	let a = M.len();
	let b = M[0].len();
	let b_ = N.len();
	let c = N[0].len();

	if b != b_ {
		panic!("Tried to multiply matricies with dimensions {}x{} and {}x{}", a, b, b_, c);
	}

	let mut out = vec![];
	for (row_loc, row_m) in M.iter().enumerate() {
		let mut row_out = vec![];
		for col_loc in 0..c {
			let mut sum = 0.0 as f64;
			for x in 0..b {
				sum += M[row_loc][x] * N[x][col_loc];
			}
			row_out.push(sum);
		}
		out.push(row_out);
	}

	out
}


fn pivot_matrix(M: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
	// Create a pivot matrix as used in the Doolittle LU method

    let m = M[0].len();

    let mut id_mat = vec![vec![0.0 as f64; m]; m];
    for i in 0..m {
        id_mat[i][i] = 1.0 as f64;
    }

    for j in 0..m {
        let mut row = j;
        let mut max = 0.0 as f64;
        for i in j..m {
            if M[i][j].abs() > max {
                max = M[i][j].abs();
                row = i;
            }
        }
        if j != row {
        	let temp = id_mat[row].clone();
            id_mat[row] = id_mat[j].clone();
            id_mat[j] = temp;
        }
    }

    id_mat
}


fn LU_decompose(A: Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>) {
	// Calculate the LU decomposition of A (using the pivot method)
	// Used to transform Ax = b into LUx = Pb

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

	(P, L, U)
}


fn solve_LU_system(L: Vec<Vec<f64>>, U: Vec<Vec<f64>>, Pb: Vec<Vec<f64>>) -> Vec<f64>{
	// Solve equations of the form LUx = Pb by substitution

	let n = Pb.len();

	// First solve Ly = Pb for y
	let mut y = vec![0.0 as f64; n];
	for i in 0..n {
		// Need to subtract already known values of L * y as they become known
		let mut minus_sum = 0.0 as f64;
		for j in 0..i {
			minus_sum += L[i][j] * y[j];
		}

		y[i] = (Pb[i][0] - minus_sum) / L[i][i];
	}

	// Now solve Ux = y for x
	let mut x = vec![0.0 as f64; n];
	for i in (0..n).rev() {
		// Need to subtract already known values of U * x as they become known
		let mut minus_sum = 0.0 as f64;
		for j in i..n {
			minus_sum += U[i][j] * x[j];
		}

		x[i] = (y[i] - minus_sum) / U[i][i];
	}

	x
}


fn create_Ax_b(n: u32, target_fn: &Vec<f64>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
	// Create a matrix A and vector b that represents the problem in Ax=b format

	// EXAMPLE
	// n = 4
	// a_0 + n*a_1 + n^2*a_2 + n^3*a_3 = f(n) for n = 1, 2, 3, 4
	// (1  1  1  1)   (a_0) = (f(1))
	// (1  2  4  16)  (a_1)   (f(2))
	// (1  3  9  27)  (a_2)   (f(3))
	// (1  4  16 64)  (a_3)   (f(4))

	let mut A = vec![];
	for i in 1..n+1 {
		let mut inner_v = vec![];
		for j in 0..n {
			inner_v.push((i as i64).pow(j) as f64);
		}
		A.push(inner_v);
	}

	let mut b = vec![];
	for i in 1..n+1 {
		let val = calc_fn(i as f64, target_fn);
		b.push(vec![val]);
	}

	(A, b)
}


fn main() {
	// Represent polynomials as a vector of coefficents: f(n) = a_0 + a_1*n + ...
	let target_fn = vec![1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0];

	let mut FIT_sum = 0.0;
	for n in 1..11 {
		let tup = create_Ax_b(n, &target_fn);
		let A = tup.0;
		let b = tup.1;

		let tup = LU_decompose(A);
		let P = tup.0;
		let L = tup.1;
		let U = tup.2;

		let Pb = mult_matrix(&P, &b);
		let solved = solve_LU_system(L, U, Pb);

		// It is possible that the n+1'th term still fit with the target polynomial
		// To generalise, this should be checked here
		// However as we have a n^k term for k=1, 10 inclusive we know this cannot happen here
		FIT_sum += calc_fn((n+1) as f64, &solved);

		println!("n: {:?}", n);
		println!("Actual value for n+1: {:?}", calc_fn((n+1) as f64, &target_fn));
		println!("Calculated value for n+1: {:?}", calc_fn((n+1) as f64, &solved));
		println!("");
	}

	// I am happy rounding to i64 as we know the final answer must be in interger in this example
	println!("Sum of FIT: {:?}", FIT_sum as i64);
}
