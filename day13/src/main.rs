use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::iter::Peekable;
use std::str::Chars;

// data structure used to parse a signal
#[derive(Debug, Clone)]
enum Node {
    Val(i32),
    List(Vec<Node>),
}

fn fill_node(node: &mut Node, char_iter: &mut Peekable<Chars>) {
    match node {
        Node::List(v) => {
            loop {
                let next = char_iter.next();
                match next {
                    Some(c) => {
                        if c == ',' || char::is_whitespace(c) { continue; }
                        if c == '[' {
                            let mut sub_node = Node::List(vec![]);
                            fill_node(&mut sub_node, char_iter);
                            v.push(sub_node);
                        } else if c == ']' {
                            return;
                        } else if c.is_digit(10) {
                            if c == '1' && char_iter.peek() == Some(&'0') {
                                v.push(Node::Val(10));
                                char_iter.next(); // iterate before next loop iter to move past "10"
                            } else {
                                v.push(Node::Val(c.to_digit(10).unwrap() as i32));
                            }
                        }
                   },
                    None => { break; } // reached end of iterator
                }
            }
        },
        _ => { return; }
    }
}

fn parse_string_into(s: &mut String) -> Node {
    let mut char_iter = s.chars().peekable();
    let mut entry_pt: Node = Node::List(vec![]);
    fill_node(&mut entry_pt, &mut char_iter);
    entry_pt
}

fn ordered(pair: (&mut Node, &mut Node)) -> i32 {
    match pair.0 {
        Node::Val(v_0) => { // first val
            match pair.1 {
                Node::Val(v_1) => { //second val -> compare two vals
                    if *v_1 < *v_0 { return -1; }
                    else if *v_1 == *v_0 { return 0; }
                    else { return 1; }
                },
                Node::List(l_1) => { // second list -> have to convert first to list && try again
                    let mut first_as_vec: Vec<Node> = vec![];
                    first_as_vec.push(Node::Val(*v_0));
                    return ordered((&mut Node::List(first_as_vec), &mut Node::List(l_1.to_vec())));
                }
            }
        },
        Node::List(l_0) => { // first list 
            match pair.1 {
                Node::Val(v_1) => { // second val -> have to convert second to list && try again
                    let mut second_as_vec: Vec<Node> = vec![];
                    second_as_vec.push(Node::Val(*v_1));
                    return ordered((&mut Node::List(l_0.to_vec()), &mut Node::List(second_as_vec)));
                },
                Node::List(l_1) => { // second list -> compare entry by entry
                    let len_0 = l_0.len();
                    let len_1 = l_1.len();
                    for i in 0..i32::min(len_0 as i32, len_1 as i32) {
                        let cmp = ordered((&mut l_0[i as usize], &mut l_1[i as usize]));
                        match cmp {
                            -1 => { return -1; },
                            0 => { continue; },
                            1 => { return 1; },
                            _ => ()
                        }
                    }
                    // getting here means no determining ordering found so far
                    if len_0 < len_1 { return 1; }  // left ran out, so ordered
                    else if len_0 > len_1 { return -1; } // right ran out, so not ordered
                    else { return 0; } // same size, so return and move on to next 
                }
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let path = "input";
	let input = File::open(path)?;
	let buffered = BufReader::new(input);
	let lines: Vec<String> = buffered.lines().map(|l| l.unwrap()).collect(); 
    let mut pairs: Vec<(String, String)> = vec![];
    let mut pair: (String, String) = (String::new(), String::new());

    // pre-process lines into vec of pairs (don't have to do this, can refactor later)
	for line in lines {
        if !pair.0.is_empty() && !pair.1.is_empty() {
            pairs.push(pair.clone());
            pair.0.clear();
            pair.1.clear();
        } else if pair.0.is_empty() {
            pair.0 = line;
        } else if pair.1.is_empty() {
            pair.1 = line;
        }
    }

    // parse pairs into vec of pairs of nodes
    let mut pairs_nodes: Vec<(Node, Node)> = vec![];
    for p in &mut pairs {
        let first = parse_string_into(&mut p.0);
        let second = parse_string_into(&mut p.1);
        pairs_nodes.push((first, second));
    }
    // onto the ordering alg...
    let mut index: i32 = 1;
    let mut sum: i32 = 0;
    for mut p in pairs_nodes {
        let cmp = ordered((&mut p.0, &mut p.1));
        if cmp == 1 { 
            println!("{:?}", p.0);
            println!("{:?}", p.1);
            println!("");
            sum += index; 
        }
        index += 1;
    }
    println!("{sum}");
    Ok(())
}

