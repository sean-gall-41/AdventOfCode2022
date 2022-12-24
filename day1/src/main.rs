use std::fs::File;
use std::io::{Error, BufReader, BufRead};

fn main() -> Result<(), Error> {
    let in_path = "input";

    let input = File::open(in_path)?;
    let buffered = BufReader::new(input);
    
    let mut elf_calories = vec![];
    let mut accum = 0i32;
    for line in buffered.lines().map(|l| l.unwrap()) {
        if line == String::from("") {
            elf_calories.push(accum);
            accum = 0; 
        }
        else {
            let line_as_i32 = line.parse::<i32>().unwrap();
            accum += line_as_i32;
        }
    }
    elf_calories.sort();
    let top_three_sum = 
    elf_calories[elf_calories.len()-1] 
    + elf_calories[elf_calories.len()-2]
    + elf_calories[elf_calories.len()-3];
    println!("{top_three_sum}"); 
    Ok(())
}

