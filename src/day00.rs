pub fn day00(_input: Vec<String>) -> (String, String) {
    let answer_1 = "ANSWER_1".to_string();
    let answer_2 = "ANSWER_2".to_string();
    (format!("{}", answer_1), format!("{}", answer_2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day00_1() {
        assert_eq!(day00(vec!["".to_string()]).0, "ANSWER_1")
    }

    #[test]
    fn day00_2() {
        assert_eq!(day00(vec!["".to_string()]).1, "ANSWER_2")
    }
}
