use std::io::{self, BufRead};
use std::collections::HashMap;

pub fn solve() {
	let stdin = io::stdin();

	let input: Vec<Vec<char>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.chars().collect::<Vec<_>>())
		.collect();

	let columns = input[0].len();
	let mut part1 = Vec::new();
	let mut part2 = Vec::new();

	for i in 0..columns {
		let mut hist = HashMap::new();
		for line in &input {
			*hist.entry(line[i]).or_insert(0) += 1;
		}

		part1.push(hist.iter().max_by_key(|&(_ch,freq)| freq).unwrap().0.clone());
		part2.push(hist.iter().min_by_key(|&(_ch,freq)| freq).unwrap().0.clone());
	}

	println!("Part 1: {}", part1.iter().collect::<String>());
	println!("Part 2: {}", part2.iter().collect::<String>());
	
}