use std::collections::HashSet;
use std::env;
use std::fs;

struct Grid {
    data: Vec<u8>,
    nrows: usize,
    ncols: usize,
    start: (usize, usize),
    stop: (usize, usize),
}

impl Grid {
    fn get_value(&self, pos: (usize, usize)) -> u8 {
        let (x, y) = pos;
        self.data[y*self.ncols+x]
    }

    fn contains(&self, pos: (i32, i32)) -> bool {
        pos.0 >= 0 && pos.0 < (self.nrows as i32) && pos.1 >= 0 && pos.1 < (self.ncols as i32)
    }
}

struct Node {
    pos: (usize, usize),
    cost: u32,
    heuristic_cost: u32,
}

impl Node {
    fn get_total_cost(&self) -> u32 {
        self.cost + self.heuristic_cost
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
    let contents = fs::read_to_string(infile).unwrap();
    let grid = parse_file(&contents);
    print_grid(&grid);
    let nsteps = search(&grid, grid.start);
    println!("Result: {}", nsteps);
}

fn do_part2(infile: &str) {
    let contents = fs::read_to_string(infile).unwrap();
    let grid = parse_file(&contents);

    let mut min_steps = 1000;
    // for start in possible_starts(&grid).iter() {
    //     println!("{:?}", start);
    // }
    for start in possible_starts(&grid).iter() {
        let nsteps = search(&grid, *start);
        println!("{:?}", nsteps);
        if nsteps < min_steps {
            min_steps = nsteps;
        }
    }
    println!("Result: {}", min_steps);
}

fn possible_starts(grid: &Grid) -> Vec<(usize, usize)> {
    let mut starts: Vec<(usize, usize)> = Vec::new();
    for i in 0..grid.nrows {
        for j in 0..grid.ncols {
            if grid.get_value((j, i)) == 0 {
                starts.push((j, i));
            }
        }
    }
    starts
}

fn parse_file(contents: &str) -> Grid {
    let mut data: Vec<u8> = Vec::new();
    let mut nrows: usize = 0;
    let mut ncols: usize = 0;
    let mut start: (usize, usize) = (0, 0);
    let mut stop: (usize, usize) = (0, 0);
    for line in contents.lines() {
        nrows += 1;
        ncols = 0;  // nasty
        for ch in line.chars() {
            ncols += 1;
            let mut elevation: u8;
            if ch == 'S' {
                start = (ncols-1, nrows-1);
                elevation = 'a'.to_digit(36).unwrap() as u8;
            } else if ch == 'E' {
                stop = (ncols-1, nrows-1);
                elevation = 'z'.to_digit(36).unwrap() as u8;
            } else {
                elevation = ch.to_digit(36).unwrap() as u8;
            }
            data.push(elevation - 10);  // so we start from 0
        }
    }
    Grid { data, nrows, ncols, start, stop }
}

fn print_grid(grid: &Grid) {
    for i in 0..grid.nrows {
        for j in 0..grid.ncols {
            print!("{:02} ", grid.get_value((j, i)));
        }
        println!();
    }
}

fn search(grid: &Grid, start_pos: (usize, usize)) -> u32 {
    let start = Node {
        pos: start_pos,
        cost: 0,
        heuristic_cost: heuristic(start_pos, &grid),
    };
    let mut paths: Vec<Node> = vec![start];
    let mut visited: HashSet<(usize, usize)> = HashSet::from([start_pos]);
    // while !paths.is_empty() {
    while !visited.contains(&grid.stop) {
        // pop the node with the smallest value
        let node = paths.pop();

        if node.is_none() {
            break;
        }

        let node = node.unwrap();

        let mut new_locs: Vec<(usize, usize)> = Vec::new();

        if let Some(left) = go_left(node.pos, &grid) {
            new_locs.push(left);
        }
        if let Some(right) = go_right(node.pos, &grid) {
            new_locs.push(right);
        }
        if let Some(up) = go_up(node.pos, &grid) {
            new_locs.push(up);
        }
        if let Some(down) = go_down(node.pos, &grid) {
            new_locs.push(down);
        }

        for pos in new_locs.iter() {
            if grid.get_value(*pos) <= grid.get_value(node.pos) + 1 &&
                !visited.contains(pos)
            {
                let new_node = Node {
                    pos: *pos,
                    cost: node.cost + 1,
                    heuristic_cost: heuristic(*pos, &grid),
                };
                paths.push(new_node);
                visited.insert(*pos);
            }
        }

        // sort paths
        paths.sort_by(|a, b| b.get_total_cost().cmp(&a.get_total_cost()));
    }

    let mut result = 1000;
    for node in paths.iter() {
        if node.pos == grid.stop {
            result = node.cost;
        }
    }
    result
}

fn go_down(pos: (usize, usize), grid: &Grid) -> Option<(usize, usize)> {
    if pos.1 > 0 {
        Some((pos.0, pos.1 - 1))
    } else {
        None
    }
    
}

fn go_up(pos: (usize, usize), grid: &Grid) -> Option<(usize, usize)> {
    if pos.1 < grid.nrows - 1 {
        Some((pos.0, pos.1 + 1))
    } else {
        None
    }
}

fn go_left(pos: (usize, usize), grid: &Grid) -> Option<(usize, usize)> {
    if pos.0 > 0 {
        Some((pos.0 - 1, pos.1))
    } else {
        // wrap around
        go_down((grid.ncols-1, pos.1), grid)
    }
}

fn go_right(pos: (usize, usize), grid: &Grid) -> Option<(usize, usize)> {
    if pos.0 < grid.ncols - 1 {
        Some((pos.0 + 1, pos.1))
    } else {
        // wrap around
        go_up((0, pos.1), grid)
    }
}

fn heuristic(pos: (usize, usize), grid: &Grid) -> u32 {
    // this isn't quite right any more
    // ((grid.stop.0 as i32 - pos.0 as i32).abs() + (grid.stop.1 as i32 - pos.1 as i32).abs()) as u32
    0
}
