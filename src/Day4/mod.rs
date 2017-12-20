use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;

pub fn solve() {
	let stdin = io::stdin();

	let input: Vec<String> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(String::from)
		.collect();

	let mut part1_sum = 0;
	let re = Regex::new(r"(.*)-(\d+)\[(\w{5})\]").unwrap();

	for line in &input {
		let caps = re.captures(line).unwrap();

		let mut freqs = HashMap::new();
		for ch in caps[1].chars().filter(|ch| ch.is_alphabetic()) {
			*freqs.entry(ch).or_insert(0) += 1;
		}

		let mut letters = freqs.iter().collect::<Vec<_>>();
		letters.sort_by(|a,b| if a.1 != b.1 { a.1.cmp(b.1).reverse() } else { a.0.cmp(b.0) });

		let actual_checksum = letters.iter().take(5).map(|freq| freq.0).collect::<String>();

		if caps[3] == actual_checksum {
			let sector_id = caps[2].parse::<u32>().unwrap();
			part1_sum += sector_id;

			let decrypted = caps[1].chars().map(|ch| {
					if ch.is_ascii_lowercase() {
						let cur_shift = ch as u8 - 'a' as u8;
						let new_shift = ((cur_shift as u32 + sector_id) % 26) as u8;
						('a' as u8 + new_shift) as char
					} else {
						ch
					}
				}).collect::<String>();

			if decrypted == "northpole-object-storage" {
				println!("{} in sector {} (part 2)", decrypted, sector_id);
			}
		}
	}
	println!("Part 1: {}", part1_sum);
}