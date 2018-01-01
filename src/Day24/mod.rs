use std::io::{self, BufRead};
use std::collections::VecDeque;
use permutohedron::LexicalPermutation;

// Find the distance through the maze between 'from' and 'to' using simple Breadth First Search
fn dist(mut grid : Vec<Vec<char>>, from : (usize, usize), to : (usize, usize)) -> usize {
	let mut queue = VecDeque::new();
	queue.push_back((0, from));

	while !queue.is_empty() {
		let q = queue.pop_front().unwrap();
		if q.1 == to {
			return q.0;
		} else if grid[(q.1).0][(q.1).1] == '#' {
			continue;
		}

		grid[(q.1).0][(q.1).1] = '#';

		queue.push_back((q.0 + 1, ((q.1).0 - 1, (q.1).1)));
		queue.push_back((q.0 + 1, ((q.1).0 + 1, (q.1).1)));
		queue.push_back((q.0 + 1, ((q.1).0, (q.1).1 - 1)));
		queue.push_back((q.0 + 1, ((q.1).0, (q.1).1 + 1)));
	}
	panic!("Didn't find path!")
}

struct Node {
	ch : char,
	pos : (usize, usize),
}

fn shortest_route(grid : &Vec<Vec<char>>, nodes : &Vec<Node>, return_to_zero : bool) -> usize {
	let mut distances = vec![vec![0; nodes.len()]; nodes.len()];
	for i in 0..nodes.len() {
		for j in i+1..nodes.len() {
			let d = dist(grid.clone(), nodes[i].pos, nodes[j].pos);
			distances[i][j] = d;
			distances[j][i] = d;
		}
	}

	let mut path = (0..nodes.len()).collect::<Vec<_>>();

	let mut min_dist = None;
	loop {
		let mut dist = 0;
		for i in 0..path.len()-1 {
			dist += distances[path[i]][path[i+1]];
		}

		if return_to_zero {
			dist += distances[*path.last().unwrap()][0];
		}

		if min_dist.is_none() || dist < min_dist.unwrap() {
			min_dist = Some(dist);
		}

		if !path.next_permutation() || path[0] != 0 {
			break;
		}
	}

	min_dist.unwrap()
}

pub fn solve() {
	let stdin = io::stdin();
	let input: Vec<Vec<_>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.chars().collect::<Vec<_>>())
		.collect();

	let mut nodes = Vec::new();
	for (r, row) in input.iter().enumerate() {
		for (c, ch) in row.iter().enumerate() {
			if ch.is_digit(10) {
				nodes.push( Node { ch : *ch, pos : (r, c)});
			}
		}
	}
	nodes.sort_by(|a,b| a.ch.cmp(&b.ch));

	println!("Part 1: {}", shortest_route(&input, &nodes, false /*return_to_zero*/));
	println!("Part 2: {}", shortest_route(&input, &nodes, true /*return_to_zero*/));
}