// Template for a day
pub fn day00(_input: Vec<String>) -> (String, String) {
    let answer_1 = "ANSWER_1".to_string();
    let answer_2 = "ANSWER_2".to_string();
    (format!("{}", answer_1), format!("{}", answer_2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "template for a test"]
    #[test]
    fn example_both() {
        let result = day00(vec!["".to_string()]);
        assert_eq!(result.0, "ANSWER_1");
        assert_eq!(result.0, "ANSWER_2");
    }
}
