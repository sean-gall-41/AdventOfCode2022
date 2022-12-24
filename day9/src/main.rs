use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::collections::HashSet;

fn diff(v1: (i32, i32), v2: (i32, i32)) -> (i32, i32) {
	(v2.0 - v1.0, v2.1 - v1.1)
}

fn norm_sq(v: (i32, i32)) -> i32 {
	return i32::pow(v.0, 2) + i32::pow(v.1, 2);
}

fn main() -> Result<(), Error> {
	let in_path = "input";
	
	let input = File::open(in_path)?;
	let buffered = BufReader::new(input);
	let lines: Vec<String> = buffered.lines().map(|l| l.unwrap()).collect();

	let mut head  = (0i32, 0i32);
	let mut tail  = (0i32, 0i32);
	let mut rope: Vec<(i32, i32)> = vec![(0, 0); 10];
	let mut tail_coords: HashSet<(i32, i32)> = HashSet::new();
	tail_coords.insert(rope[rope.len()-1]);

	for line in lines {
		let line_tokens: Vec<&str> = line.split_whitespace().collect();
		let dir_steps = line_tokens[1].parse::<u32>().unwrap();
		for _ in 0..dir_steps {
			println!("{:?}", rope);
			match line_tokens[0] {
				"U" => {
					rope[0].1 += 1;
				},
				"D" => {
					rope[0].1 -= 1;
				},
				"L" => {
					rope[0].0 -= 1;
				},
				"R" => {
					rope[0].0 += 1;
				},
				_ => (),
			}
			for i in 1..rope.len() {
				let diff = diff(rope[i], rope[i-1]);
				if norm_sq(diff) > 2i32 { // need to update tail coords
					match diff { // 8 cases to consider
						(2, 0)   => { rope[i].0 += 1;  rope[i].1 += 0},
						(2, 1)   => { rope[i].0 += 1;  rope[i].1 += 1},
						(2, 2)   => { rope[i].0 += 1;  rope[i].1 += 1},
						(1, 2)   => { rope[i].0 += 1;  rope[i].1 += 1},
						(0, 2)   => { rope[i].0 += 0;  rope[i].1 += 1},
						(-1, 2)  => { rope[i].0 += -1; rope[i].1 += 1},
						(-2, 2)  => { rope[i].0 += -1; rope[i].1 += 1},
						(-2, 1)  => { rope[i].0 += -1; rope[i].1 += 1},
						(-2, 0)  => { rope[i].0 += -1; rope[i].1 += 0},
						(-2, -1) => { rope[i].0 += -1; rope[i].1 += -1},
						(-2, -2) => { rope[i].0 += -1; rope[i].1 += -1},
						(-1, -2) => { rope[i].0 += -1; rope[i].1 += -1},
						(0, -2)  => { rope[i].0 += 0;  rope[i].1 += -1},
						(1, -2)  => { rope[i].0 += 1;  rope[i].1 += -1},
						(2, -2)  => { rope[i].0 += 1;  rope[i].1 += -1},
						(2, -1)  => { rope[i].0 += 1;  rope[i].1 += -1},
						(_, _) => (),
					}
					if i == rope.len()-1 {
						tail_coords.insert(rope[i]);
					}
				}
			}
		}
	}
	println!("{}", tail_coords.len());
	Ok(())
}
