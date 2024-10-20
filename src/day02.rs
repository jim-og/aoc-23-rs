pub fn day02(_input: Vec<String>) -> (String, String) {
    let answer_1 = "".to_string();
    let answer_2 = "".to_string();
    (format!("{}", answer_1), format!("{}", answer_2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_1() {
        assert_eq!(
            day02(vec![
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                    .to_string(),
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                    .to_string(),
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()
            ])
            .0,
            "8"
        )
    }
}
