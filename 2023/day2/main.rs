use std::collections::HashMap;

use read_input::read_lines;

mod read_input;


/* EXAMPLE INPUT
[
	"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
	"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
	"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
	"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
	"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
]; */

fn part_one() -> () {
	let input = read_lines("./input.txt");
	let mut sum = 0;
	let possible_amounts = HashMap::from([
		("red", 12),
		("green", 13),
		("blue", 14),
	]);
	
	for line in input {
		let id = line.split(": ")
					 .nth(0)
					 .unwrap()
					 .split(' ')
					 .nth(1)
					 .unwrap()
					 .parse::<i32>()
					 .unwrap();
		let cube_groups = line.split(": ")
							  .nth(1)
							  .unwrap()
							  .split("; ");
		sum += id;
		for group in cube_groups {
			let cubes = group.split(", ");
			let mut broke = false;
			for cube in cubes {
				let amt = cube.split(" ")
							  .nth(0)
							  .unwrap()
							  .parse::<i32>()
							  .unwrap();
				let color = cube.split(" ")
								.nth(1)
								.unwrap();
				if possible_amounts[color] < amt {
					broke = !broke;
					sum -= id;
					break;
				}
			}
			if broke {
				break;
			}
		}
	}

	println!("part one: {sum}");
}

fn main() {
	part_one();	
}
