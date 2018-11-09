use std::io::{self, BufRead};
use permutohedron::LexicalPermutation;

fn scramble(mut password : Vec<char>, instructions : &Vec<Vec<String>>) -> Vec<char> {
	let password_len = password.len();

	for line in instructions {
		match line[0].as_ref() {
			"swap" => {
				match line[1].as_ref() {
					"letter" => {
						let pos1 = password.iter().position(|&ch| ch == line[2].chars().next().unwrap()).unwrap();
						let pos2 = password.iter().position(|&ch| ch == line[5].chars().next().unwrap()).unwrap();
						password.swap(pos1, pos2);
					},
				 	"position" => password.swap(line[2].parse::<usize>().unwrap(), line[5].parse::<usize>().unwrap()),
					_ => panic!("Bad swap"),

				};
			},
			"rotate" => {
				let amount = line[2].parse::<usize>();
				match line[1].as_ref() {
					"left" => password.rotate_left(amount.unwrap()),
					"right" => password.rotate_right(amount.unwrap()),
					"based" => {
						let pos = password.iter().position(|&ch| ch == line[6].chars().next().unwrap()).unwrap();
						let rotate_pos = ((pos + 1) + (if pos >= 4 { 1 } else { 0} )) % password_len;
						password.rotate_right(rotate_pos);
					}
					_ => panic!("Bad rotate"),

				};
			},
			"reverse" => {
				let pos1 = line[2].parse::<usize>().unwrap();
				let pos2 = line[4].parse::<usize>().unwrap();
				password[pos1..(pos2+1)].reverse();
			}
			"move" => {
				let ch = password.remove(line[2].parse::<usize>().unwrap());
				password.insert(line[5].parse::<usize>().unwrap(), ch);
			}
			_ => panic!("Unhandled instruction: '{}'", line[0]),
		}
	}
	password
}

fn part1(instructions : &Vec<Vec<String>>) {
	let mut password = "abcdefgh".chars().collect::<Vec<_>>();
	password = scramble(password, &instructions);
	println!("Part 1: {}", password.iter().collect::<String>());
}

fn part2(instructions : &Vec<Vec<String>>) {
	let needle = "fbgdceah".chars().collect::<Vec<_>>();
	let mut password = "abcdefgh".chars().collect::<Vec<_>>();
	loop {
		let scrambled = scramble(password.clone(), &instructions);

		if scrambled == needle {
			println!("Part 2: {}", password.iter().collect::<String>());
			break;
		}

		if !password.next_permutation() {
			break;
		}
	}
}

pub fn solve() {
	let stdin = io::stdin();
	let instructions: Vec<Vec<_>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split_whitespace().map(String::from).collect::<Vec<_>>())
		.collect();

	part1(&instructions);
	part2(&instructions);
}