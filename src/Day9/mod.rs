use std::io::{self, BufRead};
use regex::Regex;

fn decompress(mut input : &str, recursive : bool) -> String {
	let mut result = String::new();
	lazy_static! {
			static ref REPEAT_REGEX : Regex = Regex::new(r"([^(]*)\((\d+)x(\d+)\)").unwrap();
		}

	while !input.is_empty() {
		if let Some(m) = REPEAT_REGEX.captures(input) {
			let rep_len = m.get(2).unwrap().as_str().parse::<usize>().unwrap();
			let rep_count = m.get(3).unwrap().as_str().parse::<usize>().unwrap();
			let match_end = m.get(0).unwrap().end();

			// Push the prefix into the result
			result.push_str(m.get(1).unwrap().as_str());
			if !recursive {
				result.push_str(&input[match_end..match_end+rep_len].repeat(rep_count));
			} else {
				result.push_str(&decompress(&input[match_end..match_end+rep_len], true).repeat(rep_count));
			}
			input = &input[match_end+rep_len..];
		} else {
			result.push_str(input);
			return result;
		}
	}

	return result;
}

pub fn solve() {
	let stdin = io::stdin();

	let input: Vec<_> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.collect();

	let uncompressed_byte_count_part1 = input.iter()
		.map(|s| decompress(s, false /*recursive*/).len()).sum::<usize>();

	let uncompressed_byte_count_part2 = input.iter()
		.map(|s| decompress(s, true /*recursive*/).len()).sum::<usize>();

	println!("Part 1: {}", uncompressed_byte_count_part1);
	println!("Part 2: {}", uncompressed_byte_count_part2);
}
