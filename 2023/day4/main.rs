use std::collections::HashSet;
use std::convert::TryInto;
use std::iter::FromIterator;

use read_input::read_lines;

mod read_input;


fn part_one() -> () {
	let input = read_lines("./input.txt");
	let mut points = 0;
	for line in input  {
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

fn main() {
	part_one();
}
