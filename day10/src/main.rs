use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::{thread, time};

fn main() -> Result<(), Error> {
	let in_path = "input";
	
	let input = File::open(in_path)?;
	let buffered = BufReader::new(input);
	let mut instructions: Vec<String> = buffered.lines().map(|l| l.unwrap()).collect(); 
	let mut op_clock = 0usize;
	let mut cpu_clock = 1usize;
	let mut reg_x = 1i32;
	//let mut good_sig_strength: i32 = 0; // part 1
	let mut last_op = String::new();
	let mut last_addx_val = 0i32;
	let mut crt_screen: Vec<&str> = vec!["."; 240]; // 40 * 6 total pixels
	instructions.reverse(); // treat input as a stack, so first needs to become last
	while !instructions.is_empty() {
		print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
		if !last_op.is_empty() {
			if last_op.as_str() == "noop" && op_clock == 1 { op_clock = 0; }
			else if  last_op.as_str() == "addx" && op_clock == 2 {
				reg_x += last_addx_val;
				op_clock = 0;
			}
		}
		if op_clock == 0 {
			let instruction = instructions.pop();
			match instruction {
				Some(ins_string) => {
					let ins_as_vec: Vec<&str> = ins_string.split_whitespace().collect();
					if ins_as_vec[0] == "addx" {
						last_addx_val = ins_as_vec[1].parse::<i32>().unwrap();
					}
					last_op = ins_as_vec[0].to_string();
				},
				None => () 
			}
		}
		// update rule for this pixel
		if (cpu_clock % 40) == reg_x as usize || (cpu_clock % 40)  == (reg_x+1) as usize
			                           || (cpu_clock % 40) == (reg_x+2) as usize {
			crt_screen[cpu_clock-1] = "#";
		}
		// draw all the pixels
		for i in 0..6 {
			for j in 0..40 {
				if j == 39 { 
					println!("{}", crt_screen[i as usize * 40usize + j as usize]);
				}
					
				else { print!("{}", crt_screen[i as usize * 40usize + j as usize]); }
			}
		}

		// part 1
		//if cpu_clock % 40 == 20 {
		//	good_sig_strength += cpu_clock as i32 * reg_x;
		//	println!("clock tick: {}, x-register contains: {}", cpu_clock, reg_x);
		//}
		op_clock +=1;
		cpu_clock += 1;
		// sleep so we can actually see the rasterization
		thread::sleep(time::Duration::from_millis(50));
	}
	// part 1
	//println!("last clock tick: {}, x-register contains: {}", cpu_clock, reg_x);
	//println!("total good signal strength is: {}", good_sig_strength);

	Ok(())
}
