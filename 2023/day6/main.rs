use read_input::read_lines;

mod read_input;

fn binary_upper(start: i64, end: i64, current: i64, total_time: i64, distance: i64) -> i64 {
	if start >= end {
		let calculated_distance = start * (total_time - start);
		if calculated_distance > distance {
			return start;
		} else {
			return current;
		}
	} else {
		let mid = (end - start) / 2 + start;
		let calculated_distance = mid * (total_time - mid);
		if calculated_distance > distance {
			return binary_upper(mid + 1, end, mid, total_time, distance);
		} else {
			return binary_upper(start, mid - 1, current, total_time, distance);
		}
	}
}

fn binary_lower(start: i64, end: i64, current: i64, total_time: i64, distance: i64) -> i64 {
	if start >= end {
		let calculated_distance = start * (total_time - start);
		if calculated_distance > distance {
			return start;
		} else {
			return current;
		}
	} else {
		let mid = (end - start) / 2 + start;
		let calculated_distance = mid * (total_time - mid);
		if calculated_distance > distance {
			return binary_lower(start, mid - 1, mid, total_time, distance);
		} else {
			return binary_lower(mid + 1, end, current, total_time, distance);
		}
	}
}

fn part_one() -> () {
	let input = read_lines("./input.txt");
	let times: Vec<i64> = input[0].split(":")
									  .nth(1)
									  .unwrap()
									  .split_whitespace()
									  .map(|x| x.parse::<i64>().unwrap())
									  .collect();
	let distances: Vec<i64> = input[1].split(":")
										  .nth(1)
										  .unwrap()
										  .split_whitespace()
										  .map(|x| x.parse::<i64>().unwrap())
										  .collect();
	let races = times.iter().zip(distances.iter());
	let mut margin: i64 = 1;
	for race in races {
		let (time, distance) = race;
		let midpoint = time/2;
		let upper_bound = binary_upper(midpoint + 1, *time, 0, *time, *distance);
		let lower_bound = binary_lower(0, midpoint, 0, *time, *distance);
		margin *= (upper_bound - lower_bound) + 1;
	}
	println!("part one: {}", margin);
}

fn part_two() -> () {
	let input = read_lines("./input.txt");
	let times: Vec<&str> = input[0].split(":")
									  .nth(1)
									  .unwrap()
									  .split_whitespace()
									  .collect();
	let distances: Vec<&str> = input[1].split(":")
										  .nth(1)
										  .unwrap()
										  .split_whitespace()
										  .collect();
	let overall_time: i64 = times.join("")
								 .parse::<i64>()
								 .unwrap();
	let overall_distance: i64 = distances.join("")
								 .parse::<i64>()
								 .unwrap();
	let midpoint = overall_time/2;
	let upper_bound = binary_upper(midpoint + 1, overall_time, 0, overall_time, overall_distance);
	let lower_bound = binary_lower(0, midpoint, 0, overall_time, overall_distance);
	println!("part two: {}", (upper_bound - lower_bound) + 1);
}

fn main() {
	part_one();
	part_two();
}
