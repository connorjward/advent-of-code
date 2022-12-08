use std::env;
use std::fs;

struct File {
    name: String,
    size: usize,
}

struct Directory {
    name: String,
    subdirs: Vec<Directory>,
    files: Vec<File>,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory { name: name, subdirs: Vec::new(), files: Vec::new() }
    }

    fn add_subdir(&mut self, dirname: String) {
        let mut subdir = Directory::new(dirname);
        self.subdirs.push(subdir);
    }

    fn get_subdir(&self, dirname: &String) -> &mut Directory {
        for mut subdir in self.subdirs.iter() {
            if subdir.name.as_str() == dirname.as_str() {
                return &mut subdir;
            }
        }
        panic!("Directory not found");
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file); 
    }
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
    // first build the tree
    let mut root = Directory::new(String::from("/"));
    let mut dirstack: Vec<&mut Directory> = Vec::new();
    let mut current_dir = &mut root;
    let contents = fs::read_to_string(infile).unwrap();
    for line in contents.lines() {
        if line == "$ cd /" {
            dirstack.clear();
            let mut current_dir = &mut root;
        } else if line == "$ cd .." {
            let mut current_dir = dirstack.pop().unwrap();
        } else if line.starts_with("$ cd") {
            dirstack.push(&mut current_dir);
            let dirname = line.split(" ").nth(2).unwrap().to_string();
            let mut current_dir = current_dir.get_subdir(&dirname);
        } else if line == "$ ls" {
            continue;
        } else if line.starts_with("dir") {
            let dirname = line.split(" ").nth(1).unwrap().to_string();
            current_dir.add_subdir(dirname);
        } else {  // must be a file
            let split_line = line.split(" ");
            let size = split_line.next().unwrap().parse::<usize>().unwrap();
            let name = split_line.next().unwrap();
            let file = File { name: name.to_string(), size: size };
            current_dir.add_file(file);
        }
    }

    // now traverse and compute
}

fn do_part2(infile: &str) {
    let contents = fs::read_to_string(infile).unwrap();
    for line in contents.lines() {
        println!("{}", line);
    }
}
