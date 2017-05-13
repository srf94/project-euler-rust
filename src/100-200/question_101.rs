fn calc_fn(n: f64, coefs: &Vec<i64>) -> f64 {
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


fn main() {
	let target_fn = vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1];
	println!("{:?}", calc_fn(1.0 as f64, &target_fn));
	println!("{:?}", calc_fn(2.0 as f64, &target_fn));
	println!("{:?}", calc_fn(3.0 as f64, &target_fn));
	println!("{:?}", calc_fn(4.0 as f64, &target_fn));

	// OP(1, n)
	// a_0 = f(n) for n = 1
	let A = vec![vec![1]];

	// OP(2, n)
	// a_0 + n*a_1 = f(n) for n = 1, 2
	// (1  1) (a_0) = (f(1))
	// (1  2) (a_1)   (f(2))
	let A = vec![vec![1.0 as f64, 1.0 as f64], vec![1.0 as f64, 2.0 as f64]];
	// let b = vec![calc_fn(1, &target_fn), calc_fn(2, &target_fn)];
	let b = vec![vec![calc_fn(1.0 as f64, &target_fn)], vec![calc_fn(2.0 as f64, &target_fn)]];

	let tup = LU_decompose(A);
	let P = tup.0;
	let L = tup.1;
	let U = tup.2;

	// Ly = Pb
	let Pb = mult_matrix(&P, &b);

	let mut y = vec![0.0 as f64; b.len()];
	y[0] = Pb[0][0] / L[0][0];
	y[1] = (Pb[1][0] - L[1][0] * y[0]) / L[1][1];

	// Ux = y
	let mut x = vec![0.0 as f64; b.len()];
	x[1] = y[1] / U[1][1];
	x[0] = (y[0] - U[0][1] * y[1])/ U[0][0];

	println!("{:?}", L);
	println!("{:?}", Pb);
	println!("{:?}", y);

	println!("{:?}", U);
	println!("{:?}", x);

	// x = [-681, 682] which is what we expect! This means f(1)=1, f(2)=638 (both correct)
	// Then f(3) = 1365 - our FIT.

}
