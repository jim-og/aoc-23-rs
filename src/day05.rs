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
            _ => Self::HumidityToLocation,
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

    pub fn source_to_destination(&self, s: u64) -> Option<u64> {
        if s >= self.source && s < self.source + self.range {
            return Some(self.destination + s - self.source);
        }
        None
    }

    pub fn destination_to_source(&self, d: u64) -> Option<u64> {
        if d >= self.destination && d < self.destination + self.range {
            return Some(self.source + d - self.destination);
        }
        None
    }
}

struct SeedRange {
    start: u64,
    range: u64,
}

impl SeedRange {
    pub fn new(start: u64, range: u64) -> Self {
        Self { start, range }
    }

    pub fn contains(&self, v: u64) -> bool {
        v >= self.start && v < self.start + self.range
    }
}

fn get_destination(source: u64, mappings: &Vec<Mapping>) -> u64 {
    for mapping in mappings {
        if let Some(destination) = mapping.source_to_destination(source) {
            return destination;
        }
    }
    source
}

fn get_source(destination: u64, mappings: &Vec<Mapping>) -> u64 {
    for mapping in mappings {
        if let Some(source) = mapping.destination_to_source(destination) {
            return source;
        }
    }
    destination
}

#[derive(Default)]
pub struct Data {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Data {
    let mut res = Data::default();
    let mut state: State = State::new();

    for line in input.trim().lines().map(|l| l.trim()) {
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
                res.seeds = data.clone();
                None
            }
            State::SeedToSoil => Some(&mut res.seed_to_soil),
            State::SoilToFertilizer => Some(&mut res.soil_to_fertilizer),
            State::FertilizerToWater => Some(&mut res.fertilizer_to_water),
            State::WaterToLight => Some(&mut res.water_to_light),
            State::LightToTemperature => Some(&mut res.light_to_temperature),
            State::TemperatureToHumidity => Some(&mut res.temperature_to_humidity),
            State::HumidityToLocation => Some(&mut res.humidity_to_location),
        };

        // Fill the map
        if let Some(m) = mapping_to_fill {
            m.push(Mapping::new(data[0], data[1], data[2]));
        }
    }

    res
}

#[aoc(day5, part1)]
pub fn part1(input: &Data) -> u64 {
    let mut part_1_min_location = u64::MAX;
    for seed in input.seeds.clone() {
        let soil = get_destination(seed, &input.seed_to_soil);
        let fertilizer = get_destination(soil, &input.soil_to_fertilizer);
        let water = get_destination(fertilizer, &input.fertilizer_to_water);
        let light = get_destination(water, &input.water_to_light);
        let temperature = get_destination(light, &input.light_to_temperature);
        let humidity = get_destination(temperature, &input.temperature_to_humidity);
        let location = get_destination(humidity, &input.humidity_to_location);
        part_1_min_location = cmp::min(part_1_min_location, location);
    }
    part_1_min_location
}

#[aoc(day5, part2)]
pub fn part2(input: &Data) -> u64 {
    // Given the now vast range of input seeds it's much quicker to perform the search in reverse, checking locations
    // from 0..u64::MAX until a matching seed is found.
    let mut seed_ranges = Vec::new();
    let mut iter = input.seeds.iter();
    let mut seed = iter.next();
    let mut range = iter.next();

    // Reinterpet the "seeds:" line as a range of seeds
    while let (Some(s), Some(r)) = (seed, range) {
        seed_ranges.push(SeedRange::new(*s, *r));
        seed = iter.next();
        range = iter.next();
    }

    // Work backwards from location to find the first seed which lies within one of the ranges
    for location in 0..u64::MAX {
        let humidity = get_source(location, &input.humidity_to_location);
        let temperature = get_source(humidity, &input.temperature_to_humidity);
        let light = get_source(temperature, &input.light_to_temperature);
        let water = get_source(light, &input.water_to_light);
        let fertilizer = get_source(water, &input.fertilizer_to_water);
        let soil = get_source(fertilizer, &input.soil_to_fertilizer);
        let seed = get_source(soil, &input.seed_to_soil);
        for seed_range in &seed_ranges {
            if seed_range.contains(seed) {
                return location;
            }
        }
    }

    // Shouldn't reach
    u64::MAX
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn mapping_source_to_destination() {
        let mapping = Mapping::new(50, 98, 2);
        assert!(mapping.source_to_destination(97).is_none());
        assert_eq!(mapping.source_to_destination(98).unwrap(), 50);
        assert_eq!(mapping.source_to_destination(99).unwrap(), 51);
        assert!(mapping.source_to_destination(100).is_none());
    }

    #[test]
    fn mapping_destination_to_source() {
        let mapping = Mapping::new(50, 98, 2);
        assert!(mapping.destination_to_source(49).is_none());
        assert_eq!(mapping.destination_to_source(50).unwrap(), 98);
        assert_eq!(mapping.destination_to_source(51).unwrap(), 99);
        assert!(mapping.destination_to_source(52).is_none());
    }

    #[test]
    fn mapping_get_destination() {
        let mappings = Vec::from([Mapping::new(50, 98, 2), Mapping::new(52, 50, 48)]);
        assert_eq!(get_destination(0, &mappings), 0);
        assert_eq!(get_destination(1, &mappings), 1);
        assert_eq!(get_destination(48, &mappings), 48);
        assert_eq!(get_destination(49, &mappings), 49);
        assert_eq!(get_destination(50, &mappings), 52);
        assert_eq!(get_destination(51, &mappings), 53);
        assert_eq!(get_destination(96, &mappings), 98);
        assert_eq!(get_destination(97, &mappings), 99);
        assert_eq!(get_destination(98, &mappings), 50);
        assert_eq!(get_destination(99, &mappings), 51);
        assert_eq!(get_destination(100, &mappings), 100);
    }

    #[test]
    fn mapping_get_source() {
        let mappings = Vec::from([Mapping::new(50, 98, 2), Mapping::new(52, 50, 48)]);
        assert_eq!(get_source(0, &mappings), 0);
        assert_eq!(get_source(1, &mappings), 1);
        assert_eq!(get_source(48, &mappings), 48);
        assert_eq!(get_source(49, &mappings), 49);
        assert_eq!(get_source(52, &mappings), 50);
        assert_eq!(get_source(53, &mappings), 51);
        assert_eq!(get_source(98, &mappings), 96);
        assert_eq!(get_source(99, &mappings), 97);
        assert_eq!(get_source(50, &mappings), 98);
        assert_eq!(get_source(51, &mappings), 99);
        assert_eq!(get_source(100, &mappings), 100);
    }

    #[test]
    fn both_test() {
        let input = parse(
            "seeds: 79 14 55 13
            
            seed-to-soil map:
            50 98 2
            52 50 48
            
            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15
            
            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4
            
            water-to-light map:
            88 18 7
            18 25 70
           
            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69
         
            humidity-to-location map:
            60 56 37
            56 93 4",
        );
        assert_eq!(part1(&input), 35);
        assert_eq!(part2(&input), 46);
    }

    #[test]
    fn mainline() {
        let input = parse(&parser::load_input_string(5));
        assert_eq!(part1(&input), 462648396);
        assert_eq!(part2(&input), 2520479);
    }
}
