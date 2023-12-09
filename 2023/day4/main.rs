use std::collections::HashSet;
use std::collections::HashMap;
use std::convert::TryInto;
use std::iter::FromIterator;

use read_input::read_lines;

mod read_input;


fn part_one() -> () {
	let input = read_lines("./input.txt");
	let mut points = 0;
	for line in input {
		let winning = HashSet::<_>::from_iter(line.split(": ")
													  .nth(1)
													  .unwrap()
													  .split(" | ")
													  .nth(0)
													  .unwrap()
													  .split_whitespace()
													  .map(|x| x.parse::<i32>()
													  .unwrap()));
		let your = HashSet::<_>::from_iter(line.split(": ")
												   .nth(1)
												   .unwrap()
												   .split(" | ")
												   .nth(1)
												   .unwrap()
												   .split_whitespace()
												   .map(|x| x.parse::<i32>()
												   .unwrap()));
		let intersection = your.intersection(&winning)
							   .collect::<HashSet<_>>();
		let power: Option<u32> = if intersection.len() == 0 { None } 
			else {Some((intersection.len() - 1).try_into().unwrap())};
		if power != None {
			points += 2_i32.pow(power.unwrap());
		}
	}
	println!("part one: {}", points);
}

fn part_two() -> () {
	let input = read_lines("./input.txt");
	let mut count = 0;
	let mut counter = HashMap::new();
	for i in 0..input.len() {
		counter.insert(i, 1);
	}
	for (i, line) in input.into_iter().enumerate() {
		count += counter.get(&i).unwrap();
		let winning = HashSet::<_>::from_iter(line.split(": ")
													  .nth(1)
													  .unwrap()
													  .split(" | ")
													  .nth(0)
													  .unwrap()
													  .split_whitespace()
													  .map(|x| x.parse::<i32>()
													  .unwrap()));
		let your = HashSet::<_>::from_iter(line.split(": ")
												   .nth(1)
												   .unwrap()
												   .split(" | ")
												   .nth(1)
												   .unwrap()
												   .split_whitespace()
												   .map(|x| x.parse::<i32>()
												   .unwrap()));
		let intersection = your.intersection(&winning)
							   .collect::<HashSet<_>>();
		for j in i+1..i+1+intersection.len() {
			let amt = counter.get(&j).unwrap() + counter.get(&i).unwrap();
			counter.insert(j, amt);
		}
	}
	println!("part two: {}", count);
}
fn main() {
	part_one();
	part_two();
}
