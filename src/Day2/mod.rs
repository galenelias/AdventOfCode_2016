use std::io::{self, BufRead};

// Find the (row,column) of the '5' key, which is always the start
fn start_pos(keypad : &Vec<Vec<char>>) -> (usize, usize) {
	let row = keypad.iter().position(|pad_row| pad_row.iter().any(|ch| ch == &'5')).unwrap();
	(row, keypad[row].iter().position(|ch| ch == &'5').unwrap())
}

// Type in the code given by the sequence of inputs, returning the code as a string
fn type_code(commands : &Vec<Vec<char>>, keypad : &Vec<Vec<char>>) -> String {
	let mut pos = start_pos(&keypad);
	let mut code : Vec<char> = Vec::new();
	let rows = keypad.len();
	let cols = keypad[0].len();

	for command in commands {
		for ch in command {
			let new_pos = match ch {
				&'U' if pos.0 > 0 => (pos.0 - 1, pos.1),
				&'D' if pos.0 < rows - 1 => (pos.0 + 1, pos.1),
				&'L' if pos.1 > 0 => (pos.0, pos.1 - 1),
				&'R' if pos.1 < cols - 1 => (pos.0, pos.1 + 1),
				_ => pos,
			};

			if keypad[new_pos.0][new_pos.1] != ' ' {
				pos = new_pos;
			}
		}
		code.push(keypad[pos.0][pos.1]);
	}
	code.iter().collect::<String>()
}

pub fn solve() {
	let stdin = io::stdin();
	let input: Vec<Vec<_>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.chars().collect())
		.collect();

	let keypad_1 = vec![
		vec!['1','2','3'],
		vec!['4','5','6'],
		vec!['7','8','9'] ];
	println!("Part 1: {}", type_code(&input, &keypad_1)); 

	let keypad_2 = vec![
		vec![' ',' ','1',' ',' '],
		vec![' ','2','3','4',' '],
		vec!['5','6','7','8','9'],
		vec![' ','A','B','C',' '],
		vec![' ',' ','D',' ',' '], ];
	println!("Part 2: {}", type_code(&input, &keypad_2)); 
}