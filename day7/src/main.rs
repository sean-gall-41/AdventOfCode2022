use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::collections::HashMap;

#[derive(Default)]
struct Dir {
	name: String,
	size: i32,
	children: HashMap<String, Option<Box<Dir>>>,
	files: Vec<(i32, String)>,
}

impl Dir {
	fn new() -> Self {
		Dir {
			name: String::from(""),
			size: 0,
			children: HashMap::<String, Option<Box<Dir>>>::new(),
			files: Vec::<(i32, String)>::new(),
		}
	}
}

impl From<Dir> for Option<Box<Dir>> {
	fn from(dir: Dir) -> Self {
		Some(Box::new(dir)) 
	}
}

#[derive(Default)]
struct FileSystem {
	root: Option<Box<Dir>>,
}

impl FileSystem {
	fn new() -> Self {
		FileSystem { root: None }
	}
	fn insert(&mut self, child: Dir) {
		match &mut self.root {
			None => {
				self.root = child.into();
			},
			Some(_) => ()
		}
	}
}

fn fs_entry_pt(input: &Vec<String>, fs: &mut FileSystem) {
	let mut root = Dir::new();
	root.name = String::from("/");
	fs.insert(root);
	let mut index = 1usize;
	fs_from_input(&mut index, input, &mut fs.root);
	println!("current dir name: {}",fs.root.as_ref().unwrap().name);
	println!("current dir size: {}", fs.root.as_ref().unwrap().size);
}

// alg
// - check if the line has a command
// - if the command is a cd command, check if the current dir has it in its children, then recursve
// on the child
// - if the command is cd and we are given the directory above, we return
// - if the command is ls, loop over the children and add them to the current dir in right attribs
// - if we've reached the end of the input, return

fn fs_from_input(i: &mut usize, input: &Vec<String>, dir: &mut Option<Box<Dir>>) {
	while *i < input.len() {
		let mut line_tokens: Vec<&str> = input[*i].split_whitespace().collect();
		if line_tokens[0] == "$" {
			match line_tokens[1] {
				"cd" => {
					match line_tokens[2] {
						".." => { return; },
						dir_name => {
							match dir {
								Some(dir_ptr) => {
									if !(*dir_ptr).children.contains_key(&(dir_name.to_string())) {
										(*dir_ptr).children.insert(dir_name.to_string(), Some(Box::new(Dir::new())));
									}
									*i += 1;
									fs_from_input(i, input, &mut (*dir_ptr).children.get_mut(&dir_name.to_string()).unwrap());
									(*dir_ptr).size += (*dir_ptr).children
									                             .get(&dir_name.to_string())
									                             .unwrap()
									                             .as_ref()
									                             .unwrap().size;
									//println!("current dir name: {}", (*dir_ptr).name);
									//println!("current dir size: {}", (*dir_ptr).size);
									*i += 1;
								},
								None => ()
							}
						} 
					}
				},
				"ls" => {
					*i += 1;
					line_tokens = input[*i].split_whitespace().collect();
					match dir {
						Some(dir_ptr) => {
							while line_tokens[0] != "$" {
								if line_tokens[0].chars().all(char::is_numeric) {
									(*dir_ptr).size += line_tokens[0].parse::<i32>().unwrap();
									//println!("current dir name: {}", (*dir_ptr).name);
									//println!("current dir size: {}", (*dir_ptr).size);
									(*dir_ptr).files.push((line_tokens[0].parse::<i32>().unwrap(), line_tokens[1].to_string()));
								} else if line_tokens[0] == "dir" {
									let mut child = Dir::new();
									child.name = String::from(line_tokens[1]);
									(*dir_ptr).children.insert(line_tokens[1].to_string(), Some(Box::new(child)));
								}
								*i += 1;
								if *i == input.len() { return; }
								line_tokens = input[*i].split_whitespace().collect();
							}
						},
						None => ()
					}
				},
				&_ => (),
			}
		}
	}
}

