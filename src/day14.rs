use std::{
    collections::{HashMap, VecDeque},
    fmt,
};

type Point = (usize, usize);

#[derive(Clone, PartialEq)]
enum Rock {
    Empty,
    Round,
    Cube,
}

#[derive(Clone, PartialEq)]
struct Platform {
    layout: HashMap<Point, Rock>,
    width: usize,
    height: usize,
}

impl Platform {
    fn tilt(mut self) -> Self {
        for col in 0..self.width {
            let mut empty_spaces = VecDeque::new();

            for row in 0..self.height {
                match self.layout[&(row, col)] {
                    Rock::Empty => empty_spaces.push_back((row, col)),
                    Rock::Round => {
                        if let Some(space) = empty_spaces.pop_front() {
                            self.layout.insert(space, Rock::Round);
                            self.layout.insert((row, col), Rock::Empty);
                            empty_spaces.push_back((row, col));
                        }
                    }
                    Rock::Cube => {
                        empty_spaces.clear();
                    }
                }
            }
        }
        self
    }

    fn rotate_clockwise(mut self) -> Self {
        let mut clockwise = HashMap::new();

        for row in 0..self.height {
            for col in 0..self.width {
                clockwise.insert(
                    (col, self.height - row - 1),
                    self.layout[&(row, col)].clone(),
                );
            }
        }
        self.layout = clockwise;
        self
    }

    fn cycle(self) -> Self {
        self.tilt()
            .rotate_clockwise()
            .tilt()
            .rotate_clockwise()
            .tilt()
            .rotate_clockwise()
            .tilt()
            .rotate_clockwise()
    }

    fn cycles(self, n: usize) -> Self {
        (0..n).fold(self, |acc, _| acc.cycle())
    }

    // Brent's cycle detection algorithm
    // "https://en.wikipedia.org/wiki/Cycle_detection#Brent's_algorithm"
    fn brent_cycles(self, n: usize) -> Self {
        let mut power = 1;
        let mut lambda = 1;
        let mut tortoise = self.clone();
        let mut hare = self.clone().cycle();

        // This assumes there is a cycle; otherwise this loop won't terminate
        while tortoise != hare {
            if power == lambda {
                tortoise = hare.clone();
                power *= 2;
                lambda = 0;
            }
            hare = hare.cycle();
            lambda += 1;
        }

        // Find the position of the first repetition of length lamda
        tortoise = self.clone();
        hare = self.clone().cycles(lambda);

        // The distance between the hare and tortoise is now lamda
        // Next, the hare and tortoise move at the same speed until they agree
        let mut mu = 0;
        while tortoise != hare {
            tortoise = tortoise.cycle();
            hare = hare.cycle();
            mu += 1;
        }

        self.cycles(mu).cycles((n - mu) % lambda)
    }

    fn total_load(&self) -> usize {
        let mut load = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                if self.layout[&(row, col)] == Rock::Round {
                    load += self.height - row;
                }
            }
        }
        load
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let symbol = match self.layout[&(row, col)] {
                    Rock::Empty => '.',
                    Rock::Round => 'O',
                    Rock::Cube => '#',
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Platform {
    Platform {
        layout: input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.trim().chars().enumerate().map(move |(col, c)| {
                    let rock = match c {
                        'O' => Rock::Round,
                        '#' => Rock::Cube,
                        '.' => Rock::Empty,
                        _ => panic!("Invalid rock char"),
                    };
                    ((row, col), rock)
                })
            })
            .collect(),
        width: input.trim().lines().last().unwrap().trim().len(),
        height: input.trim().lines().count(),
    }
}

#[aoc(day14, part1)]
fn part1(input: &Platform) -> usize {
    input.clone().tilt().total_load()
}

#[aoc(day14, part2)]
fn part2(input: &Platform) -> usize {
    input.clone().brent_cycles(1_000_000_000).total_load()
}

#[cfg(test)]
mod tests {
    use crate::parser;

    use super::*;

    const TEST: &str = "
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST)), 136);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TEST)), 64);
    }

    #[test]
    fn mainline() {
        assert_eq!(part1(&parse(&parser::load_input_string(14))), 110128);
        // assert_eq!(part2(&parse(&parser::load_input_string(14))), 103861);
    }
}
