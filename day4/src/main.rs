use std::fs::File;
use std::io::{Error, BufReader, BufRead};

fn main() -> Result<(), Error> {
    let in_path = "input";

    let input = File::open(in_path)?;
    let buffered = BufReader::new(input);
    
    let mut accum = 0i32;
    for line in buffered.lines().map(|l| l.unwrap()) {
        let pair_split: Vec<&str> = line.split(",").collect();

        let sec_1: Vec<&str> = pair_split[0].split("-").collect();
        let sec_2: Vec<&str> = pair_split[1].split("-").collect();

        let sec_1_low = sec_1[0].parse::<i32>().unwrap();
        let sec_1_high = sec_1[1].parse::<i32>().unwrap();
        let sec_2_low = sec_2[0].parse::<i32>().unwrap();
        let sec_2_high = sec_2[1].parse::<i32>().unwrap();

        // part 1
        //if sec_1_low <= sec_2_low && sec_2_high <= sec_1_high  ||
        //   sec_2_low <= sec_1_low && sec_1_high <= sec_2_high {
        //    accum += 1;
        //    //println!("{sec_1_low} {sec_1_high}, {sec_2_low}, {sec_2_high}");
        //}

        // part 2
        if sec_1_low <= sec_2_high && sec_2_low <= sec_1_high ||
           sec_2_low <= sec_1_high && sec_1_low <= sec_2_high {
            accum += 1;
        }
    }
    println!("{accum}");
    Ok(())
}

