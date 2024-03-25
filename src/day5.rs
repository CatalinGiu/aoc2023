use std::{cmp::min, fs::File, io, usize};

#[derive(Debug)]
struct Entry {
    destination: u64,
    source: u64,
    range: u64,
}

#[derive(Debug)]
struct Map {
    mappings: Vec<Entry>,
}

impl Map {
    fn from_lines(lines: &mut io::Lines<io::BufReader<File>>) -> Self {
        let mut mappings: Vec<Entry> = Vec::new();
        loop {
            let line;
            let mapping: Vec<&str> = match lines.next() {
                Some(l) => {
                    line = l.expect("wat??");
                    line.split(" ").collect()
                }
                None => break,
            };

            if let [dest, src, range] = mapping.as_slice() {
                mappings.push(Entry {
                    destination: dest.parse::<u64>().unwrap(),
                    source: src.parse::<u64>().unwrap(),
                    range: range.parse::<u64>().unwrap(),
                });
            } else {
                break;
            }
        }
        Map { mappings }
    }
}

#[derive(Debug)]
struct Farm {
    seeds: Vec<u64>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Farm {
    fn from_lines(mut lines: io::Lines<io::BufReader<File>>) -> Self {
        let mut seeds: Vec<u64> = Vec::new();
        // first line is seeds
        let seeds_line: String = lines.next().unwrap().expect("no seed?");
        let seeds_str = seeds_line.split_once(":").unwrap().1.trim().split(" ");

        for seed in seeds_str {
            seeds.push(seed.parse::<u64>().unwrap())
        }

        lines.next();
        lines.next();
        let seed_to_soil = Map::from_lines(&mut lines);
        lines.next();
        let soil_to_fertilizer = Map::from_lines(&mut lines);
        lines.next();
        let fertilizer_to_water = Map::from_lines(&mut lines);
        lines.next();
        let water_to_light = Map::from_lines(&mut lines);
        lines.next();
        let light_to_temperature = Map::from_lines(&mut lines);
        lines.next();
        let temperature_to_humidity = Map::from_lines(&mut lines);
        lines.next();
        let humidity_to_location = Map::from_lines(&mut lines);

        Farm {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    fn find_closest_location_part1(&self) -> u64 {
        let mut min_location: u64 = std::u64::MAX;

        for seed in &self.seeds {
            let mut seed_pos = *seed;
            for map in &self.seed_to_soil.mappings {
                if map.source <= seed_pos && seed_pos < map.source + map.range {
                    seed_pos = seed_pos - map.source + map.destination;
                    break;
                }
            }
            for map in &self.soil_to_fertilizer.mappings {
                if map.source <= seed_pos && seed_pos < map.source + map.range {
                    seed_pos = seed_pos - map.source + map.destination;
                    break;
                }
            }
            for map in &self.fertilizer_to_water.mappings {
                if map.source <= seed_pos && seed_pos < map.source + map.range {
                    seed_pos = seed_pos - map.source + map.destination;
                    break;
                }
            }
            for map in &self.water_to_light.mappings {
                if map.source <= seed_pos && seed_pos < map.source + map.range {
                    seed_pos = seed_pos - map.source + map.destination;
                    break;
                }
            }
            for map in &self.light_to_temperature.mappings {
                if map.source <= seed_pos && seed_pos < map.source + map.range {
                    seed_pos = seed_pos - map.source + map.destination;
                    break;
                }
            }
            for map in &self.temperature_to_humidity.mappings {
                if map.source <= seed_pos && seed_pos < map.source + map.range {
                    seed_pos = seed_pos - map.source + map.destination;
                    break;
                }
            }
            for map in &self.humidity_to_location.mappings {
                if map.source <= seed_pos && seed_pos < map.source + map.range {
                    seed_pos = seed_pos - map.source + map.destination;
                    break;
                }
            }
            min_location = min(min_location, seed_pos);
        }

        min_location
    }

    fn find_closest_location_part2(&self) -> u64 {
        let mut seed_ranges: Vec<SeedRange> = Vec::new();

        for seed in self.seeds.chunks(2) {
            seed_ranges.push(SeedRange {
                start: seed[0],
                end: seed[0] + seed[1],
            });
        }

        let mut new_ranges = Vec::new();
        for mut seed in seed_ranges {
            new_ranges.extend(seed.overlap(&self.seed_to_soil))
        }

        seed_ranges = new_ranges;
        new_ranges = Vec::new();
        for mut seed in seed_ranges {
            new_ranges.extend(seed.overlap(&self.soil_to_fertilizer))
        }

        seed_ranges = new_ranges;
        new_ranges = Vec::new();
        for mut seed in seed_ranges {
            new_ranges.extend(seed.overlap(&self.fertilizer_to_water))
        }

        seed_ranges = new_ranges;
        new_ranges = Vec::new();
        for mut seed in seed_ranges {
            new_ranges.extend(seed.overlap(&self.water_to_light))
        }

        seed_ranges = new_ranges;
        new_ranges = Vec::new();
        for mut seed in seed_ranges {
            new_ranges.extend(seed.overlap(&self.light_to_temperature))
        }

        seed_ranges = new_ranges;
        new_ranges = Vec::new();
        for mut seed in seed_ranges {
            new_ranges.extend(seed.overlap(&self.temperature_to_humidity))
        }

        seed_ranges = new_ranges;
        new_ranges = Vec::new();
        for mut seed in seed_ranges {
            new_ranges.extend(seed.overlap(&self.humidity_to_location))
        }

        let mut min_location = std::u64::MAX;
        for seed in new_ranges {
            min_location = min(min_location, seed.start);
        }
        min_location
    }
}

#[derive(Debug, Clone)]
struct SeedRange {
    start: u64,
    end: u64,
}

impl SeedRange {
    fn overlap(&mut self, map: &Map) -> Vec<SeedRange> {
        let mut ret = Vec::new();

        for map in &map.mappings {
            if self.end < map.source || map.source + map.range < self.start {
                // no overlap
                continue;
            }

            if self.start > map.source && self.end < map.source + map.range {
                ret.push(SeedRange {
                    start: self.start + map.destination - map.source,
                    end: self.end + map.destination - map.source,
                });
                return ret;
            } else {
                if self.start < map.source {
                    ret.push(SeedRange {
                        start: self.start,
                        end: map.source - 1,
                    });
                    self.start = map.source;
                }

                if self.end > map.source + map.range {
                    ret.push(SeedRange {
                        start: map.source + map.range + 1,
                        end: self.end,
                    });
                    self.end = map.source + map.range;
                }

                if self.start >= map.source && self.end <= map.source + map.range {
                    ret.push(SeedRange {
                        start: self.start + map.destination - map.source,
                        end: self.end + map.destination - map.source,
                    });
                    self.start = 0;
                    self.end = 0;
                }
            }
        }

        if self.start != self.end {
            ret.push(self.clone());
        }
        ret
    }
}

pub fn aoc_05_part1(lines: io::Lines<io::BufReader<File>>) -> io::Result<usize> {
    let farm = Farm::from_lines(lines);
    Ok(farm.find_closest_location_part1().try_into().unwrap())
}

pub fn aoc_05_part2(lines: io::Lines<io::BufReader<File>>) -> io::Result<usize> {
    let farm = Farm::from_lines(lines);
    Ok(farm.find_closest_location_part2().try_into().unwrap())
    // Ok(0)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        assert!(true)
    }
}
