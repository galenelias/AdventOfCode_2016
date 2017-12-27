use std::io::{self, BufRead};
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
enum ItemType {
	Generator, Microchip
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
struct Item {
	element : char,
	item_type : ItemType,
}

impl Item {
	fn from_string(s : &str) -> Item {
		Item {
			element : s.chars().nth(0).unwrap(),
			item_type : match s.chars().nth(1).unwrap() {
				'M' => ItemType::Microchip,
				'G' => ItemType::Generator,
				_ => panic!("Unexpected item type! {}", s.chars().nth(1).unwrap()),
			}
		}
	}

	fn is_paired_item(&self, other : &Item) -> bool {
		self.element == other.element && self.item_type != other.item_type
	}
}

fn are_items_invalid(items : &[Item]) -> bool {
	// For each generator, see if it's paired, if not, see if there are any unpaired chips to corrupt
	let mut generators = 0;
	let mut unpaired_chips = 0;

	for item1 in items {
		match item1.item_type {
			ItemType::Generator => generators += 1,
			ItemType::Microchip => {
				if !items.iter().any(|item2| item1.is_paired_item(item2)) {
					unpaired_chips += 1;
				}
			}
		}
	}
	unpaired_chips > 0 && generators > 0
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct WorldState {
	steps : usize,
	elevator_pos : usize,
	floors : [Vec<Item>; 4],
}

impl WorldState {
	fn is_done(&self) -> bool {
		self.floors[0].is_empty() && self.floors[1].is_empty() && self.floors[2].is_empty()
	}

	fn is_invalid(&self) -> bool {
		self.floors.iter().any(|floor| are_items_invalid(floor))
	}

	fn min_steps_total(&self) -> usize {
		self.steps + (self.floors[0].len() * 3 + self.floors[1].len() * 2 + self.floors[2].len())/2
	}

	fn canonicalize(&self) -> Vec<(usize,usize)> {
		let mut result = Vec::new();
		for (floor1, items1) in self.floors.iter().enumerate() {
			for item1 in items1.iter().filter(|item| item.item_type == ItemType::Generator) {
				for (floor2, items2) in self.floors.iter().enumerate() {
					if items2.iter().any(|item2| item2.item_type == ItemType::Microchip && item1.element == item2.element) {
						result.push((floor1, floor2));
						break;
					}
				}
			}
		}
		result.sort();
		result.push((self.elevator_pos, 0));
		result
	}
}

impl Ord for WorldState {
	fn cmp(&self, other: &Self) -> Ordering {
		self.min_steps_total().cmp(&other.min_steps_total()).reverse()
	}
}

impl PartialOrd for WorldState {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

pub fn solve() {
	let stdin = io::stdin();

	let input: Vec<Vec<_>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split_whitespace().map(String::from).collect::<Vec<_>>())
		.collect();

	let mut initial_state = WorldState {
		steps : 0,
		elevator_pos : 0,
		floors : [vec![], vec![], vec![], vec![]],
	};

	for (i, line) in input.iter().enumerate() {
		for item in line {
			initial_state.floors[i].push(Item::from_string(item));
		}
	}

	let mut memory = HashSet::new();
	let mut heap = BinaryHeap::new();
	heap.push(initial_state);

	while !heap.is_empty() {
		let state = heap.pop().unwrap();
		let canonicalized_state = state.canonicalize();
		if memory.contains(&canonicalized_state) || state.is_invalid() {
			continue;
		}

		if state.is_done() {
			println!("Found solution. Steps = {}.  Heap size = {}, memory = {}", state.steps, heap.len(), memory.len());
			break;
		}

		// Take every combination of single/double item to adjacent floors
		let items = &state.floors[state.elevator_pos];
		let new_floors : &[usize] = match state.elevator_pos {
			0 => &[1],
			1 => &[0,2],
			2 => &[1,3],
			3 => &[2],
			_ => panic!("Unexpected floor"),
		};

		// Try moving two items to an adjacent level
		for i in 0..items.len() {
			for j in i+1..items.len() {
				for &new_floor in new_floors {
					let mut new_state = state.clone();
					new_state.steps += 1;
					new_state.elevator_pos = new_floor;

					let temp1 = new_state.floors[state.elevator_pos].swap_remove(j);
					let temp2 = new_state.floors[state.elevator_pos].swap_remove(i);
					new_state.floors[new_floor].push(temp1);
					new_state.floors[new_floor].push(temp2);
					if !new_state.is_invalid() && !memory.contains(&new_state.canonicalize()) {
						heap.push(new_state);
					}
				}
			}
		}

		// Try moving only one item to an adjacent level
		for i in 0..items.len() {
			for &new_floor in new_floors {
				let mut new_state = state.clone();
				new_state.steps += 1;
				new_state.elevator_pos = new_floor;
				let temp = new_state.floors[state.elevator_pos].swap_remove(i);
				new_state.floors[new_floor].push(temp);
				if !new_state.is_invalid() && !memory.contains(&new_state.canonicalize()) {
					heap.push(new_state);
				}
			}
		}

		memory.insert(canonicalized_state);
	}

	if heap.is_empty() {
		println!("Warning, heap is empty, likely didn't find the right answer");
	}
}
