use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

type Point = (i32, i32);

#[derive(Clone, Eq, Hash, PartialEq)]
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

#[derive(PartialEq, Clone)]
enum Route {
    NorthAndSouth,
    EastAndWest,
    Backslash,
    Forwardslash,
    Empty,
}

#[derive(Clone)]
struct Tile {
    route: Route,
    beams: HashSet<Heading>,
}

impl Tile {
    pub fn new(symbol: char) -> Self {
        let route = match symbol {
            '|' => Route::NorthAndSouth,
            '-' => Route::EastAndWest,
            '\\' => Route::Backslash,
            '/' => Route::Forwardslash,
            '.' => Route::Empty,
            _ => panic!("Unrecognised tile route: {}", symbol),
        };
        Self {
            route,
            beams: HashSet::new(),
        }
    }

    pub fn forward_beam(&mut self, heading: &Heading) -> bool {
        self.beams.insert(heading.clone())
    }

    pub fn is_energized(&self) -> bool {
        !self.beams.is_empty()
    }

    pub fn get_heading(&self, heading: Heading) -> Vec<Heading> {
        match heading {
            Heading::North => match self.route {
                Route::NorthAndSouth | Route::Empty => vec![heading],
                Route::EastAndWest => vec![Heading::East, Heading::West],
                Route::Backslash => vec![Heading::West],
                Route::Forwardslash => vec![Heading::East],
            },
            Heading::East => match self.route {
                Route::NorthAndSouth => vec![Heading::North, Heading::South],
                Route::EastAndWest | Route::Empty => vec![heading],
                Route::Backslash => vec![Heading::South],
                Route::Forwardslash => vec![Heading::North],
            },
            Heading::South => match self.route {
                Route::NorthAndSouth | Route::Empty => vec![heading],
                Route::EastAndWest => vec![Heading::East, Heading::West],
                Route::Backslash => vec![Heading::East],
                Route::Forwardslash => vec![Heading::West],
            },
            Heading::West => match self.route {
                Route::NorthAndSouth => vec![Heading::North, Heading::South],
                Route::EastAndWest | Route::Empty => vec![heading],
                Route::Backslash => vec![Heading::North],
                Route::Forwardslash => vec![Heading::South],
            },
        }
    }
}

struct Parser {
    layout: Layout,
    max: Point,
}

type Layout = HashMap<(i32, i32), Tile>;

trait Energized {
    fn energized_count(&self) -> usize;
}

impl Energized for Layout {
    fn energized_count(&self) -> usize {
        self.iter().filter(|(_, tile)| tile.is_energized()).count()
    }
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Parser {
    let mut layout = Layout::new();

    for (row, line) in input.trim().lines().enumerate() {
        for (col, c) in line.trim().chars().enumerate() {
            let tile = Tile::new(c);
            layout.insert((row as i32, col as i32), tile);
        }
    }

    let max = layout.keys().cloned().max().unwrap_or((0, 0));

    Parser { layout, max }
}

fn dfs(layout: &mut Layout, point: Point, heading: Heading) {
    if let Some(tile) = layout.get_mut(&point) {
        if tile.forward_beam(&heading) {
            for h in tile.get_heading(heading) {
                let next_point = h.from(point);
                dfs(layout, next_point, h);
            }
        }
    }
}

#[aoc(day16, part1)]
fn part1(input: &Parser) -> usize {
    let mut layout = input.layout.clone();
    dfs(&mut layout, (0, 0), Heading::East);
    layout.energized_count()
}

#[aoc(day16, part2)]
fn part2(input: &Parser) -> usize {
    let candidates: Vec<(Point, Heading)> = (0..input.max.1)
        .flat_map(|col| {
            vec![
                ((0, col), Heading::South),
                ((input.max.0, col), Heading::North),
            ]
        })
        .chain((0..input.max.0).flat_map(|row| {
            vec![
                ((row, 0), Heading::East),
                ((row, input.max.1), Heading::West),
            ]
        }))
        .collect();

    let mut result = 0;

    candidates.iter().for_each(|(point, heading)| {
        let mut layout = input.layout.clone();
        dfs(&mut layout, *point, heading.clone());
        result = max(result, layout.energized_count());
    });

    result
}

#[cfg(test)]
mod tests {
    use crate::parser;

    use super::*;

    const TEST: &str = r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
        ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST)), 46);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TEST)), 51);
    }

    #[test]
    fn mainline() {
        let input = &parser::load_input_string(16);
        assert_eq!(part1(&parse(input)), 6740);
        assert_eq!(part2(&parse(input)), 7041);
    }
}
