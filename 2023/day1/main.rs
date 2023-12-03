use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
	read_to_string(filename)
		.unwrap()			// panic on possible file-reading errors
		.lines()			// split the string into an iterator of string slices
		.map(String::from)	// make each slice into a string
		.collect()			// gather them together into a vector
}

fn main() {
	let input = read_lines("./input");
	let mut sum = 0;

	for line in input{
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

	println!("{}", sum);
}
