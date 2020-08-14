use rand::Rng;

fn main() {
	println!("{}", get_pass(16, true, true, true, false));
}

fn get_pass(len: usize, use_num: bool, use_lwr: bool, use_upr: bool, use_sym: bool) -> String {
	let mut letters = "".to_string();
	if use_num {letters.push_str("0123456789");}
	if use_lwr {letters.push_str("abcdefghijklmnopqrstuvwxyz");}
	if use_upr {letters.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");}
	if use_sym {letters.push_str("!\"#$%&'()-=^~\\|@`[{;+:*]},<.>/?_");}

	if letters == "" || len <= 0 {
		return "Error".to_string();
	}

	let mut i = 0;
	let mut password = "".to_string();
	let mut rng = rand::thread_rng();
	while i < len {
		let rand = rng.gen::<usize>() % letters.len();
		if !password.ends_with(letters.chars().nth(rand).unwrap()) {
			password.push_str(&letters.chars().nth(rand).unwrap().to_string());
			i += 1;
		}
	}
	password
}