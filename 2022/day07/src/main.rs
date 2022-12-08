use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

struct Directory {
    name: String,
    files: HashMap<String, File>,
    subdir_names: HashSet<String>,
}

struct File {
    name: String,
    size: usize,
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    match &args[..] {
        [_, part, infile] => {
            if part == "1" {
                do_part1(infile);
            } else if part == "2" {
                do_part2(infile);
            } else {
                panic!("Part not valid");
            }
        }
        _ => panic!("Args wrong length"),
    }
}

fn do_part1(infile: &str) {
    let mut score = 0;
    let mut dirs = parse_commands(infile);
    for dirstack in dirs.keys() {
        let size = compute_dir_size(&dirs, &dirstack);
        if size <= 100000 {
            score += size;
        }
    }
    println!("Result: {}", score);
}

fn do_part2(infile: &str) {
    let max_space = 70000000 - 30000000;
    let mut dirs = parse_commands(infile);
    let rootkey = vec![String::from("/")];
    let amount_to_delete = compute_dir_size(&dirs, &rootkey) - max_space;

    let mut smallest_valid_dir_to_delete = max_space;
    for dirstack in dirs.keys() {
        let dirsize = compute_dir_size(&dirs, &dirstack);
        if dirsize >= amount_to_delete && dirsize < smallest_valid_dir_to_delete {
            smallest_valid_dir_to_delete = dirsize;
        }
    }
    println!("Result: {}", smallest_valid_dir_to_delete);
}

fn parse_commands(infile: &str) -> HashMap<Vec<String>, Directory> {
    let root = Directory {
        name: String::from("/"),
        files: HashMap::new(),
        subdir_names: HashSet::new(),
    };
    // dir name -> Directory
    let mut dirs: HashMap<Vec<String>, Directory> = HashMap::new();
    let mut dirstack = vec![root.name.clone()];
    dirs.insert(dirstack.clone(), root);
    let contents = fs::read_to_string(infile).unwrap();

    for line in contents.lines() {
        if line.starts_with("$ cd") {
            let dirname = line.split(" ").nth(2).unwrap();
            let dirname = String::from(dirname);
            if dirname == ".." {
                dirstack.pop();
            } else if dirname == "/" {
                dirstack.drain(1..);
            } else {
                // if !dirs.contains_key(&dirname) {
                //     panic!("{} not found but we want to cd into it", dirname);
                // } else if !dirs[&current_dirname].subdir_names.contains(&dirname) {
                //     panic!("{} not registered as a subdim", dirname);
                // }
                dirstack.push(dirname.clone());
            }
        } else if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("dir") {
            let subdirname = line.split_once(" ").unwrap().1.to_string();
            let mut substack = dirstack.clone();
            substack.push(subdirname.clone());
            if !dirs.contains_key(&substack) {
                let subdir = Directory {
                    name: subdirname.clone(),
                    files: HashMap::new(),
                    subdir_names: HashSet::new(),
                };
                dirs.insert(substack, subdir);
                // add to list of subdirs for the current directory
                dirs.get_mut(&dirstack).unwrap().subdir_names.insert(subdirname.clone());
            }
        } else {  // must be a file
            let (size, name) = line.split_once(" ").unwrap();
            let name = name.to_string();
            let size = size.parse::<usize>().unwrap();

            let mut files = &mut dirs.get_mut(&dirstack).unwrap().files;
            if !files.contains_key(&name) {
                let file = File { name: name.clone(), size: size };
                files.insert(name, file);
            }
        }
    }
    dirs
}

fn compute_dir_size(dirs: &HashMap<Vec<String>, Directory>, dirstack: &Vec<String>) -> usize {
    let mut size = 0;
    for subdir_name in dirs[dirstack].subdir_names.iter() {
        let mut substack = dirstack.clone();
        substack.push(subdir_name.clone());
        size += compute_dir_size(dirs, &substack);
    }
    for file in dirs[dirstack].files.values() {
        size += file.size;
    }
    size
}

fn print_dirs(dirs: &HashMap<Vec<String>, Directory>) {
    for (name, dir) in dirs.iter() {
        println!("{}: ({})", name.last().unwrap(), compute_dir_size(dirs, &name));
        for subdir in dir.subdir_names.iter() {
            println!("    dir {}", subdir);
        }
        for file in dir.files.values() {
            println!("    {} ({})", file.name, file.size);
        }
    }
}
