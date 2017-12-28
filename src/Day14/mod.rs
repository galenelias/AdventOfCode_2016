use std::io::{self, BufRead};
use std::collections::VecDeque;
use std::iter;
use md5;

// Find set of 3 repeated characters, or return none otherwise
fn get_key(hash : &[char]) -> Option<char> {
	for window in hash.windows(3) {
		if window[0] == window[1] && window[0] == window[2] {
			return Some(window[0]);
		}
	}
	None
}

fn is_potential_key(hash : &[char]) -> bool {
	get_key(hash).is_some()
}

fn stretch_hash(mut initial_hash : String) -> String {
	for _ in 0..2016 {
		initial_hash = format!("{:x}", md5::compute(initial_hash));
	}
	initial_hash
}

fn generate_keys(input : &str, key_amount : usize, salt_transform : fn(String) -> String) -> usize {
	let mut output : VecDeque<(usize, Vec<char>)> = VecDeque::new();
	let mut keys = 0;
	let mut i = 0;

	loop {
		let digest_str = salt_transform(format!("{}{}", input, i)).chars().collect::<Vec<char>>();
		if is_potential_key(&digest_str) {
			// Try to pull off hashes and see if they are valid keys
			while !output.is_empty() && output.front().unwrap().0 + 1000 <= i {
				let hash = output.pop_front().unwrap();
				let ch5 = iter::repeat(get_key(&hash.1).unwrap()).take(5).collect::<Vec<char>>();
				let ch5_slice : &[char] = &ch5;
				if let Some(_pos) = output.iter().position(|val| hash.0 + 1000 > val.0 && (val.1).windows(5).any(|window| window == ch5_slice)) {
					keys += 1;
					// println!("Found key: {}  {} ({} matched {})", keys, hash.1.iter().collect::<String>(), hash.0, output[_pos].0);

					if keys == key_amount {
						return hash.0;
					}
				}
			}

			output.push_back((i, digest_str));
		}
		i += 1;
	}
}

pub fn solve() {
	let stdin = io::stdin();
	let input = stdin.lock().lines().next().unwrap().unwrap().to_string();

	println!("Part 1: {}", generate_keys(&input, 64, |salt| format!("{:x}", md5::compute(salt))));
	println!("Part 2: {}", generate_keys(&input, 64, |salt| stretch_hash(format!("{:x}", md5::compute(salt)))));
}
