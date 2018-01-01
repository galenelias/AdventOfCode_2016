#![feature(slice_rotate)]
#![feature(iterator_step_by)]

extern crate clap;
extern crate regex;
extern crate md5;
extern crate permutohedron;

#[macro_use] extern crate lazy_static;
use clap::{Arg,App};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;

fn main() {

	let matches = App::new("Advent of Code")
		.author("Galen Elias, gelias@gmail.com")
		.version("0.1.0")
		.about("Advent of code 2016 solutions in Rust")
		.arg(
			Arg::with_name("day")
				.short("d")
				.required(true)
				.index(1)
				.help("specifies which day's challenge to run")
				.validator(|str|
					str.parse::<u32>()
						.or(Err("day must be an integer".to_owned()))
						.and_then(|v| match v {
							0...25 => Ok(()),
							_ => Err("day must be between 1 and 25".to_owned())
						})))
		.arg(
			Arg::with_name("stats")
				.long("stats")
				.help("Parses leaderboard JSON into a readable format"))
		.after_help("Longer explaination to appear after the options when \
					displaying the help information from --help or -h")
		.get_matches();

	let day = matches.value_of("day").unwrap().parse::<u32>().unwrap();
	match day {
		1 => day1::solve(),
		2 => day2::solve(),
		3 => day3::solve(),
		4 => day4::solve(),
		5 => day5::solve(),
		6 => day6::solve(),
		7 => day7::solve(),
		8 => day8::solve(),
		9 => day9::solve(),
		10 => day10::solve(),
		11 => day11::solve(),
		12 => day12::solve(),
		13 => day13::solve(),
		14 => day14::solve(),
		15 => day15::solve(),
		16 => day16::solve(),
		17 => day17::solve(),
		18 => day18::solve(),
		19 => day19::solve(),
		20 => day20::solve(),
		21 => day21::solve(),
		22 => day22::solve(),
		_ => println!("Oops! Day {} isn't implemented yet!", day)
	}
}
