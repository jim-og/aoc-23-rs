use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn load_input(day: usize) -> Vec<String> {
    let path = format!("{}/{}.txt", "inputs", day);
    let f: File = File::open(path).unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    reader
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .expect("Unable to open file")
}

#[cfg(test)]
pub fn test_input(input: &str) -> Vec<String> {
    input
        .trim()
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>()
}
