use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(PartialEq, Copy, Clone)]
enum Dir {
	North,
	East,
	South,
	West,
}

pub fn solve() {
	let stdin = io::stdin();
	let input : Vec<_> = stdin.lock().lines().next().unwrap().unwrap()
		.split(", ").map(String::from).collect();

	let dirs = vec!(Dir::North, Dir::East, Dir::South, Dir::West);
	let dirs_len = dirs.len();

	let mut x : i32 = 0;
	let mut y : i32 = 0;
	let mut dir = Dir::North;

	let mut first_intersection : Option<(i32,i32)> = None;
	let mut path : HashSet<(i32,i32)> = HashSet::new();

	for cmd in input {
		let (mv, dist) = cmd.split_at(1);
		let dist = dist.parse::<i32>().unwrap();

		let dir_arr_pos = dirs.iter().position(|d| d == &dir).unwrap();
		dir = match mv {
			"L" => dirs[(dir_arr_pos + dirs_len - 1) % dirs_len],
			"R" => dirs[(dir_arr_pos + 1) % dirs_len],
			_ => panic!("Unexpected direction"),
		};

		for _ in 0..dist {
			match dir {
				Dir::North => y += 1,
				Dir::South => y -= 1,
				Dir::West => x -= 1,
				Dir::East => x += 1,
			}

			if first_intersection.is_none() && path.contains(&(x,y)) {
				first_intersection = Some((x,y))
			} else {
				path.insert((x,y));
			}
		}
	}

	println!("Part 1: {}", x.abs() + y.abs());
	let int = first_intersection.unwrap_or_default();
	println!("Part 2: {:?}", int.0.abs() + int.1.abs());
}