use std::collections::HashSet;
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
    let forest = parse_forest(infile);
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    add_visible_from_top(&mut visible, &forest);
    add_visible_from_right(&mut visible, &forest);
    add_visible_from_bottom(&mut visible, &forest);
    add_visible_from_left(&mut visible, &forest);
    println!("Result: {}", visible.len());
}

fn do_part2(infile: &str) {
    let forest = parse_forest(infile);
    let mut max_scenic_score = 0;
    for row in 1..(forest.len()-1) {
        for col in 1..(forest[0].len()-1) {
            let score = compute_scenic_score(&forest, row, col);
            if score > max_scenic_score {
                max_scenic_score = score;
            }
        }
    }
    println!("Result: {}", max_scenic_score);
}

fn parse_forest(infile: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(infile).unwrap();
    let mut forest: Vec<Vec<i32>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<i32> = Vec::new();
        for tree_height in line.chars() {
            let h = tree_height.to_digit(10).unwrap() as i32;
            row.push(h);
        }
        forest.push(row);
    }
    forest
}

fn compute_scenic_score(forest: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    let tscore = compute_scenic_score_top(forest, row, col);
    let rscore = compute_scenic_score_right(forest, row, col);
    let bscore = compute_scenic_score_bottom(forest, row, col);
    let lscore = compute_scenic_score_left(forest, row, col);
    tscore * rscore * bscore * lscore
}

fn compute_scenic_score_top(forest: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    let mut score = 0;
    let height = forest[row][col];
    for r in (0..row).rev() {
        score += 1; 
        if forest[r][col] >= height {
            break;
        }
    }
    score
}

fn compute_scenic_score_right(forest: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    let mut score = 0;
    let height = forest[row][col];
    for c in (col+1)..forest[0].len() {
        score += 1; 
        if forest[row][c] >= height {
            break;
        }
    }
    score
}

fn compute_scenic_score_bottom(forest: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    let mut score = 0;
    let height = forest[row][col];
    for r in (row+1)..forest.len() {
        score += 1; 
        if forest[r][col] >= height {
            break;
        }
    }
    score
}

fn compute_scenic_score_left(forest: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    let mut score = 0;
    let height = forest[row][col];
    for c in (0..col).rev() {
        score += 1; 
        if forest[row][c] >= height {
            break;
        }
    }
    score
}

fn add_visible_from_top(visible: &mut HashSet<(usize, usize)>, forest: &Vec<Vec<i32>>) {
    for col in 0..forest[0].len() {
        let mut current_height = -1;
        for row in 0..forest.len() {
            let height = forest[row][col];
            if height > current_height {
                visible.insert((row, col));
                current_height = height;
            }
        }
    }
}

fn add_visible_from_right(visible: &mut HashSet<(usize, usize)>, forest: &Vec<Vec<i32>>) {
    for row in 0..forest.len() {
        let mut current_height = -1;
        for col in (0..forest[0].len()).rev() {
            let height = forest[row][col];
            if height > current_height {
                visible.insert((row, col));
                current_height = height;
            }
        }
    }
}

fn add_visible_from_bottom(visible: &mut HashSet<(usize, usize)>, forest: &Vec<Vec<i32>>) {
    for col in 0..forest[0].len() {
        let mut current_height = -1;
        for row in (0..forest.len()).rev() {
            let height = forest[row][col];
            if height > current_height {
                visible.insert((row, col));
                current_height = height;
            }
        }
    }
}

fn add_visible_from_left(visible: &mut HashSet<(usize, usize)>, forest: &Vec<Vec<i32>>) {
    for row in 0..forest.len() {
        let mut current_height = -1;
        for col in 0..forest[0].len() {
            let height = forest[row][col];
            if height > current_height {
                visible.insert((row, col));
                current_height = height;
            }
        }
    }
}

fn print_forest(forest: &Vec<Vec<i32>>) {
    for row in forest {
        for tree_height in row {
            print!("{}", tree_height);
        }
        println!();
    }
}

fn print_visible(visible: &HashSet<(usize, usize)>, forest: &Vec<Vec<i32>>) {
    for row in 0..forest.len() {
        for col in 0..forest[0].len() {
            if visible.contains(&(row, col)) {
                println!("({}, {}): {}", row, col, forest[row][col]);
            }
        }
    }
}
