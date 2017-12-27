use std::io::{self, BufRead};

fn checksum(input : &[char]) -> Vec<char> {
	input.windows(2).step_by(2).map(|window| if window[0] == window[1] { '1' } else { '0' }).collect::<Vec<_>>()
}

pub fn solve() {
	let stdin = io::stdin();
	let mut data = stdin.lock().lines().next().unwrap().unwrap().to_string();

	let target_length = 272;

	// Dragon curve up to target length
	while data.len() < target_length {
		let mut reverse_half = data.chars().map(|ch| if ch == '0' { '1' } else { '0' }).collect::<Vec<char>>();
		reverse_half.reverse();
		data.push('0');
		data.push_str(&reverse_half.iter().collect::<String>());
	}

	// Trim down to target length
	data = data[0..target_length].to_string();

	let mut cs = checksum(&data.chars().collect::<Vec<_>>());
	while cs.len() % 2 == 0 {
		cs = checksum(&cs);
	}

	println!("Part 1: {}", cs.iter().collect::<String>());
}