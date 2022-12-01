use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let part = 2;

    if part == 1 {
        let mut max_calories: u32 = 0;
        let mut current_calories: u32 = 0;

        for line in contents.lines() {
            if line.is_empty() {
                if current_calories > max_calories {
                    max_calories = current_calories;
                }
                current_calories = 0;
            } else {
                current_calories += line.parse::<u32>().unwrap();
            }
        }
        println!("{max_calories}");
    } else if part == 2 {
        let mut top4 = [0; 4];
        let mut cals = 0;

        for line in contents.lines() {
            if line.is_empty() {
                top4[0] = cals;
                top4.sort();
                cals = 0;
            } else {
                cals += line.parse::<u32>().unwrap();
            }
        }
        println!("Answer: {}", top4[1]+top4[2]+top4[3]);
    }
}

fn print_top4(top4: &[u32]) {
    println!("{}", top4[0]);
    println!("{}", top4[1]);
    println!("{}", top4[2]);
    println!("{}", top4[3]);
}
