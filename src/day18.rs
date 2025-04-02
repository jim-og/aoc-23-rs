use core::panic;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from(&self, loc: Point, dist: isize) -> Point {
        match self {
            Direction::Up => (loc.0 - dist, loc.1),
            Direction::Right => (loc.0, loc.1 + dist),
            Direction::Down => (loc.0 + dist, loc.1),
            Direction::Left => (loc.0, loc.1 - dist),
        }
    }
}

struct Instruction {
    direction: Direction,
    distance: isize,
    hex_direction: Direction,
    hex_distance: isize,
}

type Point = (isize, isize);

trait Perimeter {
    fn area(&self) -> usize;
}

impl Perimeter for Vec<(isize, isize)> {
    /// Calculate area with Shoelace Formula (https://en.wikipedia.org/wiki/Shoelace_formula).
    /// The starting point must be present at the start and the end of the outline
    fn area(&self) -> usize {
        let sum = self.windows(2).fold(0, |acc, matrix| {
            acc + (matrix[0].0 * matrix[1].1) - (matrix[1].0 * matrix[0].1)
        });
        // If the points are labeled sequentially in the counterclockwise direction, then the area is positive,
        // if they are labeled in the clockwise direction, the area will be negative.
        (sum.abs() / 2) as usize
    }
}

trait Lagoon {
    fn volume(&self, decode: bool) -> usize;
}

impl Lagoon for Vec<Instruction> {
    fn volume(&self, decode: bool) -> usize {
        let mut pos = (0, 0);
        let mut vertices = Vec::from([pos]);
        let mut perimeter = 0;

        for i in self {
            let (distance, direction) = match decode {
                true => (i.hex_distance, i.hex_direction),
                false => (i.distance, i.direction),
            };
            pos = direction.from(pos, distance);
            vertices.push(pos);
            perimeter += distance;
        }

        let area = vertices.area();

        // Pick's Theorem (https://en.wikipedia.org/wiki/Pick%27s_theorem)
        let interior_points = area - (perimeter as usize - 1) / 2 + 1;

        interior_points + perimeter as usize - 1
    }
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| {
            let segments = line.split_whitespace().collect::<Vec<&str>>();
            let direction = match segments[0] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction"),
            };
            let distance = segments[1]
                .parse::<isize>()
                .expect("Unable to parse distance");

            let hex = segments[2].to_string();
            let hex_distance = isize::from_str_radix(&hex[2..7], 16).unwrap();
            let hex_direction = match &hex[7..8] {
                "3" => Direction::Up,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "0" => Direction::Right,
                _ => panic!("Invalid direction"),
            };
            Instruction {
                direction,
                distance,
                hex_direction,
                hex_distance,
            }
        })
        .collect()
}

#[aoc(day18, part1)]
fn part1(input: &Vec<Instruction>) -> usize {
    input.volume(false)
}

#[aoc(day18, part2)]
fn part2(input: &Vec<Instruction>) -> usize {
    input.volume(true)
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
    fn part2_example() {
        assert_eq!(part2(&parse(TEST)), 952408144115);
    }

    #[test]
    fn mainline() {
        assert_eq!(part1(&parse(&parser::load_input_string(18))), 67891);
        assert_eq!(part2(&parse(&parser::load_input_string(18))), 67891);
    }
}
