pub fn day01(input: Vec<String>) -> (String, String) {
    let mut sum = 0;
    for line in input {
        let mut digits = Vec::new();
        for character in line.chars() {
            if character.is_numeric() {
                digits.push(character);
            }
        }
        sum += format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
            .parse::<u32>()
            .unwrap();
    }
    let answer1 = sum;
    let answer2 = "TODO".to_string();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_1() {
        assert_eq!(
            day01(vec![
                "1abc2".to_string(),
                "pqr3stu8vwx".to_string(),
                "a1b2c3d4e5f".to_string(),
                "treb7uchet".to_string()
            ])
            .0,
            "142".to_string()
        )
    }
}
