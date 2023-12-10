use std::collections::HashSet;
use std::iter::FromIterator;

use read_input::read_lines;

mod read_input;

fn part_one() -> () {
	let input = read_lines("./input.txt");
	let mut seeds: Vec<i64> = input[0].split(": ")
									  .nth(1)
									  .unwrap()
									  .split(' ')
									  .map(|x| x.parse::<i64>().unwrap())
									  .collect();
	let mut idx = 1;
	let mut maps: Vec<Vec<String>> = vec![];
	while idx < input.len() {
		if input[idx] == "" {
			idx += 1; // skip map name
			maps.push(vec![]);
		} else {
			let map_idx = maps.len() - 1;
			let mapping = input[idx].clone();
			maps[map_idx].push(mapping);
		}
		idx += 1;
	}
	for map in maps {
		let mut set = HashSet::<_>::from_iter(0..seeds.len());
		for entry in map {
			if let [destination, source, range] = entry.split_whitespace()
									  				.map(|x| x.parse::<i64>().unwrap())
													.collect::<Vec<i64>>()[..] {
				let mut changed = HashSet::<_>::new();
				for i in set.iter().cloned() {
					if seeds[i] >= source && seeds[i] < source + range {
						seeds[i] = seeds[i] - source + destination;
						changed.insert(i);
					}
				}
				set = set.difference(&changed)
						 .map(|x| *x)
						 .collect();
			}
		}
	}
	let closest_location = seeds.iter().min().unwrap();
	println!("{}", closest_location);
}

fn main() {
	part_one();
}
