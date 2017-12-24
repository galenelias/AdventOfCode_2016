use std::io::{self, BufRead};

const ROWS: usize = 6;
const COLUMNS: usize = 50;

fn reverse_column(grid : &mut Vec<bool>, col : usize, start : usize, end : usize) {
	for i in 0..(end-start)/2 {
		grid.swap((start + i)*COLUMNS + col, (end - 1 - i)*COLUMNS + col);
	}
}

// Rotate using the reverse method:  https://www.geeksforgeeks.org/?p=2838
fn rotate_column(grid : &mut Vec<bool>, col : usize, amount : usize) {
	reverse_column(grid, col, 0, amount);
	reverse_column(grid, col, amount, ROWS);
	reverse_column(grid, col, 0, ROWS);
}

pub fn solve() {
	let stdin = io::stdin();

	let input: Vec<_> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.collect();

	let mut grid = vec![false; ROWS * COLUMNS];

	for line in input {
		let parts = line.split_whitespace().collect::<Vec<_>>();

		match parts[0] {
			"rect" => {
				let dims = parts[1].split('x').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
				for r in 0..dims[1] {
					for c in 0..dims[0] {
						grid[r*COLUMNS + c] = true;
					}
				}
			}
			"rotate" => {
				let target = parts[2].split('=').nth(1).unwrap().parse::<usize>().unwrap();
				let amount = parts[4].parse::<usize>().unwrap();
				match parts[1] {
					"row" => {
						let offset = target * COLUMNS;
						grid[offset..(offset+COLUMNS)].rotate(COLUMNS-amount);
					},
					"column" => {
						rotate_column(&mut grid, target, ROWS - amount)
					}
					_ => panic!("Unexpected rotation: {}", parts[1]),
				}
			}
			_ => panic!("Unhandled pattern: {}", parts[0]),
		}
	}

	println!("Part 1: {}", grid.iter().filter(|&&v| v).count());

	for row in 0..ROWS {
		for col in 0..COLUMNS {
			print!("{}", if grid[row * COLUMNS + col] { '#' } else { ' ' });
		}
		println!("");
	}
}