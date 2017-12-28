use std::io::{self, BufRead};

pub fn solve() {
	let stdin = io::stdin();
	let input = stdin.lock().lines().next().unwrap().unwrap().parse::<usize>().unwrap();

	let mut elves = (0..input).collect::<Vec<_>>();
	while elves.len() > 1 {
		let start = if elves.len() % 2 == 1 { 2 } else { 0 };
		elves = elves.iter().skip(start).step_by(2).cloned().collect::<Vec<usize>>();

		// let mut trimmed = Vec::with_capacity(elves.len()/2);
		// for i in (start..elves.len()).step_by(2) {
			// trimmed.push(elves[i]);
		// }
		// elves = trimmed;
	}

	println!("Part 1: {}", elves[0] + 1);
}