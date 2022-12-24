use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::collections::HashSet;

fn main() -> Result<(), Error> {
    let in_path = "input";

    let input = File::open(in_path)?;
    let mut buffered = BufReader::new(input);

    let mut raw_line = String::new();
    // part 1: marker_len == 4, part2: marker_len == 14
    let marker_len: usize = 14;
    let mut marker = HashSet::<char>::new();
    buffered.read_line(&mut raw_line).unwrap();
    let line_vec: Vec::<char> = raw_line.chars().collect();
    for (i, _) in line_vec.iter().enumerate() {
        if i + marker_len < line_vec.len() {
            for j in i..(i+marker_len) {
                marker.insert(line_vec[j]);
            }
            if marker.len() == marker_len {
               println!("{}",i+marker_len);
               break;
            }
            marker.clear();
        }
    }
    Ok(())
}
