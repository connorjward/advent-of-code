use std::collections;
use std::env;
use std::fs;

fn main() {
    match &env::args().collect::<Vec<String>>()[..] {
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

fn do_part1(infile: &String) {
    let mut total = 0;
    for line in fs::read_to_string(infile).unwrap().lines() {
        let (rucksack1, rucksack2) = split_line(line.to_string());
        let common = find_common_letter(&rucksack1, &rucksack2);
        total += item_priority(common);
    }
    println!("Result: {}", total);
}

fn do_part2(infile: &String) {
    let contents = fs::read_to_string(infile).unwrap();
    let mut lines = contents.lines();
    let mut total = 0;
    loop {
        if let Some(line1) = lines.next() {
            let line2 = lines.next().unwrap();
            let line3 = lines.next().unwrap();
            let common = find_common_letter_across_lines(&line1, &line2, &line3);
            total += item_priority(common);
        } else {
            break;
        }
    }
    println!("Result: {}", total);
}

fn find_common_letter_across_lines(line1: &str, line2: &str, line3: &str) -> char {
    let set1: collections::HashSet<char> = line1.to_string().chars().collect();
    let set2: collections::HashSet<char> = line2.to_string().chars().collect();
    let set3: collections::HashSet<char> = line3.to_string().chars().collect();

    let intersection: collections::HashSet<char> = set1.intersection(&set2).cloned().collect();
    let intersection: Vec<char> = intersection.intersection(&set3).cloned().collect();

    match intersection[..] {
        [letter] => letter,
        _ => panic!("Common letter not found"),
    }
}

fn split_line(line: String) -> (String, String) {
    let (a, b) = line.split_at(line.len() / 2);
    (a.to_string(), b.to_string())
}

fn find_common_letter(rucksack1: &String, rucksack2: &String) -> char {
    // find the set intersection between the rucksacks
    let set1: collections::HashSet<char> = rucksack1.to_string().chars().collect();
    let set2: collections::HashSet<char> = rucksack2.to_string().chars().collect();
    match set1.intersection(&set2).collect::<Vec<&char>>()[..] {
        [letter] => *letter,
        _ => panic!("Common letter not found"),
    }
}

fn item_priority(item: char) -> u32 {
    if !item.is_alphabetic() {
        panic!("Needs to be a letter");
    }

    let mut priority = item.to_digit(36).unwrap() - 9;
    if item.is_ascii_uppercase() {
        priority += 26
    }
    priority
}
