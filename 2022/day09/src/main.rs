use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Copy, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
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
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    let insns = parse_instructions(infile);

    for insn in insns {
        head = shift_head(head, &insn);
        tail = update_tail(tail, head);
        tail_visited.insert(tail);
    }

    println!("Result: {}", tail_visited.len());
}

fn do_part2(infile: &str) {
    let mut snake: Vec<(i32, i32)> = Vec::new();
    for _ in 0..10 {
        snake.push((0, 0));
    }
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    let insns = parse_instructions(infile);

    for insn in insns {
        let head = shift_head(snake[0], &insn);
        snake[0] = head;
        for i in 1..10 {
            snake[i] = update_tail(snake[i], snake[i-1]); 
        }
        tail_visited.insert(snake[9]);
    }

    println!("Result: {}", tail_visited.len());
}

fn parse_instructions(infile: &str) -> Vec<Move> {
    let mut insns: Vec<Move> = vec![];
    let contents = fs::read_to_string(infile).unwrap();
    for line in contents.lines() {
        let (insn, nsteps) = line.split_once(" ").unwrap();
        let nsteps = nsteps.parse::<usize>().unwrap();
        let insn = match insn {
            "U" => Move::Up,
            "D" => Move::Down,
            "L" => Move::Left,
            "R" => Move::Right,
            &_ => panic!("command {} not recognised", insn),
        };
        for _ in 0..nsteps {
            insns.push(insn);
        }
    }
    insns
}

fn print_instructions(insns: &Vec<Move>) {
    for insn in insns.iter() {
        match insn {
            Move::Up => println!("up"),
            Move::Down => println!("down"),
            Move::Left => println!("left"),
            Move::Right => println!("right"),
        }
    }
}

fn shift_head(current: (i32, i32), mov: &Move) -> (i32, i32) {
    match mov {
        Move::Up => (current.0, current.1+1),
        Move::Down => (current.0, current.1-1),
        Move::Left => (current.0-1, current.1),
        Move::Right => (current.0+1, current.1),
    }
}

fn update_tail(current: (i32, i32), head: (i32, i32)) -> (i32, i32) {
    let horiz_shift = current.0 - head.0;
    let vert_shift = current.1 - head.1;

    let is_on_top_of = current.0 == head.0 && current.1 == head.1;
    let is_above_of = current.1 > head.1;
    let is_below_of = current.1 < head.1;
    let is_left_of = current.0 < head.0;
    let is_right_of = current.0 > head.0;

    // don't move if sufficiently close
    if is_on_top_of || tail_is_adjacent_to_head(current, head) {
        current
    } else {
        let mut horiz_new: i32;
        let mut vert_new: i32;

        if is_left_of {
            horiz_new = current.0 + 1;
        } else if is_right_of {
            horiz_new = current.0 - 1;
        } else {
            horiz_new = current.0;
        }

        if is_above_of {
            vert_new = current.1 - 1;
        } else if is_below_of {
            vert_new = current.1 + 1;
        } else {
            vert_new = current.1;
        }
        (horiz_new, vert_new)
    }
}

fn tail_is_adjacent_to_head(tail: (i32, i32), head: (i32, i32)) -> bool {
    let horiz_check = (head.0-1..head.0+2).contains(&tail.0);
    let vert_check = (head.1-1..head.1+2).contains(&tail.1);
    horiz_check && vert_check
}
