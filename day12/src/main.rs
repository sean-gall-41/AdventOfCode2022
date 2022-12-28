use std::fs::File;
use std::io::{Error, BufReader, BufRead};

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, Clone)]
struct Node {
    val: i32,
    visited: bool,
    dist: i32,
    coords: (i32, i32),
}

fn main() -> Result<(), Error> {
    let in_path = "input";
    let input = File::open(in_path)?;
    let buffered = BufReader::new(input);
    let lines: Vec<String> = buffered.lines().map(|l| l.unwrap()).collect();
    let mut nodes: Vec<Node> = vec![];
    let mut num_cols: i32 = 0;
    let mut row_num: i32 = 0;
    for l in lines {
        let chars: Vec<i32> = l.chars().map(|c| c as i32 - '0' as i32).collect();
        if num_cols == 0 { num_cols = chars.len() as i32; }
        let mut col_num: i32 = 0;
        for c in chars {
            let node = Node {
                val: c,
                visited: false,
                dist: i32::MAX,
                coords: (row_num, col_num),
            };
            nodes.push(node);
            col_num += 1;
        }
        row_num += 1;
    }
    let mut dists: Vec<i32> = vec![];
    let mut player_id = nodes.iter().position(|n| n.val == 'S' as i32 - '0' as i32).unwrap();
    let init_end_id = nodes.iter().position(|n| n.val == 'E' as i32 - '0' as i32).unwrap();
    nodes[player_id].val = 'a' as i32 - '0' as i32;
    let nodes_clone = nodes.clone();
    for (node_id, node) in nodes_clone.iter().enumerate()
    {
        // my particular input has valid starting points which can reach the end only on the
        // leftmost wall
        if node.val == 'a' as i32 - '0' as i32 && 0 == node_id % (num_cols as usize) {
            player_id = node_id; 
            let mut end_id = init_end_id;
            nodes[player_id].dist = 0;
            let mut found = false;
            while !found {
                let player_dist = nodes[player_id].dist;
                for dir in &DIRS {
                    let neighbor_id = nodes.iter().position(
                        |n| n.coords.0 == nodes[player_id].coords.0 + (*dir).0 && n.coords.1 == nodes[player_id].coords.1 + (*dir).1);
                    match neighbor_id {
                        Some(i) => {
                            let mut compare_val = nodes[i].val;
                            if compare_val == 'E' as i32 - '0' as i32 {
                                compare_val = 'z' as i32 - '0' as i32;
                            }
                            if compare_val <= nodes[player_id].val + 1 {
                                let neighbor_len = player_dist + 1;
                                if nodes[i].dist > neighbor_len { nodes[i].dist = neighbor_len; }
                                if nodes[i].coords.0 == nodes[end_id].coords.0 && nodes[i].coords.1 == nodes[end_id].coords.1 {
                                    dists.push(nodes[i].dist);
                                    found = true;
                                    break;
                                }
                            }
                        },
                        None => { continue; }
                    }
                } 
                nodes[player_id].visited = true;
                if !found {
                    nodes.sort_by(|a,b| a.dist.cmp(&b.dist));
                    for (i, node) in nodes.iter().enumerate() {
                        if !node.visited {
                            player_id = i;
                            break;
                        }
                    }
                }
                end_id = nodes.iter().position(|n| n.val == 'E' as i32 - '0' as i32).unwrap();
            }
            //println!("{:?}", dists);
            nodes = nodes_clone.clone(); // reset nodes for next find
        }
    }
    println!("{}", dists.iter().min().unwrap());
    Ok(())
}

