use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
/* doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html */
	read_to_string(filename)
		.unwrap()			// panic on possible file-reading errors
		.lines()			// split the string into an iterator of string slices
		.map(String::from)	// make each slice into a string
		.collect()			// gather them together into a vector
}

