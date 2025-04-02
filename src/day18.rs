use core::panic;
use std::usize;

use crate::day10::shoelace;

type Point = (i32, i32);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from(&self, loc: Point) -> Point {
        match self {
            Direction::Up => (loc.0 - 1, loc.1),
            Direction::Right => (loc.0, loc.1 + 1),
            Direction::Down => (loc.0 + 1, loc.1),
            Direction::Left => (loc.0, loc.1 - 1),
        }
    }
}

struct Trench {
    point: Point,
    hex: String,
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<Trench> {
    // let floor = Vec<(usize, usize)>;
    let mut floor = Vec::new();
    floor.push(Trench {
        point: (0, 0),
        hex: "?".to_string(),
    });
    let mut pos = (0, 0);

    for (_, line) in input.trim().lines().enumerate() {
        let segments = line.split_whitespace().collect::<Vec<&str>>();
        let direction = match segments[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        let distance = segments[1]
            .parse::<usize>()
            .expect("Unable to parse distance");
        let hex = segments[2].to_string();

        for _ in 0..distance {
            pos = direction.from(pos);
            floor.push(Trench {
                point: pos.clone(),
                hex: hex.clone(),
            });
        }
    }

    floor
}

#[aoc(day18, part1)]
fn part1(input: &Vec<Trench>) -> usize {
    let outline: Vec<(i32, i32)> = input.iter().map(|trench| trench.point).collect();
    let area = shoelace(&outline);

    // Pick's Theorem (https://en.wikipedia.org/wiki/Pick%27s_theorem)
    let interior_points = area - (outline.len() - 1) / 2 + 1;

    interior_points + outline.len() - 1
}

#[aoc(day18, part2)]
fn part2(_input: &Vec<Trench>) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::parser;

    use super::*;

    const TEST: &str = "
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST)), 62);
    }

    #[test]
    fn mainline() {
        assert_eq!(part1(&parse(&parser::load_input_string(18))), 67891);
    }
}
