use std::env;
use std::fs;

enum Instruction {
    Add(i32),
    Null,
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
    let mut result = 0;
    let mut cycle = 0;
    let mut register = 1;
    let contents = fs::read_to_string(infile).unwrap();
    for line in contents.lines() {
        match parse_line(&line) {
            Instruction::Add(count) => {
                for _ in 0..2 {
                    cycle += 1;
                    if (cycle+20) % 40 == 0 {
                        println!("cycle={} reg={} score={}", cycle, register, result);
                        result += cycle * register;
                    }
                }
                register += count;
            },
            Instruction::Null => {
                cycle += 1;
                if (cycle+20) % 40 == 0 {
                    println!("cycle={} reg={} score={}", cycle, register, result);
                    result += cycle * register;
                }
            }
        }
    }
    println!("Result: {}", result);
}

fn do_part2(infile: &str) {
    let mut pixels: Vec<bool> = Vec::new();
    let mut cycle = 0;
    let mut sprite_pos = 1;
    let contents = fs::read_to_string(infile).unwrap();
    for line in contents.lines() {
        match parse_line(&line) {
            Instruction::Add(count) => {
                for _ in 0..2 {
                    pixels.push(sprite_is_overlapping(sprite_pos, cycle));
                    cycle = (cycle+1) % 40;
                }
                sprite_pos += count;
            },
            Instruction::Null => {
                pixels.push(sprite_is_overlapping(sprite_pos, cycle));
                cycle = (cycle+1) % 40;
            }
        }
    }
    assert_eq!(pixels.len(), 240);
    render_image(&pixels); 
}

fn parse_line(line: &str) -> Instruction {
    if line.starts_with("addx") {
        let (_, count) = line.split_once(" ").unwrap();
        Instruction::Add(count.parse::<i32>().unwrap())
    } else {
        Instruction::Null
    }
}

fn render_image(pixels: &Vec<bool>) {
    let nrows = 6;
    let ncols = 40;
    for i in 0..nrows {
        for j in 0..ncols {
            if pixels[i*ncols+j] {
                print!("##");
            } else {
                print!("..");
            }
        }
        println!();
    }
}

fn sprite_is_overlapping(sprite_pos: i32, cycle: i32) -> bool {
    (cycle-1..cycle+2).contains(&sprite_pos)
}
