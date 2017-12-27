use std::io::{self, BufRead};
use std::collections::HashMap;

struct Cpu {
	pc : i32,
	registers : HashMap<String,i32>,
}

impl Cpu {
	fn get_value(&self, val : &str) -> i32 {
		if let Some(ival) = val.parse::<i32>().ok() {
			ival
		} else {
			*self.registers.get(val).unwrap_or(&0)
		}
	}

	fn run_instruction(&mut self, program : &Vec<Vec<String>>) -> bool {
		if self.pc as usize >= program.len() {
			return false;
		}

		let parts = &program[self.pc as usize];
		match parts[0].as_ref() {
			"cpy" => *self.registers.entry(parts[2].to_string()).or_insert(0) = self.get_value(&parts[1]),
			"inc" => *self.registers.entry(parts[1].to_string()).or_insert(0) += 1,
			"dec" => *self.registers.entry(parts[1].to_string()).or_insert(0) -= 1,
			"jnz" => if self.get_value(&parts[1]) != 0 { self.pc += self.get_value(&parts[2]) - 1 },
			_ => panic!("Unexpected instruction: {}", parts[0]),
		}

		self.pc += 1;
		true
	}
}

fn part1(program : &Vec<Vec<String>>) {
	let mut cpu = Cpu {
		pc : 0,
		registers : HashMap::new(),
	};

	while cpu.run_instruction(&program) { }
	println!("Part 1: {}", cpu.registers.get("a").unwrap());
}

fn part2(program : &Vec<Vec<String>>) {
	let mut cpu = Cpu {
		pc : 0,
		registers : HashMap::new(),
	};

	cpu.registers.insert("c".to_string(), 1);

	while cpu.run_instruction(&program) { }
	println!("Part 2: {}", cpu.registers.get("a").unwrap());
}

pub fn solve() {
	let stdin = io::stdin();
	let program: Vec<Vec<_>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split_whitespace().map(String::from).collect::<Vec<_>>())
		.collect();

	part1(&program);
	part2(&program);
}