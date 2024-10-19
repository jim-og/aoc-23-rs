use std::env;
mod day01;

type DayFn = fn(&str) -> (String, String);
const DAYS: [DayFn; 1] = [day01::day01];

fn load_input(day: usize) -> String {
    let path = format!("{}/{}.txt", "inputs", day);
    println!("Loading {}", path);
    std::fs::read_to_string(path).expect("Unable to open file")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please specify a single day as an integer")
    }
    let day = args[1]
        .parse::<usize>()
        .expect("Day parameter not an integer");

    let input = load_input(day);
    let (part1, part2) = DAYS[day - 1](&input);
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
