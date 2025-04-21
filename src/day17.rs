use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};

type Point = (usize, usize);

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Start,
    North,
    East,
    South,
    West,
}

trait Flippable {
    fn flip(&self) -> Direction;
}

impl Flippable for Direction {
    fn flip(&self) -> Direction {
        match self {
            Direction::Start => Direction::Start,
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct Node {
    pos: Point,
    f: usize,
    g: usize,
    dir: Direction,
    steps: usize,
    parent: NodeKey,
}

impl Node {
    fn from(
        pos: Point,
        dest: Point,
        g: usize,
        dir: Direction,
        steps: usize,
        parent: NodeKey,
    ) -> Node {
        let h = heuristic(pos, dest);
        Node {
            pos,
            f: g + h,
            g,
            dir,
            steps,
            parent,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
struct NodeKey {
    pos: Point,
    dir: Direction,
    steps: usize,
}

impl NodeKey {
    fn from(pos: Point, dir: Direction, steps: usize) -> NodeKey {
        NodeKey { pos, dir, steps }
    }
}

trait OpenList {
    fn initialise(city: &City, start: Point, dest: Point) -> Self;
}

impl OpenList for BinaryHeap<Node> {
    fn initialise(city: &City, start: Point, dest: Point) -> Self {
        let mut open = BinaryHeap::new();

        for (pos, dir) in city.get_neighbours(start) {
            let g = city.map.get(&pos).expect("Expected point in city map");
            open.push(Node::from(
                pos,
                dest,
                *g,
                dir,
                1,
                NodeKey::from(start, Direction::Start, 0),
            ));
        }

        open
    }
}

trait ClosedList {
    fn initialise(city: &City, start: Point, dest: Point) -> Self;
    #[allow(dead_code)]
    fn get_path(&self, last_node: Node) -> Vec<Node>;
    #[allow(dead_code)]
    fn get_visited(&self) -> HashSet<Point>;
}

impl ClosedList for HashMap<NodeKey, Node> {
    fn initialise(city: &City, start: Point, dest: Point) -> Self {
        let mut closed = HashMap::new();
        for (_, dir) in city.get_neighbours(start) {
            closed.insert(
                NodeKey::from(start, dir, 0),
                Node::from(
                    start,
                    dest,
                    0,
                    dir,
                    0,
                    NodeKey::from(start, Direction::Start, 0),
                ),
            );
        }
        closed
    }

    fn get_path(&self, last_node: Node) -> Vec<Node> {
        let mut path = Vec::new();
        let mut curr_node = last_node;
        while curr_node.parent.dir != Direction::Start {
            path.push(curr_node);
            curr_node = *self
                .get(&curr_node.parent)
                .expect("Node parent does not exist.");
        }
        path.push(curr_node);
        path
    }

    fn get_visited(&self) -> HashSet<Point> {
        self.keys().map(|k| k.pos).collect()
    }
}

struct City {
    map: HashMap<Point, usize>,
    max: Point,
}

impl City {
    fn a_star_search(
        &self,
        start: Point,
        dest: Point,
        max_steps: usize,
        min_steps: usize,
    ) -> Option<usize> {
        // OPEN list is a min-heap with lowest f at the top
        let mut open = BinaryHeap::initialise(self, start, dest);

        // CLOSED list holds the visited nodes
        let mut closed = HashMap::initialise(self, start, dest);

        while let Some(node) = open.pop() {
            // Destination reached
            if node.pos == dest && node.steps >= min_steps {
                // self.print_path(closed.get_path(node));
                // self.print_visited(closed.get_visited());
                return Some(node.g);
            }

            let node_key = NodeKey::from(node.pos, node.dir, node.steps);

            // Skip if we've already visited this node
            if closed.contains_key(&node_key) {
                continue;
            }

            for (pos, dir) in self.get_neighbours(node.pos) {
                // Skip a neighbour which would be going back on ourselves
                if dir == node.dir.flip() {
                    continue;
                }

                // Skip changes in direction if min steps have not been taken
                if dir != node.dir && node.steps < min_steps {
                    continue;
                }

                let steps = if dir == node.dir { node.steps + 1 } else { 1 };

                // Skip neighbours which exceed the single direction limit
                if steps > max_steps {
                    continue;
                }

                // If neighbour exists in CLOSED list with <= g, skip
                if closed
                    .get(&NodeKey::from(pos, dir, steps))
                    .is_some_and(|&n| n.g <= node.g)
                {
                    continue;
                }

                let heat_loss = node.g + self.map.get(&pos).expect("Expected point in city map");

                // Add neighbour to OPEN list
                open.push(Node::from(pos, dest, heat_loss, dir, steps, node_key));

                // Add node to CLOSED list
                closed.insert(node_key, node);
            }
        }

        None
    }

    fn get_neighbours(&self, pos: Point) -> Vec<(Point, Direction)> {
        let mut neighbours = Vec::new();

        if pos.0 > 0 {
            neighbours.push(((pos.0 - 1, pos.1), Direction::West));
        }
        if pos.0 < self.max.0 {
            neighbours.push(((pos.0 + 1, pos.1), Direction::East));
        }
        if pos.1 > 0 {
            neighbours.push(((pos.0, pos.1 - 1), Direction::North));
        }
        if pos.1 < self.max.1 {
            neighbours.push(((pos.0, pos.1 + 1), Direction::South));
        }

        neighbours
    }

    #[allow(dead_code)]
    fn print_path(&self, path: Vec<Node>) {
        let mut grid: HashMap<Point, String> = self.get_grid();

        for node in path {
            let symbol = match node.dir {
                Direction::Start => "#",
                Direction::North => "^",
                Direction::East => ">",
                Direction::South => "v",
                Direction::West => "<",
            };
            grid.insert(node.pos, symbol.to_string());
        }

        self.print_grid(grid);
    }

    #[allow(dead_code)]
    fn print_visited(&self, visited: HashSet<Point>) {
        let mut grid: HashMap<Point, String> = self.get_grid();

        for point in visited {
            grid.insert(point, "#".to_string());
        }

        self.print_grid(grid);
    }

    fn get_grid(&self) -> HashMap<Point, String> {
        self.map
            .iter()
            .map(|(&point, &_val)| (point, ".".to_string()))
            .collect()
    }

    fn print_grid(&self, grid: HashMap<Point, String>) {
        for row in 0..=self.max.1 {
            let line: String = (0..=self.max.0)
                .map(|col| {
                    grid.get(&(col, row))
                        .expect("Expected to find block in grid")
                })
                .cloned()
                .collect();
            println!("{}", line);
        }
        println!();
    }
}

fn heuristic(pos: Point, dest: Point) -> usize {
    // Taxicab distance
    let dx = (pos.0 as isize - dest.0 as isize).unsigned_abs();
    let dy = (pos.1 as isize - dest.1 as isize).unsigned_abs();
    dx + dy
}

impl Ord for Node {
    // min-heap based on f
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f.cmp(&other.f).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day17)]
fn parse(input: &str) -> City {
    // hashmap of coords and heat-loss
    let map: HashMap<(usize, usize), usize> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.trim().chars().enumerate().map(move |(col, c)| {
                (
                    (col, row),
                    c.to_string()
                        .parse::<usize>()
                        .expect("Error parsing block heat loss"),
                )
            })
        })
        .collect();

    let max = map.keys().cloned().max().unwrap_or((0, 0));

    City { map, max }
}

#[aoc(day17, part1)]
fn part1(city: &City) -> usize {
    city.a_star_search((0, 0), city.max, 3, 0)
        .expect("Expected to reach the end!")
}

#[aoc(day17, part2)]
fn part2(city: &City) -> usize {
    city.a_star_search((0, 0), city.max, 10, 4)
        .expect("Expected to reach the end!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use test_case::test_case;

    const TEST_A: &str = "
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    ";

    const TEST_B: &str = "
        111111111111
        999999999991
        999999999991
        999999999991
        999999999991
    ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST_A)), 102);
    }

    #[test_case(TEST_A, 94;"A")]
    #[test_case(TEST_B, 71;"B")]
    fn part2_example(input: &str, answer: usize) {
        assert_eq!(part2(&parse(input)), answer);
    }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input_string(17));
        assert_eq!(part1(input), 1138);
        assert_eq!(part2(input), 1312);
    }
}
