use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
};

pub fn load_input(day: usize) -> Vec<String> {
    let path = format!("input/2023/day{}.txt", day);
    let f: File = File::open(path).unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    reader
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .expect("Unable to open file")
}

pub fn load_input_string(day: usize) -> String {
    let path = format!("input/2023/day{}.txt", day);
    fs::read_to_string(path).expect("Unable to open file")
}

pub fn test_input(input: &str) -> Vec<String> {
    input
        .trim()
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>()
}
