use std::collections::{HashSet,VecDeque};
use std::env;
use std::fs;

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
    let mut deque: VecDeque<char> = VecDeque::new();
    let contents = fs::read_to_string(infile).unwrap();
    for (i, ch) in contents.chars().enumerate() {
        if deque.len() == 4 {
            deque.pop_front();
        }
        deque.push_back(ch);

        if has_unique_entries(&deque, 4) {
            println!("found at entry: {}", i+1);
            break; 
        }
    }
}

fn do_part2(infile: &str) {
    let mut deque: VecDeque<char> = VecDeque::new();
    let contents = fs::read_to_string(infile).unwrap();
    for (i, ch) in contents.chars().enumerate() {
        if deque.len() == 14 {
            deque.pop_front();
        }
        deque.push_back(ch);

        if has_unique_entries(&deque, 14) {
            println!("found at entry: {}", i+1);
            break; 
        }
    }
}

fn has_unique_entries(deque: &VecDeque<char>, size: usize) -> bool {
    let set = deque.iter().collect::<HashSet<_>>();
    set.len() == size
}
