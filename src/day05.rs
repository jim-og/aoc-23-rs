use std::cmp;

enum State {
    Seeds,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
    Done,
}

impl State {
    pub fn new() -> Self {
        Self::Seeds
    }

    pub fn next(self) -> Self {
        match self {
            Self::Seeds => Self::SeedToSoil,
            Self::SeedToSoil => Self::SoilToFertilizer,
            Self::SoilToFertilizer => Self::FertilizerToWater,
            Self::FertilizerToWater => Self::WaterToLight,
            Self::WaterToLight => Self::LightToTemperature,
            Self::LightToTemperature => Self::TemperatureToHumidity,
            Self::TemperatureToHumidity => Self::HumidityToLocation,
            Self::HumidityToLocation => Self::Done,
            Self::Done => Self::Done,
        }
    }
}

struct Mapping {
    destination: u64,
    source: u64,
    range: u64,
}

impl Mapping {
    pub fn new(destination: u64, source: u64, range: u64) -> Self {
        Self {
            destination,
            source,
            range,
        }
    }

    pub fn conversion(&self, s: u64) -> Option<u64> {
        if s >= self.source && s < self.source + self.range {
            return Some(self.destination + s - self.source);
        }
        None
    }
}

fn lookup(input: u64, mappings: &Vec<Mapping>) -> u64 {
    for mapping in mappings {
        if let Some(result) = mapping.conversion(input) {
            return result;
        }
    }
    input
}

