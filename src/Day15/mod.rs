use std::io::{self, BufRead};
use regex::Regex;

pub fn solve() {
	let re = Regex::new("[[:digit:]]+").unwrap();
	let stdin = io::stdin();
	let inputs: Vec<Vec<_>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|l| re.find_iter(&l)
			.filter_map(|m| m.as_str().parse::<i32>().ok())
			.collect())
		.collect();

	for delay in 0.. {
		if inputs.iter().all(|disc| (disc[0] + delay + disc[3]) % disc[1] == 0) {
			println!("Part 1: {}", delay);
			break;
		}
	}
}