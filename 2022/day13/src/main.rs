use std::boxed::Box;
use std::cmp::Ordering;
use std::env;
use std::fs;
use std::iter::zip;

enum ListContents {
    List(Box<Vec<ListContents>>),
    Item(u32),
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
    let contents = fs::read_to_string(infile).unwrap();
    let pairs = parse_input(&contents);
    for (i, pair) in pairs.iter().enumerate() {
        let order_correct = check_order(&pair.0, &pair.1).unwrap();
        if order_correct {
            result += i + 1;
        }
    }
    println!("Result: {}", result);
}

fn do_part2(infile: &str) {
    let contents = fs::read_to_string(infile).unwrap();
    let mut pairs = parse_input(&contents);

    let mut packets = unpack_pairs(&mut pairs);
    packets.push(parse_line("[[2]]"));
    packets.push(parse_line("[[6]]"));


    // println!("packets before");
    //
    // for packet in packets.iter() {
    //     print_packet(&packet);
    // }

    packets.sort_by(|a, b| check_ordering(a, b));

    // println!("sorted");
    // for packet in packets.iter() {
    //     print_packet(&packet);
    // }
    let mut found0: usize = 0;
    let mut found1: usize = 0;
    for (i, packet) in packets.iter().enumerate() {
        if print_packet_inner(&packet) == "[[2]]" {
            found0 = i + 1;
        }
        if print_packet_inner(&packet) == "[[6]]" {
            found1 = i + 1;
        }
    }
    println!("Result: {}", found0*found1);
}

fn parse_line(line: &str) -> ListContents {
    let mut stack: Vec<Vec<ListContents>> = Vec::new();
    let mut current_num = String::from("");
    for (i, ch) in line.chars().enumerate() {
        if i == line.len() - 1 {
            if current_num.len() > 0 {
                stack.last_mut().unwrap().push(ListContents::Item(current_num.parse::<u32>().unwrap()));
            }
            let popped = stack.pop().unwrap();
            return ListContents::List(Box::new(popped));
        }

        match ch {
            '[' => stack.push(Vec::new()),
            ']' => {
                if current_num.len() > 0 {
                    stack.last_mut().unwrap().push(ListContents::Item(current_num.parse::<u32>().unwrap()));
                    current_num = String::from("");
                }
                let popped = stack.pop().unwrap();
                stack.last_mut().unwrap().push(ListContents::List(Box::new(popped)));
            },
            ',' => {
                if current_num.len() > 0 {
                    stack.last_mut().unwrap().push(ListContents::Item(current_num.parse::<u32>().unwrap()));
                    current_num = String::from("");
                }
            },
            digit => current_num.push(digit),
        };
    }
    panic!("Line did not parse correctly");
}

fn check_order(a: &ListContents, b: &ListContents) -> Option<bool> {
    match (a, b) {
        (ListContents::Item(ldigit), ListContents::Item(rdigit)) => {
            if ldigit < rdigit {
                Some(true)
            } else if ldigit > rdigit {
                Some(false)
            } else {
                None
            }
        },
        (ListContents::List(litems), ListContents::List(ritems)) => {
            let mut lefts = litems.iter();
            let mut rights = ritems.iter();
            let mut left = lefts.next();
            let mut right = rights.next();
            while left.is_some() && right.is_some() {
                let result = check_order(left.unwrap(), right.unwrap());
                if result.is_some() {
                    return result;
                }
                left = lefts.next();
                right = rights.next();
            }
            if left.is_none() {
                if right.is_none() {
                    None
                } else {
                    Some(true)
                }
            } else {
                Some(false)
            }
        },
        (ListContents::Item(ldigit), ListContents::List(ritems)) => {
            let litems = ListContents::List(Box::new(vec![ListContents::Item(*ldigit)]));
            check_order(&litems, b)
        },
        (ListContents::List(litems), ListContents::Item(rdigit)) => {
            let ritems = ListContents::List(Box::new(vec![ListContents::Item(*rdigit)]));
            check_order(a, &ritems)
        },
    }
}

fn check_ordering(a: &ListContents, b: &ListContents) -> Ordering {
    let result = check_order(a, b);
    if result.is_some() {
        if result.unwrap() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    } else {
        Ordering::Equal
    }
}

fn parse_input(contents: &str) -> Vec<(ListContents, ListContents)> {
    let mut pairs: Vec<(ListContents, ListContents)> = Vec::new();
    let lines: Vec<&str> = contents.lines().collect();
    for i in (0..lines.len()).step_by(3) {
        let pair = (parse_line(lines[i]), parse_line(lines[i+1]));
        pairs.push(pair);
    }
    pairs
}

fn unpack_pairs(pairs: &mut Vec<(ListContents, ListContents)>) -> Vec<ListContents> {
    let mut packets: Vec<ListContents> = Vec::new();
    for mut pair in pairs.drain(..) {
        packets.push(pair.0);
        packets.push(pair.1);
    }
    packets
}

fn print_pairs(pairs: &Vec<(ListContents, ListContents)>) {
    for (i, pair) in pairs.iter().enumerate() {
        print_packet(&pair.0);
        print_packet(&pair.1);
        if i < pairs.len() - 1 {
            println!();
        }
    }
}

fn print_packet(packet: &ListContents) {
    let str = print_packet_inner(&packet);
    println!("{}", str);
}

fn print_packet_inner(packet: &ListContents) -> String {
    let mut str = String::from("");
    match packet {
        ListContents::List(contents) => {
            str.push('[');
            for (i, item) in contents.iter().enumerate() {
                str.push_str(print_packet_inner(item).as_str());
                if i < contents.len() - 1 {
                    str.push(',');
                }
            }
            str.push(']');
        },
        ListContents::Item(digit) => str.push_str(digit.to_string().as_str()),
    }
    str
}
