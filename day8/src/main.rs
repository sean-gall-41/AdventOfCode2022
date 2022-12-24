use std::fs::File;
use std::io::{Error, BufReader, BufRead};

fn main() -> Result<(), Error> {
    let in_path = "input";

    let input = File::open(in_path)?;
    let buffered = BufReader::new(input);

	let mut trees: Vec<Vec<u8>> = vec![];
    for line in buffered.lines().map(|l| l.unwrap()) {
        let mut row_vals: Vec<u8> = vec![];
		const RADIX: u32 = 10;
		for digit in line.chars() {
			row_vals.push(digit.to_digit(RADIX).unwrap() as u8);
		}
		// was trying to be clever, until realized
		// real puzzle input would *absolutely* produce int overflow,
		// even for i64
		//let powa: u32 = 10;
		//while line_as_num != 0 {
		//	let digit: u8 = (line_as_num % powa) as u8;
		//	row_vals.push(digit);
		//	line_as_num /= powa;
		//}
		//row_vals.reverse();
		trees.push(row_vals);
	}
	// got the trees

	// part 1
	//let mut num_vis: u32 = 2u32 * (trees.len() as u32)  + 2u32 * (trees[0].len() as u32 - 2);
	//for i in 1..(trees.len()-1) {
	//	for j in 1..(trees[i].len()-1) {
	//		let mut bitmask: u8 = 0;
	//		let mut check_up_mask: u8 = 1;
	//		for y in (0..i).rev() {
	//			check_up_mask &= (trees[y][j] < trees[i][j]) as u8;
	//			// check up 
	//		}
	//		bitmask |= check_up_mask;

	//		let mut check_down_mask: u8 = 1;
	//		for y in (i+1)..trees.len() {
	//			check_down_mask &= (trees[y][j] < trees[i][j]) as u8;
	//			// check down
	//		}
	//		bitmask |= check_down_mask;

	//		let mut check_left_mask: u8 = 1;
	//		for x in (0..j).rev() {
	//			check_left_mask &= (trees[i][x] < trees[i][j]) as u8;
	//			// check left 
	//		}
	//		bitmask |= check_left_mask;

	//		let mut check_right_mask: u8 = 1;
	//		for x in (j+1)..trees[i].len() {
	//			check_right_mask &= (trees[i][x] < trees[i][j]) as u8;
	//			// check right 
	//		}
	//		bitmask |= check_right_mask;
	//		num_vis += bitmask as u32;
	//	}
	//}
	//println!("{num_vis}");

	// part 2
	let mut scenic_scores: Vec<u32> = vec![];
	for i in 1..(trees.len()-1) {
		for j in 1..(trees[i].len()-1) {
			let mut this_scenic_score: u32 = 1;
			let mut check_up_score: u32 = 0;
			for y in (0..i).rev() {
				if trees[y][j] >= trees[i][j] {
					check_up_score += 1;
					break;
				}
				check_up_score += 1;
				// check up 
			}
			this_scenic_score *= check_up_score;

			let mut check_down_score: u32 = 0;
			for y in (i+1)..trees.len() {
				if trees[y][j] >= trees[i][j] {
					check_down_score += 1;
					break;
				}
				check_down_score += 1;
				// check down
			}
			this_scenic_score *= check_down_score;

			let mut check_left_score: u32 = 0;
			for x in (0..j).rev() {
				if trees[i][x] >= trees[i][j] {
					check_left_score += 1;
					break;
				}
				check_left_score += 1;
				// check left 
			}
			this_scenic_score *= check_left_score;

			let mut check_right_score: u32 = 0;
			for x in (j+1)..trees[i].len() {
				if trees[i][x] >= trees[i][j] {
					check_right_score += 1;
					break;
				}
				check_right_score += 1;
				// check right 
			}
			this_scenic_score *= check_right_score;
			scenic_scores.push(this_scenic_score);
		}
	}
	println!("{}", scenic_scores.iter().max().unwrap());
	//println!("{:?}", scenic_scores);
	Ok(())
}

