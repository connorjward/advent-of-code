use std::collections::VecDeque;
use std::env;

struct Monkey {
    id: usize,
    items: VecDeque<i64>,
    target_true: usize,
    target_false: usize,
}

fn update_worry(id: usize, current_worry: i64, demo: bool) -> i64 {
    if demo {
        match id {
            0 => current_worry * 19,
            1 => current_worry + 6,
            2 => current_worry * current_worry,
            3 => current_worry + 3,
            _ => panic!(),
        }
    } else {
        match id {
            0 => current_worry * 13,
            1 => current_worry + 3,
            2 => current_worry + 6,
            3 => current_worry + 2,
            4 => current_worry * current_worry,
            5 => current_worry + 4,
            6 => current_worry * 7,
            7 => current_worry + 7,
            _ => panic!(),
        }
    }
}

fn test_worry(id: usize, worry: i64, demo: bool) -> bool {
    if demo {
        match id {
            0 => worry % 23 == 0,
            1 => worry % 19 == 0,
            2 => worry % 13 == 0,
            3 => worry % 17 == 0,
            _ => panic!(),
        }
    } else {
        match id {
            0 => worry % 19 == 0,
            1 => worry % 2 == 0,
            2 => worry % 13 == 0,
            3 => worry % 5 == 0,
            4 => worry % 7 == 0,
            5 => worry % 11 == 0,
            6 => worry % 17 == 0,
            7 => worry % 3 == 0,
            _ => panic!(),
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let part = args[1].parse::<u8>().unwrap();
    let demo = args[2] == "demo";
    if part == 1 {
        do_part1(demo);
    } else if part == 2 {
        do_part2(demo);
    } else {
        panic!("Part not valid");
    }
}

fn do_part1(demo: bool) {
    let mut maxmodulo: i64;
    if demo {
        maxmodulo = 23 * 19 * 13 * 17;
    } else {
        maxmodulo = 19 * 2 * 13 * 5 * 7 * 11 * 17 * 3;
    }
    let nmonkeys: usize = if demo { 4 } else { 8 };
    let mut monkeys = init_monkeys(demo);
    let mut inspected: Vec<i32> = vec![0; nmonkeys];
    for i in 0..20 {
        println!("{}", i);
        print_monkey_items(&monkeys);
        update_items(&mut monkeys, &mut inspected, demo, true, maxmodulo);
    }

    inspected.sort();
    for m in 0..nmonkeys {
        println!("{}: {}", m, inspected[m]);
    }
}

fn do_part2(demo: bool) {
    let mut maxmodulo: i64;
    if demo {
        maxmodulo = 23 * 19 * 13 * 17;
    } else {
        maxmodulo = 19 * 2 * 13 * 5 * 7 * 11 * 17 * 3;
    }
    let nmonkeys: usize = if demo { 4 } else { 8 };
    let mut monkeys = init_monkeys(demo);
    let mut inspected: Vec<i32> = vec![0; nmonkeys];
    for i in 0..10000 {
        update_items(&mut monkeys, &mut inspected, demo, false, maxmodulo);
    }

    inspected.sort();
    for m in 0..nmonkeys {
        println!("{}: {}", m, inspected[m]);
    }
}

fn init_monkeys(demo: bool) -> Vec<Monkey> {
    if demo {
         vec![
            Monkey { id: 0, items: VecDeque::from([79, 98]), target_true: 2, target_false: 3 },
            Monkey { id: 1, items: VecDeque::from([54, 65, 75, 74]), target_true: 2, target_false: 0 },
            Monkey { id: 2, items: VecDeque::from([79, 60, 97]), target_true: 1, target_false: 3 },
            Monkey { id: 3, items: VecDeque::from([74]), target_true: 0, target_false: 1 },
         ]
    } else {
         vec![
            Monkey { id: 0, items: VecDeque::from([71, 86]), target_true: 6, target_false: 7 },
            Monkey { id: 1, items: VecDeque::from([66, 50, 90, 53, 88, 85]), target_true: 5, target_false: 4 },
            Monkey { id: 2, items: VecDeque::from([97, 54, 89, 62, 84, 80, 63]), target_true: 4, target_false: 1 },
            Monkey { id: 3, items: VecDeque::from([82, 97, 56, 92]), target_true: 6, target_false: 0 },
            Monkey { id: 4, items: VecDeque::from([50, 99, 67, 61, 86]), target_true: 5, target_false: 3 },
            Monkey { id: 5, items: VecDeque::from([61, 66, 72, 55, 64, 53, 72, 63]), target_true: 3, target_false: 0 },
            Monkey { id: 6, items: VecDeque::from([59, 79, 63]), target_true: 2, target_false: 7 },
            Monkey { id: 7, items: VecDeque::from([55]), target_true: 2, target_false: 1 },
         ]
    }
}

fn update_items(monkeys: &mut Vec<Monkey>, inspected: &mut Vec<i32>, demo: bool, divide: bool, maxmodulo: i64) {
    for id in 0..monkeys.len() {
        while monkeys[id].items.len() > 0 {
            inspected[id] += 1;

            let mut worry: i64;

            {
                let popped = monkeys[id].items.pop_front();
                if popped.is_some() {
                    worry = popped.unwrap();
                } else {
                    break;
                }
            }

            worry = update_worry(id, worry, demo);
            if divide {
                worry /= 3;
            }

            worry %= maxmodulo;
            
            let mut target: usize;
            if test_worry(id, worry, demo) {
                target = monkeys[id].target_true;
            } else {
                target = monkeys[id].target_false;
            }
            monkeys[target].items.push_back(worry);
        }
    }
}

fn print_monkey_items(monkeys: &Vec<Monkey>) {
    for monkey in monkeys.iter() {
        print!("Monkey {}: ", monkey.id);
        for item in monkey.items.iter() {
            print!("{}, ", item);
        }
        println!();
    }
}
