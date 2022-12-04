use std::env;
use std::fs;

struct Range {
    start: u32,
    stop: u32,
}

impl Range {
    fn from(range_str: &str) -> Range {
        let (start, stop) = range_str.split_once("-").unwrap();
        let start = start.parse::<u32>().unwrap();
        let stop = stop.parse::<u32>().unwrap();
        Range { start, stop }
    }

    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.stop >= other.stop         
    }

    fn overlaps_with(&self, other: &Range) -> bool {
        self.start <= other.start && self.stop >= other.start         
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
    let mut total = 0;
    let contents = fs::read_to_string(infile).unwrap();
    for line in contents.lines() {
        let (a, b) = parse_ranges(line);

        if a.contains(&b) || b.contains(&a) {
            total += 1;
        }
    }
    println!("Result: {}", total);
}

fn do_part2(infile: &str) {
    let mut total = 0;
    let contents = fs::read_to_string(infile).unwrap();
    for line in contents.lines() {
        let (a, b) = parse_ranges(line);

        if a.overlaps_with(&b) || b.overlaps_with(&a) {
            total += 1;
        }
    }
    println!("Result: {}", total);
}

fn parse_ranges(line: &str) -> (Range, Range) {
    let (str1, str2) = line.split_once(",").unwrap();
    (Range::from(&str1), Range::from(&str2))
}
