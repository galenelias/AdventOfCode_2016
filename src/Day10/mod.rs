use std::io::{self, BufRead};
use std::collections::HashMap;
use std::cell::RefCell;

struct BotInstruction {
	target_types : [String; 2],
	target_nums : [i32; 2],
}

pub fn solve() {
	let stdin = io::stdin();

	let input: Vec<Vec<_>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split_whitespace().map(String::from).collect::<Vec<_>>())
		.collect();

	let mut bots : HashMap<i32, RefCell<Vec<i32>>> = HashMap::new();
	let mut outputs : HashMap<i32, RefCell<Vec<i32>>> = HashMap::new();
	let mut bot_instructions : HashMap<i32, BotInstruction> = HashMap::new();

	for line in input.iter().filter(|l| l[0] == "bot") {
		let bot_name = &line[1].parse::<i32>().unwrap();
		bot_instructions.insert(*bot_name, BotInstruction {
			target_types : [line[5].clone(), line[10].clone()],
			target_nums : [line[6].parse::<i32>().unwrap(), line[11].parse::<i32>().unwrap()],
		});
		bots.insert(*bot_name, RefCell::new(Vec::new()));
	}

	// Push initial values into bots
	for line in input.iter().filter(|l| l[0] == "value") {
		let val = line[1].parse::<i32>().unwrap();
		let bot_name = line[5].parse::<i32>().unwrap();
		bots.get(&bot_name).unwrap().borrow_mut().push(val);
	}

	// Loop through bot transfer instructions until there are no more bots with work to do
	while bots.iter().any(|(_k, chips)| chips.borrow().len() == 2) {
		for (name, chips_ref) in bots.iter().filter(|&(_k, chips)| chips.borrow().len() == 2) {
			let mut chips = chips_ref.borrow_mut();
			chips.sort();

			if *chips == [17, 61] {
				println!("Part 1: {}", name);
			}

			let instruction = bot_instructions.get(&name).unwrap();
			for i in 0..2 {
				let dest = match instruction.target_types[i].as_ref() {
					"bot" => bots.get(&instruction.target_nums[i]).unwrap(),
					"output" => outputs.entry(instruction.target_nums[i]).or_insert(RefCell::new(Vec::new())),
					_ => panic!("Unexpected destination token: {}", instruction.target_types[i]),
				};
				dest.borrow_mut().push(chips.swap_remove(0));
			}
		}
	}

	println!("Part 2: {}", outputs[&0].borrow()[0] * outputs[&1].borrow()[0] * outputs[&2].borrow()[0]);
}
