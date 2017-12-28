use std::io::{self, BufRead};
use std::collections::VecDeque;
use md5;

struct State {
	pos : (i32,i32),
	path : String,
}

fn is_open(ch : char) -> bool {
	match ch {
		'b' | 'c' | 'd' | 'e' | 'f' => true,
		_ => false,
	}
}

pub fn solve() {
	let stdin = io::stdin();
	let passcode = stdin.lock().lines().next().unwrap().unwrap().to_string();

	let mut queue = VecDeque::new();

	queue.push_back(State { pos: (0,0), path: String::from("")});

	while !queue.is_empty() {
		let s = queue.pop_front().unwrap();

		// println!("At: {},{}, path = {}", s.pos.0, s.pos.1, s.path);
		if s.pos == (3,3) {
			println!("Part 1: {}", s.path);
			break;
		}

		let digest_str = format!("{:x}", md5::compute(format!("{}{}", passcode, s.path))).chars().collect::<Vec<char>>();

		let mut try_move = |new_pos : (i32,i32), door_ch, path_ch, s : &State| {
			if is_open(door_ch) && new_pos.0 >= 0 && new_pos.0 <= 3 && new_pos.1 >= 0 && new_pos.1 <= 3 {
				queue.push_back(State { pos: new_pos.clone(), path: s.path.clone() + path_ch});
			}
		};

		try_move((s.pos.0, s.pos.1 - 1), digest_str[0], "U", &s);
		try_move((s.pos.0, s.pos.1 + 1), digest_str[1], "D", &s);
		try_move((s.pos.0 - 1, s.pos.1), digest_str[2], "L", &s);
		try_move((s.pos.0 + 1, s.pos.1), digest_str[3], "R", &s);
	}
}