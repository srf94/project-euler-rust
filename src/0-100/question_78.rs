extern crate num;

use num::{BigInt, One, Zero};


fn main() {
	let target = "1000000";
	let big_target = BigInt::parse_bytes(target.as_bytes(), 10).unwrap();

	let mut partitions = vec![One::one()];

	let mut n_coins = 1;
	loop {


		let mut pents = Vec::new();
		let mut k: isize = 1;

		loop {
			let pent = (k * (3*k - 1) / 2) as usize;

			// Can break here as p(n_coins - pent) = 0 for all pent > n_coins
			if pent > n_coins { break; }
			pents.push(pent);

			// k follows this pattern: 1, -1, 2, -2, 3, -3, ..
			if k > 0 { k = -k; } 
			else { k = -(k - 1); }
		}


		// Use the formula p(n) = p(n-1) + p(n-2) - p(n-5) - p(n-7) + p(n-12) + ...
		let mut partition_num: BigInt = Zero::zero();

		for (sign, pent) in pents.iter().enumerate() {
			let index_loc = n_coins - pent;

			if sign % 4 == 0 || sign % 4 == 1 {
				partition_num = &partition_num + &partitions[index_loc];
			} else {
				partition_num = &partition_num - &partitions[index_loc];
			}
		}

		if (&partition_num / &big_target) * &big_target == partition_num {
			println!("First n with a partition number divisible by 1 million: {:?}", n_coins);
			break;
		}

		partitions.push(partition_num);
		n_coins += 1;
	}
}
