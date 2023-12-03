use std::fs::read_to_string;
use std::collections::HashMap;

/* EXAMPLE INPUT
[
	"two1nine",
	"eightwothree",
	"abcone2threexyz",
	"xtwone3four",
	"4nineeightseven2",
	"zoneight234",
	"7pqrstsixteen",
]; */


fn read_lines(filename: &str) -> Vec<String> {
/* doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html */
	read_to_string(filename)
		.unwrap()			// panic on possible file-reading errors
		.lines()			// split the string into an iterator of string slices
		.map(String::from)	// make each slice into a string
		.collect()			// gather them together into a vector
}

fn part_one() -> () {
	let input = read_lines("./input");
	let mut sum = 0;

	for line in input {
		let mut calibration_value = String::new();
		for c in line.chars() {
			if c.is_digit(10) {
				calibration_value = c.to_string();
				break;
			}
		}
		for c in line.chars().rev() {
			if c.is_digit(10) {
				calibration_value.push_str(&c.to_string());
				break;
			}
		}
		
		sum += calibration_value.parse::<i32>().unwrap();
	}

	println!("part one: {}", sum);
}

fn part_two() -> () {
	let input = read_lines("./input");
	let mut sum = 0;
	let spelled_out = HashMap::from([
		("one", 1),
		("two", 2),
		("three", 3),
		("four", 4),
		("five", 5),
		("six", 6),
		("seven", 7),
		("eight", 8),
		("nine", 9),
	]);

	for line in input {
		let mut word_digits = spelled_out
			.keys()
			.map(|x| (line.find(x), x))
			.filter(|x| x.0 != None)
			.collect::<HashMap<_,_>>();
		word_digits.extend(
			spelled_out
				.keys()
				.map(|x| (line.rfind(x), x))
				.filter(|x| x.0 != None)
				.collect::<HashMap<_,_>>()
		);
		let mut front: Option<_> = None;
		let mut back: Option<_> = None;
		if word_digits.len() > 0 {
			front = *word_digits.keys().min().unwrap();
			back = *word_digits.keys().max().unwrap();
		}

		if front != None && back != None { // digits are spelled out
			let mut calibration_val = spelled_out[word_digits[&front]] * 10;

			// finding first digit
			for (i, c) in line.chars().enumerate() {
				if i >= front.unwrap() {
					break;
				}
				if c.is_digit(10) {
					calibration_val = c.to_string().parse::<i32>().unwrap() * 10;
					break;
				}
			}
			
			// add value of rightmost spelled out digit in case no number found
			calibration_val += spelled_out[word_digits[&back]];
			// finding last digit
			for (i, c) in line.chars().rev().enumerate() {
				let idx = line.len() - i;
				if idx < (back.unwrap() + word_digits[&back].len()) {
					break;
				}
				if c.is_digit(10) {
					calibration_val += c.to_string().parse::<i32>().unwrap();
					calibration_val -= spelled_out[word_digits[&back]];
					break;
				}
			}
			sum += calibration_val;

		} else { // no digits are spelled out
			let mut calibration_val = 0;

			// finding first digit
			for c in line.chars() {
				if c.is_digit(10) {
					calibration_val = c.to_string().parse::<i32>().unwrap() * 10;
					break;
				}
			}
			// finding last digit
			for c in line.chars().rev() {
				if c.is_digit(10) {
					calibration_val += c.to_string().parse::<i32>().unwrap();
					break;
				}
			}
			sum += calibration_val;
		}
	} // end for loop over input
	println!("part two: {}", sum);
}

fn main() {
	part_one();
	part_two();
}