pub fn day05(input: Vec<String>) -> (String, String) {
    let mut seeds = Vec::new();
    let mut seed_to_soil = Vec::new();
    let mut soil_to_fertilizer = Vec::new();
    let mut fertilizer_to_water = Vec::new();
    let mut water_to_light = Vec::new();
    let mut light_to_temperature = Vec::new();
    let mut temperature_to_humidity = Vec::new();
    let mut humidity_to_location = Vec::new();

    let mut state = State::new();
    for line in input {
        // Transition to next state
        if line.is_empty() {
            state = state.next();
            continue;
        }

        // Skip lines which don't contain numbers - except for the first line!
        if let Some(c) = line.chars().next() {
            match state {
                State::Seeds => (),
                _ => {
                    if !c.is_numeric() {
                        continue;
                    }
                }
            }
        }

        // Extract numbers from line
        let data = match state {
            State::Seeds => line
                .split([':'])
                .collect::<Vec<&str>>()
                .get(1)
                .expect("Couldn't split seeds")
                .split_whitespace()
                .map(|v| v.parse::<u64>().expect("Couldn't parse seeds"))
                .collect::<Vec<u64>>(),
            _ => line
                .split_whitespace()
                .map(|v| v.parse::<u64>().expect("Couldn't parse mapping"))
                .collect::<Vec<u64>>(),
        };

        // Find which map should be filled
        let mapping_to_fill = match state {
            State::Seeds => {
                seeds = data.clone();
                None
            }
            State::SeedToSoil => Some(&mut seed_to_soil),
            State::SoilToFertilizer => Some(&mut soil_to_fertilizer),
            State::FertilizerToWater => Some(&mut fertilizer_to_water),
            State::WaterToLight => Some(&mut water_to_light),
            State::LightToTemperature => Some(&mut light_to_temperature),
            State::TemperatureToHumidity => Some(&mut temperature_to_humidity),
            State::HumidityToLocation => Some(&mut humidity_to_location),
            _ => None,
        };

        // Fill the map
        if let Some(m) = mapping_to_fill {
            m.push(Mapping::new(data[0], data[1], data[2]));
        }
    }

    // Part 1
    let mut part_1_min_location = u64::MAX;
    for seed in seeds.clone() {
        let soil = lookup(seed, &seed_to_soil);
        let fertilizer = lookup(soil, &soil_to_fertilizer);
        let water = lookup(fertilizer, &fertilizer_to_water);
        let light = lookup(water, &water_to_light);
        let temperature = lookup(light, &light_to_temperature);
        let humidity = lookup(temperature, &temperature_to_humidity);
        let location = lookup(humidity, &humidity_to_location);
        part_1_min_location = cmp::min(part_1_min_location, location);
    }

    // Part 2
    // This method is super slow given the vast range of input seeds. The time to lookup each mapping is O(n).
    // This could be improved to O(1) by storing each individual mapping as a key in a HashMap but this would
    // require an enormous amount of space. Assuming the minimum location fits in u32, I suspect it would be
    // quicker to perform this search in reverse, checking all possible locations from 0..u32::MAX until a
    // matching seed is found.
    let mut part_2_min_location = u64::MAX;
    let mut iter = seeds.iter();
    let mut seed = iter.next();
    let mut range = iter.next();
    while let (Some(s), Some(r)) = (seed, range) {
        for i in s.clone()..(s + r) {
            let soil = lookup(i, &seed_to_soil);
            let fertilizer = lookup(soil, &soil_to_fertilizer);
            let water = lookup(fertilizer, &fertilizer_to_water);
            let light = lookup(water, &water_to_light);
            let temperature = lookup(light, &light_to_temperature);
            let humidity = lookup(temperature, &temperature_to_humidity);
            let location = lookup(humidity, &humidity_to_location);
            part_2_min_location = cmp::min(part_2_min_location, location);
        }
        seed = iter.next();
        range = iter.next();
    }

    (
        format!("{}", part_1_min_location),
        format!("{}", part_2_min_location),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapping_conversion() {
        let mapping = Mapping::new(50, 98, 2);
        assert!(mapping.conversion(97).is_none());
        assert_eq!(mapping.conversion(98).unwrap(), 50);
        assert_eq!(mapping.conversion(99).unwrap(), 51);
        assert!(mapping.conversion(100).is_none());
    }

    #[test]
    fn mapping_lookup() {
        let mappings = Vec::from([Mapping::new(50, 98, 2), Mapping::new(52, 50, 48)]);
        assert_eq!(lookup(0, &mappings), 0);
        assert_eq!(lookup(1, &mappings), 1);
        assert_eq!(lookup(48, &mappings), 48);
        assert_eq!(lookup(49, &mappings), 49);
        assert_eq!(lookup(50, &mappings), 52);
        assert_eq!(lookup(51, &mappings), 53);
        assert_eq!(lookup(96, &mappings), 98);
        assert_eq!(lookup(97, &mappings), 99);
        assert_eq!(lookup(98, &mappings), 50);
        assert_eq!(lookup(99, &mappings), 51);
        assert_eq!(lookup(100, &mappings), 100);
    }

    #[test]
    fn day05_1() {
        let result = day05(vec![
            "seeds: 79 14 55 13".to_string(),
            "".to_string(),
            "seed-to-soil map:".to_string(),
            "50 98 2".to_string(),
            "52 50 48".to_string(),
            "".to_string(),
            "soil-to-fertilizer map:".to_string(),
            "0 15 37".to_string(),
            "37 52 2".to_string(),
            "39 0 15".to_string(),
            "".to_string(),
            "fertilizer-to-water map:".to_string(),
            "49 53 8".to_string(),
            "0 11 42".to_string(),
            "42 0 7".to_string(),
            "57 7 4".to_string(),
            "".to_string(),
            "water-to-light map:".to_string(),
            "88 18 7".to_string(),
            "18 25 70".to_string(),
            "".to_string(),
            "light-to-temperature map:".to_string(),
            "45 77 23".to_string(),
            "81 45 19".to_string(),
            "68 64 13".to_string(),
            "".to_string(),
            "temperature-to-humidity map:".to_string(),
            "0 69 1".to_string(),
            "1 0 69".to_string(),
            "".to_string(),
            "humidity-to-location map:".to_string(),
            "60 56 37".to_string(),
            "56 93 4".to_string(),
        ]);
        assert_eq!(result.0, "35");
        assert_eq!(result.1, "46");
    }
}
