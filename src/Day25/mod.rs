use std::io::{self, BufRead};
use std::collections::HashMap;

struct Cpu {
	pc : i32,
	registers : HashMap<String,i32>,
	transmissions : Vec<i32>,
}

impl Cpu {
	fn get_value(&self, val : &str) -> i32 {
		if let Some(ival) = val.parse::<i32>().ok() {
			ival
		} else {
			*self.registers.get(val).unwrap_or(&0)
		}
	}

	fn run_instruction(&mut self, instruction : &[String], program : &mut Vec<Vec<String>>) {
		match instruction[0].as_ref() {
			"cpy" => *self.registers.entry(instruction[2].to_string()).or_insert(0) = self.get_value(&instruction[1]),
			"inc" => *self.registers.entry(instruction[1].to_string()).or_insert(0) += 1,
			"dec" => *self.registers.entry(instruction[1].to_string()).or_insert(0) -= 1,
			"mul" => *self.registers.entry(instruction[1].to_string()).or_insert(0) *= self.get_value(&instruction[2]),
			"jnz" => if self.get_value(&instruction[1]) != 0 { self.pc += self.get_value(&instruction[2]) - 1 },
			"out" => {
				let val = self.get_value(&instruction[1]);
				self.transmissions.push(val);
			},
			"tgl" => {
				let tgl_offset = (self.pc + self.get_value(&instruction[1])) as usize;
				if tgl_offset < program.len() {
					let tgl_instruction = program[tgl_offset][0].clone();
					program[tgl_offset][0] = match tgl_instruction.as_ref() {
						"inc" => "dec",
						"dec" | "tgl" => "inc",
						"cpy" => "jnz",
						"jnz" => "cpy",
						_ => panic!("Unexpected toggled instruction! {}", tgl_instruction),
					}.to_string();
				}
			},
			_ => panic!("Unexpected instruction: {}", instruction[0]),
		}
	}

	fn run_next_instruction(&mut self, mut program : &mut Vec<Vec<String>>) -> bool {
		if self.pc as usize >= program.len() {
			return false;
		}

		let instruction = &program[self.pc as usize].clone();
		self.run_instruction(&instruction.clone(), &mut program);
		self.pc += 1;
		true
	}
}

fn part1(mut program : &mut Vec<Vec<String>>) {

	for a in 0.. {
		let mut cpu = Cpu {
			pc : 0,
			registers : HashMap::new(),
			transmissions : Vec::new(),
		};

		cpu.registers.insert("a".to_string(), a);

		while cpu.run_next_instruction(&mut program) && cpu.transmissions.len() < 20 { }

		if cpu.transmissions.iter().enumerate().all(|(e, &val)| val == (e % 2) as i32) {
			println!("Part 1: {}", a);
			break;
		}
	}
}


pub fn solve() {
	let stdin = io::stdin();
	let mut program: Vec<Vec<_>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split_whitespace().map(String::from).collect::<Vec<_>>())
		.collect();

	part1(&mut program);
}