use std::{ops::Range, str::Lines};

pub fn read_seeds(seeds: &str) -> Vec<i64> {
    let (_, seeds) = seeds
        .split_once(':')
        .expect(&format!("Could not find delimiter ':' in seeds {}", seeds));
    seeds
        .trim()
        .split(' ')
        .map(|seed| seed.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
        .expect(&format!("Failed to parse seeds {}", seeds))
}

pub fn read_seed_ranges(seeds: &str) -> Vec<Range<i64>> {
    let seeds = read_seeds(seeds);
    seeds
        .iter()
        .step_by(2)
        .zip(seeds.iter().skip(1).step_by(2))
        .map(|(range_start, range_length)| *range_start..(*range_start + *range_length))
        .collect()
}

pub fn get_lowest_seed_locations(seeds: Vec<i64>, lines: Lines) -> i64 {
    // define all map functions
    let mut map_fns = vec![];
    let mut last_line_empty = false;
    let mut current_map_description = None;
    let mut current_map_data = vec![];
    for line in lines.chain([""]) {
        if line.is_empty() {
            last_line_empty = true;
            if current_map_description.is_none() {
                continue;
            }
            // create map_fn for current map
            map_fns.push(move |value: i64| {
                let rule = current_map_data.clone();
                for (destination_range_start, source_range_start, range_length) in rule {
                    if source_range_start <= value && value < (source_range_start + range_length) {
                        return value + (destination_range_start - source_range_start);
                    }
                }
                value
            });
            current_map_data = vec![];
        } else if last_line_empty {
            last_line_empty = false;
            // expect map description
            let (map_description, _) = line
                .split_once(' ')
                .expect(&format!("Could not find delimiter ' ' in line {}", line));
            let (map_from, map_to) = map_description.split_once("-to-").expect(&format!(
                "Could not find delimiter \"-to-\" in line {}",
                line
            ));
            if let Some((_, map_to_previous)) = current_map_description {
                assert!(
                    map_to_previous == map_from,
                    "Invalid map_from (map_to_previous {}, map_from {})",
                    map_to_previous,
                    map_from
                );
            } else {
                assert!(
                    map_from == "seed",
                    "Invalid map_from for first map ({})",
                    map_from
                );
            }
            current_map_description = Some((map_from, map_to));
        } else {
            // expect map data
            let map_line_data = line
                .split_once(' ')
                .and_then(|(d, s_l)| s_l.split_once(' ').map(|(s, l)| (d, s, l)))
                .map(|(d, s, l)| {
                    (
                        d.parse::<i64>()
                            .expect(&format!("Failed to parse destination range start {}", d)),
                        s.parse::<i64>()
                            .expect(&format!("Failed to parse source range start {}", s)),
                        l.parse::<i64>()
                            .expect(&format!("Failed to parse range length {}", l)),
                    )
                })
                .expect(&format!("Could not find delimiter ' ' in line {}", line));
            current_map_data.push(map_line_data);
        }
    }
    let (_, map_to) = current_map_description.expect("Could not find any map");
    assert!(
        map_to == "location",
        "Invalid map_to for last map ({})",
        map_to
    );

    // use map_fns to map seeds to locations and find min
    let mut locations: Box<dyn Iterator<Item = _>> = Box::new(seeds.into_iter());
    for map_fn in map_fns {
        locations = Box::new(locations.map(map_fn));
    }
    locations
        .min()
        .expect("Failed to find min value in locations")
}

pub fn get_lowest_seed_locations_reverse(seed_ranges: Vec<Range<i64>>, lines: Lines) -> i64 {
    // define all map functions
    let mut map_fns = vec![];
    let mut last_line_empty = false;
    let mut current_map_description = None;
    let mut current_map_data = vec![];
    for line in lines.chain([""]) {
        if line.is_empty() {
            last_line_empty = true;
            if current_map_description.is_none() {
                continue;
            }
            // create map_fn for current map
            map_fns.push(move |value: i64| {
                let rule = current_map_data.clone();
                for (source_range_start, destination_range_start, range_length) in rule {
                    if source_range_start <= value && value < (source_range_start + range_length) {
                        return value + (destination_range_start - source_range_start);
                    }
                }
                value
            });
            current_map_data = vec![];
        } else if last_line_empty {
            last_line_empty = false;
            // expect map description
            let (map_description, _) = line
                .split_once(' ')
                .expect(&format!("Could not find delimiter ' ' in line {}", line));
            let (map_from, map_to) = map_description.split_once("-to-").expect(&format!(
                "Could not find delimiter \"-to-\" in line {}",
                line
            ));
            if let Some((_, map_to_previous)) = current_map_description {
                assert!(
                    map_to_previous == map_from,
                    "Invalid map_from (map_to_previous {}, map_from {})",
                    map_to_previous,
                    map_from
                );
            } else {
                assert!(
                    map_from == "seed",
                    "Invalid map_from for first map ({})",
                    map_from
                );
            }
            current_map_description = Some((map_from, map_to));
        } else {
            // expect map data
            let map_line_data = line
                .split_once(' ')
                .and_then(|(d, s_l)| s_l.split_once(' ').map(|(s, l)| (d, s, l)))
                .map(|(d, s, l)| {
                    (
                        d.parse::<i64>()
                            .expect(&format!("Failed to parse destination range start {}", d)),
                        s.parse::<i64>()
                            .expect(&format!("Failed to parse source range start {}", s)),
                        l.parse::<i64>()
                            .expect(&format!("Failed to parse range length {}", l)),
                    )
                })
                .expect(&format!("Could not find delimiter ' ' in line {}", line));
            current_map_data.push(map_line_data);
        }
    }
    let (_, map_to) = current_map_description.expect("Could not find any map");
    assert!(
        map_to == "location",
        "Invalid map_to for last map ({})",
        map_to
    );

    // search for the smallest location which maps to an existing seed
    map_fns.reverse();
    for location in 0..i64::MAX {
        let mut seed = location;
        for map_fn in &map_fns {
            seed = map_fn(seed);
        }
        for seed_range in &seed_ranges {
            if seed_range.contains(&seed) {
                return location;
            }
        }
    }
    panic!("...");
}
