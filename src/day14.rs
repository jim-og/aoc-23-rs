use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Rock {
    Empty,
    Round,
    Cube,
}

impl fmt::Display for Rock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rock::Empty => write!(f, "."),
            Rock::Round => write!(f, "O"),
            Rock::Cube => write!(f, "#"),
        }
    }
}

enum Tilt {
    North,
    East,
    South,
    West,
}

struct Platform {
    rocks: Vec<Vec<Rock>>,
}

impl Platform {
    fn new(input: Vec<String>) -> Self {
        let mut rocks = Vec::new();
        input.iter().for_each(|line| {
            rocks.push(
                line.chars()
                    .map(|c| match c {
                        'O' => Rock::Round,
                        '#' => Rock::Cube,
                        _ => Rock::Empty,
                    })
                    .collect::<Vec<_>>(),
            );
        });
        Self { rocks }
    }

    fn tilt(mut self, direction: Tilt) {
        match direction {
            Tilt::North => {
                let width = self.rocks.first().unwrap().len();
                let height = self.rocks.len();

                // Iterate through the platform from top left to bottom right
                for i in 1..height {
                    for j in 0..width {
                        if self.rocks[i][j] == Rock::Round {
                            // Look up along this column to find the next rock which is hit
                            let mut moved = false;

                            for offset in 1..i {
                                if self.rocks[i - offset][j] != Rock::Empty {
                                    // We've hit something. Move the rock to the position before this.
                                    self.rocks[i - offset][j] = Rock::Round;
                                    self.rocks[i][j] = Rock::Empty;
                                    moved = true;
                                    self.print();
                                    // TODO refactor to a fn we can return from
                                    break;
                                }
                            }

                            // If no rock was found then it must hit the boundary
                            if !moved {
                                self.rocks[0][j] = Rock::Round;
                                self.rocks[i][j] = Rock::Empty;
                            }
                        }
                    }
                }
            }
            Tilt::East => todo!(),
            Tilt::South => todo!(),
            Tilt::West => todo!(),
        }
        todo!()
    }

    fn print(&self) {
        self.rocks
            .iter()
            .map(|rocks| {
                rocks
                    .iter()
                    .map(|rock| rock.to_string())
                    .collect::<String>()
            })
            .for_each(|line| println!("{}", line));
    }
}

pub fn day14(input: Vec<String>) -> (String, String) {
    let platform = Platform::new(input);
    platform.print();
    platform.tilt(Tilt::North);

    let part_1 = 1;
    let part_2 = 2;
    (format!("{}", part_1), format!("{}", part_2))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::parser;
//     use test_case::test_case;

//     #[test_case(
//         "
//         O....#....
//         O.OO#....#
//         .....##...
//         OO.#O....O
//         .O.....O#.
//         O.#..O.#.#
//         ..O..#O..O
//         .......O..
//         #....###..
//         #OO..#....
//         ",
//         136
//         ;"1"
//     )]
//     fn example(input: &str, answer: usize) {
//         let result = day14(parser::test_input(input));
//         assert_eq!(result.0, "1");
//         assert_eq!(result.1, "2");
//     }

//     #[test]
//     fn mainline() {
//         let input = parser::load_input(14);
//         let result = day14(input);
//         assert_eq!(result.0, "1");
//         assert_eq!(result.1, "2");
//     }
// }
