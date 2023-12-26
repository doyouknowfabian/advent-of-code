use read_input::read_lines;

mod read_input;

fn binary_upper(start: i32, end: i32, current: i32, total_time: i32, distance: i32) -> i32 {
	if start >= end {
		let calculated_distance = start * (total_time - start);
		if calculated_distance > distance {
			return start;
		} else {
			return current;
		}
	} else {
		let mid = (end - start) / 2 + start;
		let calculated_distance: i64 = mid as i64 * (total_time - mid) as i64;
		if calculated_distance > distance as i64 {
			return binary_upper(mid + 1, end, mid, total_time, distance);
		} else {
			return binary_upper(start, mid - 1, current, total_time, distance);
		}
	}
}

fn binary_lower(start: i32, end: i32, current: i32, total_time: i32, distance: i32) -> i32 {
	if start >= end {
		let calculated_distance = start * (total_time - start);
		if calculated_distance > distance {
			return start;
		} else {
			return current;
		}
	} else {
		let mid = (end - start) / 2 + start;
		let calculated_distance: i64 = mid as i64 * (total_time - mid) as i64;
		if calculated_distance > distance as i64 {
			return binary_lower(start, mid - 1, mid, total_time, distance);
		} else {
			return binary_lower(mid + 1, end, current, total_time, distance);
		}
	}
}

fn part_one() -> () {
	let input = read_lines("./input.txt");
	let mut times: Vec<i32> = input[0].split(":")
									  .nth(1)
									  .unwrap()
									  .split_whitespace()
									  .map(|x| x.parse::<i32>().unwrap())
									  .collect();
	let mut distances: Vec<i32> = input[1].split(":")
										  .nth(1)
										  .unwrap()
										  .split_whitespace()
										  .map(|x| x.parse::<i32>().unwrap())
										  .collect();
	let races = times.iter().zip(distances.iter());
	let mut margin: i64 = 1;
	for race in races {
		let (time, distance) = race;
		let midpoint = (time/2);
		let upper_bound = binary_upper(midpoint + 1, *time, 0, *time, *distance);
		let lower_bound = binary_lower(0, midpoint, 0, *time, *distance);
		margin *= ((upper_bound - lower_bound) + 1) as i64;
	}
	println!("part one: {}", margin);
}

fn main() {
	part_one();
}
