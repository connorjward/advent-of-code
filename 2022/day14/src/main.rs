use std::env;
use std::fs;

#[derive(Clone, Copy)]
struct Position(i32, i32);

struct Path {
    start: Position,
    stop: Position,
}

#[derive(Clone, Copy)]
enum Space {
    SandSource,
    Sand,
    Rock,
    Empty,
}

struct Grid {
    data: Vec<Space>,
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
    source: Position,
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<&Space> {
        let width: usize = (self.xmax - self.xmin).try_into().unwrap();
        // println!("{}, {}", x, y);
        let xoff: usize = (x - self.xmin).try_into().unwrap();
        let yoff: usize = (y - self.ymin).try_into().unwrap();
        // println!("{}, {}, {}, {}, {}", x, y, width, xoff, yoff);
        self.data.get(yoff*width+xoff)
    }

    fn set(&mut self, x: i32, y: i32, value: Space) {
        let width: usize = (self.xmax - self.xmin).try_into().unwrap();
        let xoff: usize = (x - self.xmin).try_into().unwrap();
        let yoff: usize = (y - self.ymin).try_into().unwrap();
        self.data[yoff*width+xoff] = value;
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
    let mut grid = parse_input(&contents, false);

    let mut nunits = 0;
    while !evolve(&mut grid) {
        nunits += 1;
    }
    // print_grid(&grid);
    println!("Result: {}", nunits);
}

fn do_part2(infile: &str) {
    let contents = fs::read_to_string(infile).unwrap();
    let mut grid = parse_input(&contents, true);

    let mut nunits = 0;
    while !evolve(&mut grid) {
        nunits += 1;
    }
    // print_grid(&grid);
    println!("Result: {}", nunits+1);
}

fn parse_input(content: &str, with_floor: bool) -> Grid {
    // load paths
    let mut paths: Vec<Path> = vec![];
    for line in content.lines() {
        let new_paths = parse_line(line);
        paths.extend(new_paths);
    }

    // println!("{}", paths.len());

    // find the bounds of the grid
    let mut xmin: i32 = -1000;
    let mut xmax: i32 = 1500;
    let ymin: i32 = 0;  // must be true for sand source
    let mut ymax: i32 = 0;
    for path in paths.iter() {
        // if path.start.0 < xmin {
        //     xmin = path.start.0;
        // }
        // if path.stop.0 < xmin {
        //     xmin = path.stop.0;
        // }
        // if path.start.0 > xmax {
        //     xmax = path.start.0;
        // }
        // if path.stop.0 > xmax {
        //     xmax = path.stop.0;
        // }
        //
        if path.start.1 > ymax {
            ymax = path.start.1;
        }
        if path.stop.1 > ymax {
            ymax = path.stop.1;
        }
    }

    // add a buffer region
    // if xmin < 100 {
    //     xmin = 0;
    // } else {
    //     xmin -= 100;
    // }
    // xmin = 0;
    // xmax += 100;
    ymax += 2;

    println!("{}, {}, {}, {}", xmin, xmax, ymin, ymax);

    // create the grid
    let data: Vec<Space> = vec![Space::Empty; ((xmax-xmin+1)*(ymax-ymin+1)).try_into().unwrap()];
    let source = Position(500, 0);
    let mut grid = Grid { data, xmin, xmax, ymin, ymax, source };

    // add sand source
    grid.set(500, 0, Space::SandSource);

    // set a rock floor
    if with_floor {
        for x in grid.xmin .. grid.xmax+1 {
            grid.set(x, grid.ymax, Space::Rock);
        }
    }

    for path in paths {
        if path.start.0 == path.stop.0 {  // vertical
            let ystart: usize;
            let ystop: usize;
            if path.start.1 < path.stop.1 {
                ystart = path.start.1 as usize;
                ystop = path.stop.1 as usize;
            } else {
                ystart = path.stop.1 as usize;
                ystop = path.start.1 as usize;
            }
            for y in ystart .. ystop+1 {
                grid.set(path.start.0, y as i32, Space::Rock);
            }
        } else {  // horizontal
            assert_eq!(path.start.1, path.stop.1);
            let xstart: usize;
            let xstop: usize;
            if path.start.0 < path.stop.0 {
                xstart = path.start.0 as usize;
                xstop = path.stop.0 as usize;
            } else {
                xstart = path.stop.0 as usize;
                xstop = path.start.0 as usize;
            }
            for x in xstart .. xstop+1 {
                grid.set(x as i32, path.start.1, Space::Rock);
            }
        }
    }

    grid
}

fn parse_line(line: &str) -> Vec<Path> {
    let mut paths: Vec<Path> = vec![];
    let mut last: Option<Position> = None;
    for pos_str in line.split(" -> ") {
        let pos = pos_str.split_once(",").unwrap();
        let pos = Position(pos.0.parse::<i32>().unwrap(), pos.1.parse::<i32>().unwrap());
        if last.is_some() {
            let path = Path { start: last.unwrap(), stop: pos };
            paths.push(path);
        }
        last = Some(pos);
    }
    paths
}

fn evolve(grid: &mut Grid) -> bool {
    evolve_inner(grid, grid.source)
}

fn evolve_inner(grid: &mut Grid, pos: Position) -> bool {
    // try to move straight down
    match grid.get(pos.0, pos.1+1) {
        Some(Space::Empty) => {
            let next = Position(pos.0, pos.1+1);
            return evolve_inner(grid, next);
        },
        None => return true,
        _ => { },
    }

    // now down and left
    match grid.get(pos.0-1, pos.1+1) {
        Some(Space::Empty) => {
            let next = Position(pos.0-1, pos.1+1);
            return evolve_inner(grid, next);
        },
        None => return true,
        _ => { },
    }

    // now down and right
    match grid.get(pos.0+1, pos.1+1) {
        Some(Space::Empty) => {
            let next = Position(pos.0+1, pos.1+1);
            return evolve_inner(grid, next);
        },
        None => return true,
        _ => { },
    }

    match grid.get(pos.0, pos.1).unwrap() {
        Space::SandSource => true,
        _ => {
            // else sand stops moving
            grid.set(pos.0, pos.1, Space::Sand);
            // return false since we're not finished yet
            false
        }
    }
}

fn print_grid(grid: &Grid) {
    for y in grid.ymin .. grid.ymax+1 {
        for x in grid.xmin .. grid.xmax+1 {
            let ch = match grid.get(x, y).unwrap() {
                Space::SandSource => '+',
                Space::Sand => 'o',
                Space::Rock => '#',
                Space::Empty => '.',
            };
            print!("{}", ch);        
        }
        println!();
    }
}
