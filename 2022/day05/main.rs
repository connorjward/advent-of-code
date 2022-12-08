use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    match &args[..] {
        [_, part, infile, nstacks] => {
            if part == "1" {
                do_part1(infile, nstacks);
            } else if part == "2" {
                do_part2(infile, nstacks);
            } else {
                panic!("Part not valid");
            }
        }
        _ => panic!("Args wrong length"),
    }
}

fn do_part1(infile: &str, nstacks: &str) {
    let contents = fs::read_to_string(infile).unwrap();
    let mut lines = contents.lines();

    let nstacks: usize = nstacks.parse::<usize>().unwrap();
    let mut stacks: Vec<Vec<char>> = vec![];
    for i in 0..nstacks {
        stacks.push(vec![]);
    }

    // prepare the stacks
    loop {
        let line = lines.next().unwrap();
        if line.starts_with(" 1 ") {
            break;
        }
        push_line_to_stacks(&mut stacks, &line);
    }

    // need to reverse the stacks
    for i in 0..nstacks {
        stacks[i].reverse();
    }

    // skip blank line
    lines.next();  

    // now handle the moves
    for line in lines {
        let (count, from, to) = parse_move(&line);
        move_crates(&mut stacks, count, from, to);
    }

    // now show the final result
    for i in 0..nstacks {
        print!("{}", stacks[i].last().unwrap());
    }
    println!();
}

fn push_line_to_stacks(stacks: &mut Vec<Vec<char>>, line: &str) {
    let mut pattern = String::from(r"(   |\[[[:alpha:]]\])");
    for _ in 1..stacks.len() {
        // match with either a three spaces or a crate
        pattern.push_str(r" (   |\[[[:alpha:]]\])");
    }
    let re = Regex::new(&pattern).unwrap();
    let caps = re.captures(line).unwrap();

    assert_eq!(caps.len(), stacks.len()+1);
    for i in 1..caps.len() {
        let mat = caps.get(i).unwrap().as_str();
        if !mat.trim().is_empty() {
            stacks[i-1].push(mat.chars().nth(1).unwrap());
        }
    }
}

fn parse_move(line: &str) -> (usize, usize, usize) {
    let pattern = String::from(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)");
    let re = Regex::new(&pattern).unwrap();
    let caps = re.captures(line).unwrap();
    (caps["count"].parse::<usize>().unwrap(), caps["from"].parse::<usize>().unwrap(), caps["to"].parse::<usize>().unwrap())
}

fn move_crates(stacks: &mut Vec<Vec<char>>, count: usize, from: usize, to: usize) {
    for _ in 0..count {
        let item = stacks[from-1].pop().unwrap();
        stacks[to-1].push(item);
    }
}

fn move_crates_batched(stacks: &mut Vec<Vec<char>>, count: usize, from: usize, to: usize) {
    let start = stacks[from-1].len() - count;
    let mut items: Vec<_> = stacks[from-1].drain(start..).collect();
    stacks[to-1].append(&mut items);
}

fn do_part2(infile: &str, nstacks: &str) {
    let contents = fs::read_to_string(infile).unwrap();
    let mut lines = contents.lines();

    let nstacks: usize = nstacks.parse::<usize>().unwrap();
    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..nstacks {
        stacks.push(vec![]);
    }

    // prepare the stacks
    loop {
        let line = lines.next().unwrap();
        if line.starts_with(" 1 ") {
            break;
        }
        push_line_to_stacks(&mut stacks, &line);
    }

    // need to reverse the stacks
    for i in 0..nstacks {
        stacks[i].reverse();
    }

    // skip blank line
    lines.next();  

    // now handle the moves
    for line in lines {
        let (count, from, to) = parse_move(&line);
        move_crates_batched(&mut stacks, count, from, to);
    }

    // now show the final result
    for i in 0..nstacks {
        print!("{}", stacks[i].last().unwrap());
    }
    println!();
}
