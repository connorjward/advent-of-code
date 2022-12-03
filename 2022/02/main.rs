use std::fs;

fn main() {
    let part = 2;
    let contents = fs::read_to_string("input.txt").unwrap();

    if part == 1 {
        let mut score = 0;
        for line in contents.lines() {
            let moves: Vec<&str> = line.split(" ").collect();
            score += compute_score(&moves[1], &moves[0]);
        }
        println!("Final score: {score}");
    } else if part == 2 {
        let mut score = 0;
        for line in contents.lines() {
            let moves: Vec<&str> = line.split(" ").collect();
            score += compute_score2(&moves[0], &moves[1]);
        }
        println!("Final score: {score}");
    }
}

fn compute_score(self_move: &str, other_move: &str) -> u32 {
    match self_move {
        "X" => match other_move {  // rock
            "A" => 1 + 3,  // rock
            "B" => 1 + 0,  // paper
            "C" => 1 + 6,  // scissors
            &_ => panic!("oops")
        },
        "Y" => match other_move {  // paper
            "A" => 2 + 6,  // rock
            "B" => 2 + 3,  // paper
            "C" => 2 + 0,  // scissors
            &_ => panic!("oops")
        },
        "Z" => match other_move {  // scissors
            "A" => 3 + 0,  // rock
            "B" => 3 + 6,  // paper
            "C" => 3 + 3,  // scissors
            &_ => panic!("oops")
        },
        &_ => panic!("oops")
    }
}

fn compute_score2(other_move: &str, response: &str) -> u32 {
    match other_move {
        "A" => match response {  // rock
            "X" => 3 + 0,  // lose (scissors)
            "Y" => 1 + 3,  // draw
            "Z" => 2 + 6,  // win (paper)
            &_ => panic!("oops")
        },
        "B" => match response {  // paper
            "X" => 1 + 0,  // lose (rock)
            "Y" => 2 + 3,  // draw
            "Z" => 3 + 6,  // win (scissors)
            &_ => panic!("oops")
        },
        "C" => match response {  // scissors
            "X" => 2 + 0,  // lose (paper)
            "Y" => 3 + 3,  // draw
            "Z" => 1 + 6,  // win (rock)
            &_ => panic!("oops")
        },
        &_ => panic!("oops")
    }
}
