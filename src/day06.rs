struct Race {
    time: u64,
    record: u64,
}

impl Race {
    // The winning options are symmetrical i.e. there will be the same number of
    // losses on either side of the distribution, so just find when the record is
    // first broken.
    pub fn ways_to_win(&self) -> u64 {
        let mut result = 0;
        for time_charging in 1..self.time {
            let time_remaining = self.time - time_charging;
            let distance = time_charging * time_remaining;
            if distance > self.record {
                result = self.time - (2 * time_charging) + 1;
                break;
            }
        }
        result
    }
}

fn parse_input_part_1(index: usize, input: &[String]) -> Vec<u64> {
    input
        .get(index)
        .expect("Failed to extract input line")
        .split(':')
        .collect::<Vec<&str>>()
        .get(1)
        .expect("Failed to extract values")
        .split_whitespace()
        .map(|v| v.parse::<u64>().expect("Failed to parse values"))
        .collect::<Vec<u64>>()
}

fn parse_input_part_2(index: usize, input: &[String]) -> u64 {
    input
        .get(index)
        .expect("Failed to extract input line")
        .split(':')
        .collect::<Vec<&str>>()
        .get(1)
        .expect("Failed to extract values")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .concat()
        .parse::<u64>()
        .expect("Failed to parse values")
}

pub fn day06(input: Vec<String>) -> (String, String) {
    let times = parse_input_part_1(0, &input);
    let records = parse_input_part_1(1, &input);
    let mut races = Vec::new();
    for i in 0..times.len() {
        races.push(Race {
            time: *times.get(i).expect("Expected time data"),
            record: *records.get(i).expect("Expected record data"),
        });
    }

    let mut part_1 = Vec::new();
    for race in &races {
        part_1.push(race.ways_to_win());
    }

    let time = parse_input_part_2(0, &input);
    let record = parse_input_part_2(1, &input);
    let part_2 = Race { time, record }.ways_to_win();

    (
        format!("{}", part_1.iter().product::<u64>()),
        format!("{}", part_2),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn example_both() {
        let result = day06(vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ]);
        assert_eq!(result.0, "288");
        assert_eq!(result.1, "71503");
    }

    #[test]
    fn mainline() {
        let input = parser::load_input(6);
        let result = day06(input);
        assert_eq!(result.0, "781200");
        assert_eq!(result.1, "49240091");
    }
}
