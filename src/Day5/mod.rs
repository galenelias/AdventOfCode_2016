use std::io::{self, BufRead};
use md5;

fn part1(input : &str)
{
	let mut password = String::new();
	for i in 0.. {
		let digest_str = format!("{:x}", md5::compute(format!("{}{}", input, i)));
		if digest_str.chars().take(5).all(|c| c == '0') {
			password.push(digest_str.chars().nth(5).unwrap());

			if password.len() == 8 {
				break;
			}
		}
	}

	println!("Part 1: {}", password);
}

fn part2(input : &str)
{
	let mut password : Vec<char> = vec![' '; 8];
	for i in 0.. {
		let digest_str = format!("{:x}", md5::compute(format!("{}{}", input, i)));
		let digest_arr = digest_str.chars().collect::<Vec<char>>();
		if digest_arr.iter().take(5).all(|c| c == &'0') {
			let slot = digest_arr[5].to_digit(10).unwrap_or(9) as usize;
			if slot < 8 && password[slot] == ' ' {
				password[slot] = digest_arr[6];
			}

			if password.iter().all(|c| c != &' ') {
				break;
			}
		}
	}

	println!("Part 2: {}", password.iter().collect::<String>());
}

pub fn solve() {
	let stdin = io::stdin();

	println!("Enter input:");
	let input : String = stdin.lock().lines().next().unwrap().unwrap();

	println!("Computing part 1...");
	part1(&input);

	println!("Computing part 2...");
	part2(&input);
}