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
	println!("part one: {}", closest_location);
}

fn part_two() -> () {
	 let input = read_lines("./input.txt");
//	let input: Vec<String> = vec![
//		"seeds: 79 14 55 13",
//		"",
//		"seed-to-soil map:",
//		"50 98 2",
//		"52 50 48",
//		"",
//		"soil-to-fertilizer map:",
//		"0 15 37",
//		"37 52 2",
//		"39 0 15",
//		"",
//		"fertilizer-to-water map:",
//		"49 53 8",
//		"0 11 42",
//		"42 0 7",
//		"57 7 4",
//		"",
//		"water-to-light map:",
//		"88 18 7",
//		"18 25 70",
//		"",
//		"light-to-temperature map:",
//		"45 77 23",
//		"81 45 19",
//		"68 64 13",
//		"",
//		"temperature-to-humidity map:",
//		"0 69 1",
//		"1 0 69",
//		"",	
//		"humidity-to-location map:",
//		"60 56 37",
//		"56 93 4",
//	].iter().map(|x| x.to_string()).collect();

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
		let mut seed_ranges: Vec<i64> = vec![];
		for entry in map {
			if seeds.len() == 0 {
				continue;
			}
			if let [destination, source, range] = entry.split_whitespace()
									  				.map(|x| x.parse::<i64>().unwrap())
													.collect::<Vec<i64>>()[..] {
				let mut s = 0;
				let mut new_ranges: Vec<i64> = vec![];
				while s < seeds.len() {
					let seed_start = seeds[s];
					let seed_range = seeds[s+1];
					if source <= seed_start && seed_start + seed_range <= source + range {
						seed_ranges.push((seed_start - source) + destination);
						seed_ranges.push(seed_range);
					} else if seed_start < source && source < seed_start + seed_range &&
							  seed_start + seed_range <= source + range {
						seed_ranges.push(destination);
						seed_ranges.push((seed_start + seed_range) - source);
						new_ranges.push(seed_start);
						new_ranges.push(source - seed_start);
					} else if source <= seed_start && seed_start <= source + range &&
							  source + range < seed_start + seed_range {
						seed_ranges.push((seed_start - source) + destination);
						seed_ranges.push((source + range) - seed_start);
						new_ranges.push(source + range);
						new_ranges.push((seed_start + seed_range) - (source + range));
					} else if seed_start < source && source + range < seed_start + seed_range {
						seed_ranges.push(destination);
						seed_ranges.push(range);
						new_ranges.push(seed_start);
						new_ranges.push(source - seed_start);
						new_ranges.push(source + range);
						new_ranges.push((seed_start + seed_range) - (source + range));
					} else {
						new_ranges.push(seed_start);
						new_ranges.push(seed_range);
					}
					s += 2;
				}
				seeds = new_ranges;
			} else { println!("bad"); }
		}
		seeds.extend(seed_ranges);
	}
	if seeds.iter().min() == None {
		println!("something went wrong seeds: {:?}", seeds);
	} else {
		let closest_location = seeds.iter().step_by(2).min().unwrap();
		println!("part two: {}", closest_location);
	}
}

fn main() {
	part_one();
	part_two();
}
