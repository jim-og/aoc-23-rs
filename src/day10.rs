use std::collections::HashMap;

enum Heading {
    North,
    East,
    South,
    West,
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

struct Point {
    route: Route,
}

impl Point {
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

fn get_next_loc(loc: (i32, i32), heading: &Heading) -> (i32, i32) {
    match heading {
        Heading::North => (loc.0 - 1, loc.1),
        Heading::East => (loc.0, loc.1 + 1),
        Heading::South => (loc.0 + 1, loc.1),
        Heading::West => (loc.0, loc.1 - 1),
    }
}

pub fn day10(input: Vec<String>) -> (String, String) {
    let mut points = HashMap::new();
    let mut start: Option<(i32, i32)> = None;
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let point = Point::new(c);
            if point.route == Route::Start {
                start = Some((row as i32, col as i32));
            }
            points.insert((row as i32, col as i32), point);
        }
    }

    // Find a valid route from the start
    let mut heading = Heading::North;
    for h in [Heading::North, Heading::East, Heading::South, Heading::West] {
        let next_loc = get_next_loc(start.expect("No starting coord"), &h);
        if let Some(point) = points.get(&next_loc) {
            if point.get_heading(&h).is_some() {
                heading = h;
                break;
            }
        }
    }

    let mut steps = 1;
    let mut loc = get_next_loc(start.unwrap(), &heading);
    let mut point = points.get(&loc).expect("No point at location");
    while point.route != Route::Start {
        heading = point.get_heading(&heading).expect("Heading not valid");
        loc = get_next_loc(loc, &heading);
        point = points.get(&loc).expect("No point at location");
        steps += 1;
    }

    (format!("{}", steps / 2), format!("{}", ""))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn example_both() {
        let result_1 = day10(vec![
            "-L|F7".to_string(),
            "7S-7|".to_string(),
            "L|7||".to_string(),
            "-L-J|".to_string(),
            "L|-JF".to_string(),
        ]);
        assert_eq!(result_1.0, "4");
        assert_eq!(result_1.1, "");

        let result_2 = day10(vec![
            "7-F7-".to_string(),
            ".FJ|7".to_string(),
            "SJLL7".to_string(),
            "|F--J".to_string(),
            "LJ.LJ".to_string(),
        ]);
        assert_eq!(result_2.0, "8");
        assert_eq!(result_2.1, "");
    }

    #[test]
    fn mainline() {
        let input = parser::load_input(10);
        let result = day10(input);
        assert_eq!(result.0, "6927");
        assert_eq!(result.1, "");
    }
}
