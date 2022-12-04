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
    let contents = fs::read_to_string(infile).unwrap();
    for line in contents.lines() {
        println!("{}", line);
    }
}

fn do_part2(infile: &str) {
    let contents = fs::read_to_string(infile).unwrap();
    for line in contents.lines() {
        println!("{}", line);
    }
}
