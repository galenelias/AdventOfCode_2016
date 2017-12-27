use std::io::{self, BufRead};
use std::collections::VecDeque;
use std::iter;
use md5;

fn get_key(hash : &[char]) -> &[char] {
	for i in 0..hash.len()-3 {
		if hash[i] == hash[i+1] && hash[i] == hash[i+2] {
			return &hash[i..i+3];
		}
	}
	return &[];
}

fn is_potential_key(hash : &[char]) -> bool {
	get_key(hash).len() > 0
}

pub fn solve() {
	let stdin = io::stdin();
	let input = stdin.lock().lines().next().unwrap().unwrap().to_string();

	let mut output : VecDeque<(usize, Vec<char>)> = VecDeque::new();
	let mut keys = 0;
	let mut i = 0;
	while keys < 64 {
		let digest_str = format!("{:x}", md5::compute(format!("{}{}", input, i))).chars().collect::<Vec<char>>();
		if is_potential_key(&digest_str) {
			// println!("Potential key: i={}, hash={}", i, digest_str.iter().collect::<String>());

			while !output.is_empty() && output.front().unwrap().0 + 1000 < i {
				// Try to pull off hashes and see if they are valid keys
				let hash = output.pop_front().unwrap();
				let ch_repeat = get_key(&hash.1);
				let ch5 = iter::repeat(ch_repeat[0]).take(5).collect::<Vec<char>>();
				let ch5_slice : &[char] = &ch5;
				// if output.iter().any(|&val| (val.1).contains(&ch5)) {
				if output.iter().any(|val| (val.1).windows(5).any(|window| window == ch5_slice)) {
					keys += 1;
					if keys == 64 {
						println!("Part 1: {} ({})", hash.1.iter().collect::<String>(), hash.0);

					}
				}
			}

			output.push_back((i, digest_str));
		}
		i += 1;
	}
}