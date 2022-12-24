use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::collections::HashMap;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn main() -> Result<(), Error> {
    
    let priorities = hashmap![
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40, 
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52
    ];    

    let in_path = "input";

    let input = File::open(in_path)?;
    let buffered = BufReader::new(input);

    //let mut line_char = '0';
    let mut accum: i32 = 0;
    let mut group: Vec::<&str> = vec![];
    let mut buffered_input = buffered.lines().map(|l| l.unwrap());
    while let line = buffered_input.next() {
        match line {
            Some(string) => {
                let line_next = buffered_input.next().expect("Got None, expected string");
                let line_dub_next = buffered_input.next().expect("Got None, expected string");
                for l_char in string.as_str().chars() {
                    let line_next_match = line_next.as_str().find(|c: char| c == l_char);
                    let line_dub_next_match = line_dub_next.as_str().find(|c: char| c == l_char);
                    if !line_next_match.is_none() && !line_dub_next_match.is_none() {
                        println!("{l_char}");
                        accum += priorities.get(&l_char).copied().unwrap_or(0);
                        break;
                    }
                }
            },
            None => break,
        }
    }
    println!("{accum}");
    Ok(())
}