fn compute_at_most_sum(sum: &mut i32, dir: &Option<Box<Dir>>) {
	match dir {
		Some(dir_ptr) => {
				if (*dir_ptr).children.is_empty() { return; }
				for (child_name, child) in &(*dir_ptr).children {
					println!("{child_name} has size {}", child.as_ref().unwrap().size);
					//if child.as_ref().unwrap().size < 100000 {
						*sum += child.as_ref().unwrap().size; 
						compute_at_most_sum(sum, child);
					//}
				}
		},
		None => ()
	}
}

// https://www.youtube.com/watch?v=ZPM5xclRInk (general idea tysfm)
// alg 2.0 
// - loop over the lines of the input file
// - for each directory that we change to, push the directory and size total of its
//   direct file children onto the stack
// - if we encounter a cd .., we pop the stack
fn main() -> Result<(), Error> {
	let in_path = "input";
	
	let input = File::open(in_path)?;
	let buffered = BufReader::new(input);

	// attempt 2 or so
	//let lines = buffered.lines().map(|l| l.unwrap()).collect();
	//
	//let mut spaceship_fs = FileSystem::new();
	//fs_entry_pt(&lines, &mut spaceship_fs);
	//let mut size_at_most = 0;
	//compute_at_most_sum(&mut size_at_most, &spaceship_fs.root);
	//println!("{size_at_most}");

	// attempt 3
	let mut sum: u32 = 0;
	let mut paths: Vec<(u32, String)> = vec![];
	let mut paths_part_dos: Vec<(u32, String)> = vec![];
	let lines: Vec<String> = buffered.lines().map(|l| l.unwrap()).collect();
	let mut i: usize = 0;
	while i < lines.len()  {
		let line_tokens: Vec<&str> = lines[i].split_whitespace().collect();
		if line_tokens[0] == "$" && line_tokens[1] == "cd" {
			if paths.is_empty() {
				paths.push((0u32, String::from(line_tokens[2])));
			} else {
				if line_tokens[2] == ".." {
					let latest = paths.pop().unwrap();
					paths_part_dos.push(latest.clone());
					if latest.0 < 100000 { 
						sum += latest.0; 
					}
					let mut next_up = paths.pop().unwrap();
					next_up.0 += latest.0;
					paths.push(next_up);
					println!("after pop, sum: {sum}");

				}
				else {
					let mut path = paths[paths.len()-1].clone();
					path.0 = 0;
					path.1.push_str(line_tokens[2]);
					path.1.push_str("/");
					paths.push(path);
				}
			}
		} else if line_tokens[0] == "$" && line_tokens[1] == "ls" {
			i += 1;
			let mut curr_dir_sum: u32 = 0;
			while i < lines.len() {
				let ls_entry_tokens: Vec<&str> = lines[i].split_whitespace().collect();
				if ls_entry_tokens[0] == "$" && ls_entry_tokens[1] == "cd" {
					i -= 1;
					break;
				} else {
					if ls_entry_tokens[0].chars().all(char::is_numeric) {
						curr_dir_sum += ls_entry_tokens[0].parse::<u32>().unwrap();
					}
				}
				i += 1;
			}
			let mut this_dir = paths.pop().unwrap();
			this_dir.0 = curr_dir_sum;
			paths.push(this_dir);
		}
		i += 1;
	}
	let latest = paths.pop().unwrap();
	paths_part_dos.push(latest.clone());
	if latest.0 < 100000 { 
		sum += latest.0;
	}
	let mut back_to_root = paths.pop().unwrap();
	back_to_root.0 += latest.0;
	paths_part_dos.push(back_to_root.clone());
	
	println!("{sum}");
	println!("{:#?}", paths);
	println!("{:#?}", paths_part_dos);

	let root = paths_part_dos.pop().unwrap();
	let space_root_diff = 70_000_000 - root.0;
	println!("{space_root_diff}");
	let mut del_candidates: Vec<u32> = vec![];
	for dir in paths_part_dos {
		if dir.0 > 30_000_000 - space_root_diff {
			del_candidates.push(dir.0.clone());
		}
	}
	
	println!("{}", del_candidates.iter().min().unwrap());

	Ok(())
}

