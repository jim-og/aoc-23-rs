use std::collections::HashMap;

use crate::parser;

pub struct Game {
    id: usize,
    red: usize,
    green: usize,
    blue: usize,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    let input = parser::test_input(input);
    let mut games = Vec::new();

    for (index, game) in input.iter().enumerate() {
        let mut required_colours = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        let draws = game.split([':', ';']).collect::<Vec<&str>>();
        for draw in draws {
            if draw.contains("Game") {
                continue;
            }
            for result in draw.split(',').collect::<Vec<&str>>() {
                let number_colour = result.trim().split(' ').collect::<Vec<&str>>();
                let number = number_colour
                    .first()
                    .expect("Failed to extract number of colour")
                    .parse::<usize>()
                    .expect("Failed to parse number of colour");
                let colour = number_colour.get(1).expect("Failed to extract colour");
                if *required_colours.get(colour).unwrap() < number {
                    required_colours.insert(colour, number);
                }
            }
        }
        games.push(Game {
            id: index + 1,
            red: *required_colours.get("red").unwrap(),
            green: *required_colours.get("green").unwrap(),
            blue: *required_colours.get("blue").unwrap(),
        });
    }
    games
}

#[aoc(day2, part1)]
pub fn part1(games: &Vec<Game>) -> usize {
    let mut sum = 0;
    for game in games {
        if game.red <= 12 && game.green <= 13 && game.blue <= 14 {
            sum += game.id;
        }
    }
    sum
}

#[aoc(day2, part2)]
pub fn part2(games: &Vec<Game>) -> usize {
    let mut power = 0;
    for game in games {
        power += game.red * game.green * game.blue;
    }
    power
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    const TEST: &str = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_test() {
        assert_eq!(part1(&input_generator(TEST)), 8);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&input_generator(TEST)), 2286);
    }

    #[test]
    fn mainline() {
        let input = parser::load_input_string(2);
        let games = input_generator(&input);
        assert_eq!(part1(&games), 2278);
        assert_eq!(part2(&games), 67953);
    }
}
