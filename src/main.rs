use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};
mod day01;
mod day02;

type DayFn = fn(Vec<String>) -> (String, String);
const DAYS: [DayFn; 2] = [day01::day01, day02::day02];

fn load_input(day: usize) -> Vec<String> {
    let path = format!("{}/{}.txt", "inputs", day);
    let f: File = File::open(path).unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    reader
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .expect("Unable to open file")
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
    let (part1, part2) = DAYS[day - 1](input);
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
