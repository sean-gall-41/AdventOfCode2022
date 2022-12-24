use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, BufReader, BufRead};

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

// part 1
/* Opponent
 *  A -- Rock
 *  B -- Paper
 *  C -- Scissors
 *
 * You
 *  X -- Rock
 *  Y -- Paper
 *  Z -- Scissors
 *
 */

/* Opponent
 *  A -- Rock
 *  B -- Paper
 *  C -- Scissors
 *
 * You
 *  X -- lose
 *  Y -- draw
 *  Z -- win
 *
 */
fn calc_score(bout: &Vec<&str>, score_map: &HashMap<&str, i32>) -> i32 {
    assert!(bout.len() == 2);
    let mut out = 0;
    let opp_score = score_map.get(&bout[0]).copied().unwrap_or(0); 
    let your_score = score_map.get(&bout[1]).copied().unwrap_or(0); 

    // part 2
    match your_score {
        1 => {
            if opp_score == 1 { out += 3 } // you had to choose scissors
            else {
                out += opp_score - 1;
            }
        },  // you need to lose
        2 => {
            out += opp_score + 3;
        },  // you need to draw
        3 => {
            if opp_score == 3 { out += 1 }
            else {
                out += (opp_score + 1);
            }
            out += 6;
        },  // you need to win
        _ => {}
    }

    // part 1
    //if your_score == 3 && opp_score == 1 {
    //    
    //} else if opp_score < your_score || (your_score == 1 && opp_score == 3) {
    //    out += 6;
    //} else if opp_score == your_score {
    //    out += 3;
    //}
    //out += your_score;
    out
}

fn main() -> Result<(), Error> {
    let in_path = "input";

    let input = File::open(in_path)?;
    let buffered = BufReader::new(input);

    // part 1, both A-C and X-Z are scores, but 
    // in part 2 I use X-Z to identify the case for the 
    // outcome of the bout (X == lose, Y == draw, Z == Win)
    let score_map = hashmap![ "A" => 1, "B" => 2, "C" => 3,
                              "X" => 1, "Y" => 2, "Z" => 3];

    let mut accum = 0i32;
    for line in buffered.lines().map(|l| l.unwrap()) {
        let split: Vec<&str> = line.split(" ").collect();
        println!("{:#?}", split);
        accum += calc_score(&split, &score_map);
    }
    println!("{accum}");
    Ok(())
}
