use std::collections::HashMap;

struct Game {
    id: usize,
    red: usize,
    green: usize,
    blue: usize,
}

pub fn day02(input: Vec<String>) -> (String, String) {
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

    let mut sum = 0;
    for game in &games {
        if game.red <= 12 && game.green <= 13 && game.blue <= 14 {
            sum += game.id;
        }
    }

    let mut power = 0;
    for game in games {
        power += game.red * game.green * game.blue;
    }

    (format!("{}", sum), format!("{}", power))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn example() {
        let result = day02(parser::test_input(
            "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ));
        assert_eq!(result.0, "8");
        assert_eq!(result.1, "2286");
    }

    #[test]
    fn mainline() {
        let input = parser::load_input(2);
        let result = day02(input);
        assert_eq!(result.0, "2278");
        assert_eq!(result.1, "67953");
    }
}
