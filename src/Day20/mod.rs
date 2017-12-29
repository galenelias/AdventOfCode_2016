use std::io::{self, BufRead};
use std::cmp;

pub fn solve() {
	let stdin = io::stdin();
	let mut inputs: Vec<Vec<_>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split('-').map(|tok| tok.parse::<u64>().unwrap()).collect::<Vec<_>>())
		.collect();

	inputs.sort();

	let mut current_range = (inputs[0][0], inputs[0][1]);
	let mut allowed_ips = 0;

	if current_range.0 > 0 {
		allowed_ips += current_range.0;
	}

	for range in inputs.iter().skip(1) {
		if range[0] > current_range.1 + 1 {
			if current_range.0 == 0 {
				println!("Part 1: {}", current_range.1 + 1);
			}

			allowed_ips += range[0] - current_range.1 - 1;
			current_range = (range[0], range[1]);
		}

		current_range.1 = cmp::max(current_range.1, range[1]);
	}

	if current_range.1 < 4294967295 {
		allowed_ips += 4294967295 - current_range.1;
	}

	println!("Part 2: {}", allowed_ips);
}