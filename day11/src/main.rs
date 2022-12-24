extern crate queues;

use queues::*;
use std::fs::File;
use std::io::{Error, BufReader, BufRead};

fn main() -> Result<(), Error> {

	const NUM_ROUNDS: i32 = 10_000;
	const NUM_MONKEYS: i32 = 8;
	let mut monkey_queues: Vec<Queue<i64>> = vec![];
	let mut monkey_num_ops = vec![0i32; NUM_MONKEYS as usize];

	let in_path = "input";
	let input = File::open(in_path)?;
	let buffered = BufReader::new(input);
	let mut lines: Vec<String> = buffered.lines().map(|l| l.unwrap()).collect(); 
	for line in lines {
		if line.contains("Starting") {
			let items: Vec<&str> = line.split_whitespace().collect();
			let mut monkey_queue: Queue<i64> = queue![];
			for item in &items[2..] {
				monkey_queue.add(item[0..2].parse::<i64>().unwrap());
			}
			monkey_queues.push(monkey_queue);
		}
		// do stuff
	}
	for round in 0..NUM_ROUNDS {
		for monkey in 0..NUM_MONKEYS {
			while monkey_queues[monkey as usize].size() != 0 {
				let mut cur_stress = monkey_queues[monkey as usize].remove().unwrap();
				match monkey {
					0 => {
						cur_stress *= 17;
						cur_stress /= 3;
						if cur_stress % 2 == 0 {
							monkey_queues[2].add(cur_stress);
						} else {
							monkey_queues[6].add(cur_stress);
						}
					},
					1 => {
						cur_stress *= cur_stress;
						cur_stress /= 3;
						if cur_stress % 7 == 0 {
							monkey_queues[0].add(cur_stress);
						} else {
							monkey_queues[2].add(cur_stress);
						}
					},
					2 => {
						cur_stress += 7;
						cur_stress /= 3;
						if cur_stress % 13 == 0 {
							monkey_queues[7].add(cur_stress);
						} else {
							monkey_queues[6].add(cur_stress);
						}

					},
					3 => {
						cur_stress += 4;
						cur_stress /= 3;
						if cur_stress % 5 == 0 {
							monkey_queues[4].add(cur_stress);
						} else {
							monkey_queues[5].add(cur_stress);
						}
					},
					4 => {
						cur_stress += 5;
						cur_stress /= 3;
						if cur_stress % 3 == 0 {
							monkey_queues[1].add(cur_stress);
						} else {
							monkey_queues[5].add(cur_stress);
						}

					},
					5 => {
						cur_stress += 6;
						cur_stress /= 3;
						if cur_stress % 19 == 0 {
							monkey_queues[1].add(cur_stress);
						} else {
							monkey_queues[0].add(cur_stress);
						}

					},
					6 => {
						cur_stress *= 13;
						cur_stress /= 3;
						if cur_stress % 11 == 0 {
							monkey_queues[3].add(cur_stress);
						} else {
							monkey_queues[7].add(cur_stress);
						}

					},
					7 => {
						cur_stress += 2;
						cur_stress /= 3;
						if cur_stress % 17 == 0 {
							monkey_queues[4].add(cur_stress);
						} else {
							monkey_queues[3].add(cur_stress);
						}

					},
					_ => (),
				}
				monkey_num_ops[monkey as usize] += 1;
			}
		}
	}
	println!("{:?}", monkey_num_ops);
	Ok(())
}
