use std::io::{self, BufRead};

fn is_trap(prev : &[bool; 3]) -> bool {
	prev == &[true, true, false]
	 || prev == &[false, true, true]
	 || prev == &[true, false, false]
	 || prev == &[false, false, true]
}

fn build_next_row(input : &Vec<bool>) -> Vec<bool> {
	let grid_size = input.len();
	(0..grid_size).map(|c| {
		let prev = if c == 0 { [false, input[c], input[c+1]] }
					else if c == grid_size-1 { [input[c-1], input[c], false] }
					else { [input[c-1], input[c], input[c+1]] };

		is_trap(&prev)
	}).collect::<Vec<bool>>()
}

struct RoomGenerator {
	next: Vec<bool>,
}

impl RoomGenerator {
	fn new(initial_row : Vec<bool>) -> RoomGenerator {
		RoomGenerator { next: initial_row }
	}
}

impl Iterator for RoomGenerator {
	type Item = Vec<bool>;

	fn next(&mut self) -> Option<Vec<bool>> {
		let current = self.next.clone(); // Can't move for some reason...
		self.next = build_next_row(&current);
		Some(current)
	}
}

pub fn solve() {
	let stdin = io::stdin();
	let input = stdin.lock().lines().next().unwrap().unwrap().to_string();
	let initial_row = input.chars().map(|ch| ch == '^').collect::<Vec<_>>();

	println!("Part 1: {} (safe tiles)", RoomGenerator::new(initial_row.clone()).take(40).map(|row| row.iter().filter(|&b| *b == false).count()).sum::<usize>());
	println!("Part 2: {} (safe tiles)", RoomGenerator::new(initial_row).take(400000).map(|row| row.iter().filter(|&b| *b == false).count()).sum::<usize>());
}