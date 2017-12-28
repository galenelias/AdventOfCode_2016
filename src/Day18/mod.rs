use std::io::{self, BufRead};
// use std::collections::VecDeque;

fn is_trap(prev : &[bool; 3]) -> bool {
	prev == &[true, true, false]
	 || prev == &[false, true, true]
	 || prev == &[true, false, false]
	 || prev == &[false, false, true]
}

pub fn solve() {
	let stdin = io::stdin();
	let input = stdin.lock().lines().next().unwrap().unwrap().to_string();

	let mut grid = Vec::<Vec<bool>>::new();
	grid.push(input.chars().map(|ch| ch == '^').collect::<Vec<_>>());

	let grid_size = grid[0].len();
	for _ in 1..grid_size {
		let mut row = vec![false; grid_size];

		{ 	// Lifetime scope for immutable borrow of grid via grid.last
			let prev_row = grid.last().unwrap();

			for c in 0..grid_size {
				let prev = if c == 0 { [false, prev_row[c], prev_row[c+1]] }
							else if c == grid_size-1 { [prev_row[c-1], prev_row[c], false] }
							else { [prev_row[c-1], prev_row[c], prev_row[c+1]] };

				row[c] = is_trap(&prev);
			}

		}
		grid.push(row);
	}

	for row in &grid {
		println!("{}", row.iter().map(|&b| if b { '^' } else { '.' }).collect::<String>());
	}

	println!("Part 1: {} (safe tiles)", grid.iter().map(|row| row.iter().filter(|&b| *b == false).count()).sum::<usize>());
}