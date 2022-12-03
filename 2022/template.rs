use std::env;

fn main() {
    match &env::args().collect::<Vec<String>>()[..] {
        [part, infile] => {
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

fn do_part1(infile: &String) {

}

fn do_part2(infile: &String) {

}
