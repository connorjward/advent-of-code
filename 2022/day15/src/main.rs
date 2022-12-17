use std::cmp;
use std::env;
use std::fs;

use regex::Regex;

struct Coord {
    x: i32,
    y: i32,
}

struct Entry {
    sensor: Coord,
    beacon: Coord,
}

struct Range {
    min: i32,
    max: i32,
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
    let contents = fs::read_to_string(infile).unwrap();
    let entries = parse_input(&contents);

    // loop over the target row and only include entries that do not lie
    // in the area covered by any of the sensors
    let mut overlaps: Vec<Range> = vec![];
    let target: i32;
    if infile == "demo.txt" {
        target = 10;
    } else {
        target = 2000000;
    }
    for entry in entries.iter() {
        if let Some(overlap) = covering(entry, target) {
            overlaps.push(overlap);
        }
    }

    'start: loop {
        // if an overlap is found then replace the entry with the new one
        // do until no overlaps are found
        // println!("length {}", overlaps.len());
        for i1 in 0 .. overlaps.len() {
            for i2 in (i1+1) .. overlaps.len() {
                // println!("i1: {}, i2: {}", i1, i2);
                if let Some(overlap) = ranges_overlap(&overlaps[i1], &overlaps[i2]) {
                    overlaps.remove(i1); 
                    overlaps.remove(i2-1);
                    overlaps.push(overlap);
                    continue 'start;
                }
            }
        }
        break;
    }

    let mut result = 0;
    for overlap in overlaps {
        result += overlap.max - overlap.min;
    }
    println!("Result: {}", result);
}

fn do_part2(infile: &str) {
    let contents = fs::read_to_string(infile).unwrap();
    let entries = parse_input(&contents);

    let nrows: usize;
    let ncols: usize;
    if infile == "demo.txt" {
        nrows = 20;
        ncols = 20;
    } else {
        nrows = 4000000;
        ncols = 4000000;
    }

    for row in 0 .. ncols {
        let mut overlaps: Vec<Range> = vec![];
        for entry in entries.iter() {
            if let Some(overlap) = covering(entry, row as i32) {
                overlaps.push(overlap);
            }
        }

        'start: loop {
            // if an overlap is found then replace the entry with the new one
            // do until no overlaps are found
            // println!("length {}", overlaps.len());
            for i1 in 0 .. overlaps.len() {
                for i2 in (i1+1) .. overlaps.len() {
                    // println!("i1: {}, i2: {}", i1, i2);
                    if let Some(overlap) = ranges_overlap(&overlaps[i1], &overlaps[i2]) {
                        overlaps.remove(i1); 
                        overlaps.remove(i2-1);
                        overlaps.push(overlap);
                        continue 'start;
                    }
                }
            }
            break;
        }

        if overlaps.len() > 1 {
            for overlap in overlaps.iter() {
                println!("row: {} -> min: {}, max: {}", row, overlap.min, overlap.max);
            }
        }
    }
}

fn is_covered(entries: &Vec<Entry>, x: i32, y: i32) -> bool {
    let pos = Coord { x, y };
    for entry in entries.iter() {
        let r = distance_between(&entry.sensor, &entry.beacon);
        let d = distance_between(&entry.sensor, &pos);
        if d <= r {
            return true;
        }
    }
    false
}

fn parse_input(contents: &str) -> Vec<Entry> {
    let mut entries: Vec<Entry> = vec![];
    let pattern = String::from(
        r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)"
    );
    let re = Regex::new(&pattern).unwrap();
    for line in contents.lines() {
        let caps = re.captures(line).unwrap();
        let sensor = Coord {
            x: caps["sx"].parse::<i32>().unwrap(),
            y: caps["sy"].parse::<i32>().unwrap(),
        };
        let beacon = Coord {
            x: caps["bx"].parse::<i32>().unwrap(),
            y: caps["by"].parse::<i32>().unwrap(),
        };
        let entry = Entry { sensor, beacon };
        entries.push(entry);
    }
    entries
}

fn ranges_overlap(a: &Range, b: &Range) -> Option<Range> {
    if a.max+1 < b.min || b.max+1 < a.min {
        None
    } else {
        let min = cmp::min(a.min, b.min);
        let max = cmp::max(a.max, b.max);
        let overlap = Range { min, max };
        Some(overlap)
    }
}

fn covering(entry: &Entry, row: i32) -> Option<Range> {
    let r = distance_between(&entry.sensor, &entry.beacon);
    let dy = (entry.sensor.y-row).abs();
    if dy <= r {
        let dx = r - dy;
        let xmin = entry.sensor.x - dx;
        let xmax = entry.sensor.x + dx;
        let range = Range { min: xmin, max: xmax };
        Some(range)
    } else {
        None
    }
}

fn distance_between(a: &Coord, b: &Coord) -> i32 {
     (b.x-a.x).abs() + (b.y-a.y).abs()
}
