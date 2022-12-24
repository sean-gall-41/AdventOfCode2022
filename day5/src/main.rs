use std::fs::File;
use std::io::{Error, BufReader, BufRead};

fn main() -> Result<(), Error> {
    let in_path = "input";

    let input = File::open(in_path)?;
    let buffered = BufReader::new(input);

    let mut stacks = Vec::<Vec::<String>>::new();
    for _ in 0..9 {
        stacks.push(vec![]);
    }
    let mut instruction_set = Vec::<Vec::<i32>>::new();
    for line in buffered.lines().map(|l| l.unwrap()) {
        if line.len() == 0 { continue; }
        let line_tokens: Vec<&str> = line.split_whitespace().collect();
        if line_tokens[0] == "move" {
            let mut this_instruction = vec![];
            for (i, l_token) in line_tokens.iter().enumerate() {
                if i % 2 == 1 {
                    this_instruction.push(l_token.parse::<i32>().unwrap());
                } 
            }
            instruction_set.push(this_instruction);
        } else if line_tokens[0] == "1" { 
            for stack in &mut stacks {
                stack.reverse();
            }
            continue;
        } else {
            for (i, crate_candidate) in line_tokens.iter().enumerate() {
                if *crate_candidate != "*" {
                    stacks[i].push(String::from(*crate_candidate));
                }

            }
        }
    }

    for instruction in &instruction_set {
        // part 1
        //for _ in 0..(instruction[0] as usize) {
        //    let mut popped = &stacks[(instruction[1]-1) as usize].pop().unwrap();
        //    &stacks[(instruction[2]-1) as usize].push((*popped).clone());
        //}

        let mut buffer = Vec::new();
        for _ in 0..(instruction[0] as usize) {
            let stack_popped = &stacks[(instruction[1]-1) as usize].pop().unwrap();
            buffer.push((*stack_popped).clone());
        }
        for _ in 0..(instruction[0] as usize) {
            let buf_popped = buffer.pop().unwrap();
            stacks[(instruction[2]-1) as usize].push(buf_popped.to_string().clone());
        }
    }
    for stack in &mut stacks {
        print!("{}", &stack.pop().unwrap());
    }
    println!("");
    Ok(())
}

