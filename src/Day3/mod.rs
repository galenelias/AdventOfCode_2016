use std::io::{self, BufRead};


pub fn solve() {
	let stdin = io::stdin();

	let input: Vec<Vec<usize>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split_whitespace().map(|w| w.parse::<usize>().unwrap()).collect())
		.collect();

	let mut possible = 0;
	for line in &input {
		let mut lengths = line.clone();
		lengths.sort();

		if lengths[0] + lengths[1] > lengths[2] {
			possible += 1;
		}
	}
	println!("Part 1: {}", possible);

	let mut possible_2 = 0;
	for row in 0..input.len()/3 {
		for col in 0..3 {
			let mut lengths : Vec<usize> = Vec::new();
			lengths.push(input[row*3][col]);
			lengths.push(input[row*3 + 1][col]);
			lengths.push(input[row*3 + 2][col]);
			lengths.sort();

			if lengths[0] + lengths[1] > lengths[2] {
				possible_2 += 1;
			}
		}
	}

	println!("Part 1: {}", possible_2);
}