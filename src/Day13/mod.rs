use std::io::{self, BufRead};
use std::collections::VecDeque;
use std::collections::HashSet;

pub fn solve() {
	let stdin = io::stdin();
	let input = stdin.lock().lines().next().unwrap().unwrap().parse::<i64>().unwrap();

	let mut visited = HashSet::new();
	let mut queue = VecDeque::new();
	queue.push_back((0, (1, 1)));

	let is_wall = |&(_d, (x, y)) : &(i64, (i64,i64))| {
		let temp : i64 = x*x + 3*x + 2*x*y + y + y*y + input;
		(x < 0 || y < 0 || (temp.count_ones() % 2) == 1)
	};

	// let target = (7,4); // sample target
	let target = (31,39); // puzzle input target

	let mut part1_done = false;
	let mut part2_done = false;

	while !queue.is_empty() && (!part1_done || !part2_done) {
		let node = queue.pop_front().unwrap();

		if node.1 == target {
			println!("Part 1: {}", node.0);
			part1_done = true;
		}

		if node.0 == 51 && !part2_done {
			println!("Part 2: {}", visited.len());
			part2_done = true;
		}

		visited.insert(node.1);

		let mut push_if_not_wall_and_not_visited = |n : (i64, (i64,i64))| {
			if !is_wall(&n) && !visited.contains(&n.1) {
				queue.push_back(n);
			}
		};

		push_if_not_wall_and_not_visited((node.0 + 1, ((node.1).0 - 1, (node.1).1)));
		push_if_not_wall_and_not_visited((node.0 + 1, ((node.1).0 + 1, (node.1).1)));
		push_if_not_wall_and_not_visited((node.0 + 1, ((node.1).0, (node.1).1 - 1)));
		push_if_not_wall_and_not_visited((node.0 + 1, ((node.1).0, (node.1).1 + 1)));
	}
}