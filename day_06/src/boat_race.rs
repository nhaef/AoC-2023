#[derive(Debug)]
pub struct BoatRace {
    time: u64,
    distance: u64,
}

pub fn get_boat_races(input: &str) -> Vec<BoatRace> {
    let mut lines = input.lines();

    let time = lines
        .next()
        .expect(&format!("Could not read first line from input {}", input));
    let (_, time) = time
        .split_once(':')
        .expect(&format!("Could not find delimiter ':' in time {}", time));
    let time = time
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|item| {
            item.parse::<_>()
                .expect(&format!("Failed to parse item {}", item))
        });

    let distance = lines
        .next()
        .expect(&format!("Could not read second line from input {}", input));
    let (_, distance) = distance.split_once(':').expect(&format!(
        "Could not find delimiter ':' in distance {}",
        distance
    ));
    let distance = distance
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|item| {
            item.parse::<_>()
                .expect(&format!("Failed to parse item {}", item))
        });

    time.zip(distance)
        .map(|(time, distance)| BoatRace { time, distance })
        .collect()
}

pub fn get_boat_race(input: &str) -> BoatRace {
    let mut lines = input.lines();

    let time = lines
        .next()
        .expect(&format!("Could not read first line from input {}", input));
    let (_, time) = time
        .split_once(':')
        .expect(&format!("Could not find delimiter ':' in time {}", time));
    let time = time
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<_>()
        .expect(&format!("Failed to parse time {}", time));

    let distance = lines
        .next()
        .expect(&format!("Could not read second line from input {}", input));
    let (_, distance) = distance.split_once(':').expect(&format!(
        "Could not find delimiter ':' in distance {}",
        distance
    ));
    let distance = distance
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<_>()
        .expect(&format!("Failed to parse distance {}", distance));

    BoatRace { time, distance }
}

pub fn get_number_of_ways_to_win_race(boat_race: BoatRace) -> u64 {
    // some related notes
    // t1: time to hold button
    // t2: time to drive
    // v = a * t1
    // s = v * t2
    // s = a * t1 * t2
    // t = t1 + t2
    // t1 = t - t2
    // s = a * t1 * (t - t1)
    // s = a * t * t1 - a * t1^2
    // s = -at1^2 + att1
    // s' = -2at1 + at
    // 0 = -2at1 + at
    // -at = -2at1
    // 0.5t = t1
    // 0 = -a * t1^2 + at * t1 - s
    // 0 = t1^2 - t * t1 + s/a

    let p = (boat_race.time as f64) / 2.0;
    let q = boat_race.distance as f64;
    let sq = (p * p - q).sqrt();
    let min_charge_time = find_next_integer(p - sq);
    let max_charge_time = find_prev_integer(p + sq);

    max_charge_time - min_charge_time + 1
}

fn find_next_integer(value: f64) -> u64 {
    if value.fract() == 0.0 {
        return value as u64 + 1;
    }
    value.ceil() as u64
}

fn find_prev_integer(value: f64) -> u64 {
    if value.fract() == 0.0 {
        return value as u64 - 1;
    }
    value.floor() as u64
}
