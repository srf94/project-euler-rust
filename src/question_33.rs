fn fractions_equal(top: &str, bottom: &str, value: f64) -> bool {
	let top_int: i32 = top.parse().unwrap();
	let bot_int: i32 = bottom.parse().unwrap();
	
	//println!("{}", top);
	//println!("{}", bottom);
	//println!("{}", value);
	//println!("{}", (top_int as f64) / (bot_int as f64));
	//println!("{}", (top_int as f64) / (bot_int as f64) == value);

	((top_int as f64) / (bot_int as f64) - value).abs() < 0.000000001
}


fn is_cancelable(top: usize, bottom: usize) -> bool {
	let value = (top as f64) / (bottom as f64);
	let top_st = &top.to_string()[..];
	let bot_st = &bottom.to_string()[..];

	if top_st.contains("0") && bot_st.contains("0") { return false; }

	if &top_st[..1] == &bot_st[..1] {
		if fractions_equal(&top_st[1..], &bot_st[1..], value) { return true; }
	}

	if &top_st[..1] == &bot_st[1..] {
		if fractions_equal(&top_st[1..], &bot_st[..1], value) { return true; }
	}

	if &top_st[1..] == &bot_st[..1] {
		if fractions_equal(&top_st[..1], &bot_st[1..], value) { return true; }
	}

	if &top_st[1..] == &bot_st[1..] {
		if fractions_equal(&top_st[..1], &bot_st[..1], value) { return true; }
	}

	false
}


fn main() {
	let mut final_top = 1;
	let mut final_bottom = 1;

	for top in 10..99 {
		for bottom in top+1..100 {
			let top_st = &top.to_string()[..];
			let bot_st = &bottom.to_string()[..];
			
			if is_cancelable(top, bottom) {
				final_top *= top;
				final_bottom *= bottom;
				println!("{} / {}", top, bottom);
			}
		}
	}

	println!("Result: {}", (final_top as f64) / (final_bottom as f64));
}
