use std::cmp;
use std::collections::HashMap;

use read_input::read_lines;

mod read_input;


fn is_symbol(c: char) -> bool {
	!c.is_digit(10) && c != '.'
}

fn is_part_number(input: &Vec<String>, start: usize, end: usize, line_num: usize, total_lines: usize) -> bool {
	let min = if line_num == 0 { 0 } else { cmp::max(line_num-1, 0) };
	for i in min..cmp::min(line_num+2, total_lines) {
		let line: Vec<char> = (*input[i]).chars().collect();
		let line_start = if start == 0 { 0 } else { cmp::max(start-1, 0) };
		for j in line_start..cmp::min(end+1, line.len()) {
			let c = line[j];
			if is_symbol(c) {
				return true;
			}
		}
	}
	false
}

fn get_gears(
	input: &Vec<String>, start: usize, end: usize, line_num: usize, total_lines: usize
) -> Option<String> {
	let min = if line_num == 0 { 0 } else { cmp::max(line_num-1, 0) };
	for i in min..cmp::min(line_num+2, total_lines) {
		let line: Vec<char> = (*input[i]).chars().collect();
		let line_start = if start == 0 { 0 } else { cmp::max(start-1, 0) };
		for j in line_start..cmp::min(end+1, line.len()) {
			let c = line[j];
			if c == '*' {
				let mut loc = i.to_string();
				loc.push(',');
				loc.push_str(&j.to_string());
				return Some(loc);
			}
		}
	}
	None
}

fn part_one() -> () {
	let input = read_lines("./input.txt");
	/* let input: Vec<String> =  [
	    "467..114..",
	    "...*......",
	    "..35..633.",
  		"......#...",
	    "617*......",
	    ".....+.58.",
	    "..592.....",
	    "......755.",
	    "...$.*....",
	    ".664.598..",
 	].map(String::from).to_vec(); */
	let mut sum = 0;
	for (l, line) in input.iter().enumerate() {
		let mut i = 0;
		let n = line.len();
		let line_chars: Vec<char> = line.chars().collect();
		while i < n {
			let c = line_chars[i];
			let mut num = c.to_string();
			if c.is_digit(10) {
				let mut j = i + 1;
				while j < n && line_chars[j].is_digit(10) {
					num.push_str(&line_chars[j].to_string());
					j += 1;
				}
				if is_part_number(&input, i, j, l, input.len()) {
					sum += num.parse::<i32>().unwrap();
				}
				i = j;
			} else {
				i += 1;
			}
		}
	}
	println!("part one: {}", sum);
}

fn part_two() -> () {
	let input = read_lines("./input.txt");
	let mut sum = 0;
	let mut gears: HashMap<String, Vec<String>> = HashMap::new();
	for (l, line) in input.iter().enumerate() {
		let mut i = 0;
		let n = line.len();
		let line_chars: Vec<char> = line.chars().collect();
		while i < n {
			let c = line_chars[i];
			let mut num = c.to_string();
			if c.is_digit(10) {
				let mut j = i + 1;
				while j < n && line_chars[j].is_digit(10) {
					num.push_str(&line_chars[j].to_string());
					j += 1;
				}
				let gear = get_gears(&input, i, j, l, input.len());
				if gear != None {
					let location = gear.unwrap();
					let gear_list: Vec<String> = if gears.get(&location) != None {
						let mut list = gears.get(&location).unwrap().to_vec();
						list.push(num);
						list
					} else { vec![num] };
					gears.insert(location, gear_list);
				}
				i = j;
			} else {
				i += 1;
			}
		}
	}
	for (_, gear_list) in gears {
		if gear_list.len() == 2 {
			let ratio = gear_list.into_iter().map(|x| x.parse::<i32>().unwrap())
								 .reduce(|acc, e| acc * e).unwrap();
			sum += ratio;
		}
	}
	println!("part two: {}", sum);
}

fn main() {
	part_one();
	part_two();
}
