use std::io::{self, BufRead};
use std::collections::VecDeque;
use regex::Regex;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Node {
	pos : (usize, usize),
	size : u32,
	used : u32,
	is_payload : bool,
}

impl Node {
	fn avail(&self) -> u32 {
		self.size - self.used
	}
}

fn part1(nodes : &[Node]) {
	let mut viable_pairs = 0;
	for node_a in nodes {
		for node_b in nodes {
			if node_a.pos != node_b.pos && node_a.used != 0 && node_a.used <= node_b.avail() {
				viable_pairs += 1;
			}
		}
	}
	println!("Part 1: {}", viable_pairs);
}

fn dist(dimensions : (usize, usize), mut grid : Vec<bool>, start : (usize, usize), destination : (usize, usize)) -> usize {
	let mut queue = VecDeque::new();

	queue.push_back((0, start));
	while !queue.is_empty() {
		let entry = queue.pop_front().unwrap();
		if grid[(entry.1).1 * dimensions.0 + (entry.1).0] {
			continue;
		} else if entry.1 == destination {
			return entry.0;
		}

		grid[(entry.1).1 * dimensions.0 + (entry.1).0] = true;
		queue.push_back((entry.0 + 1, ((entry.1).0 - 1, (entry.1).1    )));
		queue.push_back((entry.0 + 1, ((entry.1).0 + 1, (entry.1).1    )));
		queue.push_back((entry.0 + 1, ((entry.1).0    , (entry.1).1 - 1)));
		queue.push_back((entry.0 + 1, ((entry.1).0    , (entry.1).1 + 1)));
	}

	panic!("No path found from {:?} to {:?}", start, destination);
}

fn part2(nodes : &[Node]) {
	let bottom_right = nodes.iter().map(|node| node.pos).max().unwrap();
	let wall_threshold = nodes.iter().filter(|node| node.pos.1 == 1).map(|node| node.size).max().unwrap();
	let dimensions = (bottom_right.0 + 2, bottom_right.1 + 2);
	let mut grid = vec![true; (dimensions.0)*(dimensions.1)];

	for node in nodes {
		grid[((node.pos.1)*dimensions.0 + node.pos.0) as usize] = if node.size > wall_threshold { true } else { false };
	}

	let empty_pos = nodes.iter().find(|node| node.used == 0 && node.size > 0).unwrap().pos;
	let payload_pos = (dimensions.0 - 2, 1);
	let dist_to_payload = dist(dimensions, grid.clone(), empty_pos, payload_pos);
	let dist_to_home = dist(dimensions, grid, (payload_pos.0 - 1, payload_pos.1), (1, 1));
	println!("Part 2: {}", dist_to_payload + 5 * dist_to_home);
}

pub fn solve() {
	let stdin = io::stdin();
	let input: Vec<_> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.collect();

	let re = Regex::new(r"node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T").unwrap();
	let mut nodes = Vec::new();
	for line in input.iter().skip(2) {
		let caps = re.captures(line).unwrap();
		nodes.push(Node {
					pos : (caps[1].parse::<usize>().unwrap() + 1, caps[2].parse::<usize>().unwrap() + 1),
					size : caps[3].parse::<u32>().unwrap(),
					used : caps[4].parse::<u32>().unwrap(),
					is_payload : false });
	}

	part1(&nodes);
	part2(&nodes);
}