use std::collections::HashMap;

type Point = (i32, i32);

enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    pub fn from(&self, loc: Point) -> Point {
        match self {
            Heading::North => (loc.0 - 1, loc.1),
            Heading::East => (loc.0, loc.1 + 1),
            Heading::South => (loc.0 + 1, loc.1),
            Heading::West => (loc.0, loc.1 - 1),
        }
    }
}

#[derive(PartialEq)]
enum Route {
    NorthAndSouth,
    EastAndWest,
    NorthAndEast,
    NorthAndWest,
    SouthAndWest,
    SouthAndEast,
    Ground,
    Start,
}

struct Pipe {
    route: Route,
}

impl Pipe {
    pub fn new(symbol: char) -> Self {
        let route = match symbol {
            '|' => Route::NorthAndSouth,
            '-' => Route::EastAndWest,
            'L' => Route::NorthAndEast,
            'J' => Route::NorthAndWest,
            '7' => Route::SouthAndWest,
            'F' => Route::SouthAndEast,
            'S' => Route::Start,
            _ => Route::Ground,
        };
        Self { route }
    }

    pub fn get_heading(&self, heading: &Heading) -> Option<Heading> {
        match heading {
            Heading::North => match self.route {
                Route::NorthAndSouth => Some(Heading::North),
                Route::SouthAndWest => Some(Heading::West),
                Route::SouthAndEast => Some(Heading::East),
                _ => None,
            },
            Heading::East => match self.route {
                Route::EastAndWest => Some(Heading::East),
                Route::NorthAndWest => Some(Heading::North),
                Route::SouthAndWest => Some(Heading::South),
                _ => None,
            },
            Heading::South => match self.route {
                Route::NorthAndSouth => Some(Heading::South),
                Route::NorthAndEast => Some(Heading::East),
                Route::NorthAndWest => Some(Heading::West),
                _ => None,
            },
            Heading::West => match self.route {
                Route::EastAndWest => Some(Heading::West),
                Route::NorthAndEast => Some(Heading::North),
                Route::SouthAndEast => Some(Heading::South),
                _ => None,
            },
        }
    }
}

/// Shoelace Formula (https://en.wikipedia.org/wiki/Shoelace_formula)
/// The starting point must be present at the start and the end of the outline
fn shoelace(outline: &[Point]) -> usize {
    let sum = outline.windows(2).fold(0, |acc, matrix| {
        acc + (matrix[0].0 * matrix[1].1) - (matrix[1].0 * matrix[0].1)
    });
    // If the points are labeled sequentially in the counterclockwise direction, then the area is positive,
    // if they are labeled in the clockwise direction, the area will be negative.
    (sum.abs() / 2) as usize
}

pub fn day10(input: Vec<String>) -> (String, String) {
    let mut points = HashMap::new();
    let mut start: Option<Point> = None;
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let point = Pipe::new(c);
            if point.route == Route::Start {
                start = Some((row as i32, col as i32));
            }
            points.insert((row as i32, col as i32), point);
        }
    }

    // Find a valid route from the start
    let mut heading = Heading::North;
    for h in [Heading::North, Heading::East, Heading::South, Heading::West] {
        let next_loc = h.from(start.expect("No starting coord"));
        if let Some(point) = points.get(&next_loc) {
            if point.get_heading(&h).is_some() {
                heading = h;
                break;
            }
        }
    }

    let mut steps = 1;
    let mut loc = heading.from(start.unwrap());
    let mut point = points.get(&loc).expect("No point at location");
    let mut outline = Vec::from([start.unwrap(), loc]);
    while point.route != Route::Start {
        heading = point.get_heading(&heading).expect("Heading not valid");
        loc = heading.from(loc);
        point = points.get(&loc).expect("No point at location");
        steps += 1;
        outline.push(loc);
    }
    let area: usize = shoelace(&outline);
    // Pick's Theorem (https://en.wikipedia.org/wiki/Pick%27s_theorem)
    let interior_points = area - (outline.len() - 1) / 2 + 1;

    (format!("{}", steps / 2), format!("{}", interior_points))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use test_case::test_case;

    #[test_case(
        "
        -L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF
        ",
        "4",
        "1"
        ;"1"
    )]
    #[test_case(
        "
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
        ",
        "8",
        "1"
        ;"2"
    )]
    #[test_case(
        "
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
        ",
        "23",
        "4"
        ;"3"
    )]
    #[test_case(
        "
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
        ",
        "80",
        "10"
        ;"4"
    )]
    fn example_both(input: &str, part_1: &str, part_2: &str) {
        let result_1 = day10(parser::test_input(input));
        assert_eq!(result_1.0, part_1);
        assert_eq!(result_1.1, part_2);
    }

    #[test]
    fn mainline() {
        let input = parser::load_input(10);
        let result = day10(input);
        assert_eq!(result.0, "6927");
        assert_eq!(result.1, "467");
    }
}
