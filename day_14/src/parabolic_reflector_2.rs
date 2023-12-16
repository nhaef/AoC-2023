use std::collections::HashMap;

#[derive(Clone)]
struct Interval(usize, usize);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position(usize, usize);

struct Platform {
    width: usize,
    height: usize,
    cycle: usize,
    rounded_rock_positions: Vec<Position>,
    vertical_intervals: Vec<Vec<Interval>>,
    horizontal_intervals: Vec<Vec<Interval>>,
}

impl Platform {
    fn state_hash(&self) -> usize {
        self.rounded_rock_positions
            .iter()
            .fold(0, |acc, p| acc + p.0 * p.1)
    }
    fn new(input: &str) -> Self {
        let lines = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
        let width = lines[0].len();
        let height = lines.len();
        const EMPTY_SPACE: u8 = b'.';
        const ROCK_CUBED: u8 = b'#';
        const ROCK_ROUND: u8 = b'O';
        let mut vertical_intervals = vec![vec![]; width];
        let mut horizontal_intervals = vec![vec![]; height];
        let mut current_vertical_intervals = vec![0; width];
        let mut current_horizontal_intervals = vec![0; height];
        let mut rounded_rock_positions = vec![];
        for (y, line) in lines.into_iter().enumerate() {
            for (x, char) in line.into_iter().enumerate() {
                match *char {
                    EMPTY_SPACE => (),
                    ROCK_CUBED => {
                        // handle vertical intervals
                        if current_vertical_intervals[x] == y {
                            // skip zero length vertical interval
                            current_vertical_intervals[x] += 1;
                        } else {
                            // finalize valid vertical interval
                            vertical_intervals[x].push(Interval(current_vertical_intervals[x], y));
                            current_vertical_intervals[x] = y + 1;
                        }
                        // handle horizontal intervals
                        if current_horizontal_intervals[y] == x {
                            // skip zero length horizontal interval
                            current_horizontal_intervals[y] += 1;
                        } else {
                            // finalize valid horizontal interval
                            horizontal_intervals[y]
                                .push(Interval(current_horizontal_intervals[y], x));
                            current_horizontal_intervals[y] = x + 1;
                        }
                    }
                    ROCK_ROUND => rounded_rock_positions.push(Position(x, y)),
                    c => panic!("unexpected char {}", c),
                }
            }
        }
        // finalize last vertical interval
        for (x, current_vertical_interval) in current_vertical_intervals.into_iter().enumerate() {
            if current_vertical_interval != height {
                // finalize last non-zero length vertical interval
                vertical_intervals[x].push(Interval(current_vertical_interval, height));
            }
        }
        // finalize last horizontal interval
        for (y, current_horizontal_interval) in current_horizontal_intervals.into_iter().enumerate()
        {
            if current_horizontal_interval != width {
                // finalize last non-zero length horizontal interval
                horizontal_intervals[y].push(Interval(current_horizontal_interval, width));
            }
        }
        Self {
            width,
            height,
            cycle: 0,
            rounded_rock_positions,
            vertical_intervals,
            horizontal_intervals,
        }
    }

    fn load_on_nort_support_beam(&self) -> usize {
        self.rounded_rock_positions
            .iter()
            .fold(0, |acc, pos| acc + self.height - pos.1)
    }

    fn do_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
        self.cycle += 1;
    }

    fn tilt_north(&mut self) {
        let mut adjusted_vertical_intervals = self.vertical_intervals.clone();
        'rock_loop: for rounded_rock_position in &mut self.rounded_rock_positions {
            let x = rounded_rock_position.0;
            let y = rounded_rock_position.1;
            for (interval, adjusted_interval) in self.vertical_intervals[x]
                .iter()
                .zip(adjusted_vertical_intervals[x].iter_mut())
            {
                if interval.0 <= y && y < interval.1 {
                    // rounded_rock_position is in interval
                    rounded_rock_position.1 = adjusted_interval.0;
                    adjusted_interval.0 += 1;
                    continue 'rock_loop;
                }
            }
            panic!(
                "the rock position {:02} / {:02} is not included in any vertical interval",
                rounded_rock_position.0, rounded_rock_position.1
            );
        }
    }

    fn tilt_west(&mut self) {
        let mut adjusted_horizontal_intervals = self.horizontal_intervals.clone();
        'rock_loop: for rounded_rock_position in &mut self.rounded_rock_positions {
            let x = rounded_rock_position.0;
            let y = rounded_rock_position.1;
            for (interval, adjusted_interval) in self.horizontal_intervals[y]
                .iter()
                .zip(adjusted_horizontal_intervals[y].iter_mut())
            {
                if interval.0 <= x && x < interval.1 {
                    // rounded_rock_position is in interval
                    rounded_rock_position.0 = adjusted_interval.0;
                    adjusted_interval.0 += 1;
                    continue 'rock_loop;
                }
            }
            panic!(
                "the rock position {:02} / {:02} is not included in any horizontal interval",
                rounded_rock_position.0, rounded_rock_position.1
            );
        }
    }

    fn tilt_south(&mut self) {
        let mut adjusted_vertical_intervals = self.vertical_intervals.clone();
        'rock_loop: for rounded_rock_position in &mut self.rounded_rock_positions {
            let x = rounded_rock_position.0;
            let y = rounded_rock_position.1;
            for (interval, adjusted_interval) in self.vertical_intervals[x]
                .iter()
                .zip(adjusted_vertical_intervals[x].iter_mut())
            {
                if interval.0 <= y && y < interval.1 {
                    // rounded_rock_position is in interval
                    rounded_rock_position.1 = adjusted_interval.1 - 1;
                    adjusted_interval.1 -= 1;
                    continue 'rock_loop;
                }
            }
            panic!(
                "the rock position {:02} / {:02} is not included in any vertical interval",
                rounded_rock_position.0, rounded_rock_position.1
            );
        }
    }

    fn tilt_east(&mut self) {
        let mut adjusted_horizontal_intervals = self.horizontal_intervals.clone();
        'rock_loop: for rounded_rock_position in &mut self.rounded_rock_positions {
            let x = rounded_rock_position.0;
            let y = rounded_rock_position.1;
            for (interval, adjusted_interval) in self.horizontal_intervals[y]
                .iter()
                .zip(adjusted_horizontal_intervals[y].iter_mut())
            {
                if interval.0 <= x && x < interval.1 {
                    // rounded_rock_position is in interval
                    rounded_rock_position.0 = adjusted_interval.1 - 1;
                    adjusted_interval.1 -= 1;
                    continue 'rock_loop;
                }
            }
            panic!(
                "the rock position {:02} / {:02} is not included in any horizontal interval",
                rounded_rock_position.0, rounded_rock_position.1
            );
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut rounded_rock_positions = String::new();
        for position in &self.rounded_rock_positions {
            rounded_rock_positions.push_str(&format!(
                "platform: 'O' pos: {:02} / {:02}\n",
                position.0, position.1
            ))
        }
        println!(
            "platform: rounded rock positions\n{}",
            rounded_rock_positions
        );

        let mut vertical_intervals = String::new();
        for (x, intervals) in self.vertical_intervals.iter().enumerate() {
            let joined_intervals = intervals
                .iter()
                .map(|i| format!("{}..{}", i.0, i.1))
                .collect::<Vec<String>>()
                .join(" ");
            vertical_intervals.push_str(&format!("x = {:02} ", x));
            vertical_intervals.push_str(&joined_intervals);
            vertical_intervals.push('\n');
        }
        println!("platform: vertical intervals\n{}", vertical_intervals);

        let mut horizontal_intervals = String::new();
        for (y, intervals) in self.horizontal_intervals.iter().enumerate() {
            let joined_intervals = intervals
                .iter()
                .map(|i| format!("{}..{}", i.0, i.1))
                .collect::<Vec<String>>()
                .join(" ");
            horizontal_intervals.push_str(&format!("y = {:02} ", y));
            horizontal_intervals.push_str(&joined_intervals);
            horizontal_intervals.push('\n');
        }
        println!("platform: horizontal intervals\n{}", horizontal_intervals);

        let mut map = vec![vec!['.'; self.width]; self.height];
        self.rounded_rock_positions
            .iter()
            .for_each(|r| map[r.1][r.0] = 'O');
        for line in map {
            let line = line.into_iter().collect::<String>();
            println!("platform: {}", line);
        }

        println!("\nplatform: cycle {}", self.cycle);
    }
}

pub fn get_total_load_on_north_support_beam_after_cycles(input: &str, cycles: usize) -> usize {
    let mut platform = Platform::new(input);

    // detect loop
    let mut state_to_cycle = HashMap::new();
    loop {
        let platform_ident = platform.state_hash();
        if let Some(repeated_cycle) = state_to_cycle.get(&platform_ident) {
            let loop_length = platform.cycle - repeated_cycle;
            platform.cycle = cycles - ((cycles - repeated_cycle) % loop_length);
            loop {
                if platform.cycle == cycles {
                    return platform.load_on_nort_support_beam();
                }
                platform.do_cycle();
            }
        } else {
            state_to_cycle.insert(platform_ident, platform.cycle);
        }
        platform.do_cycle();
    }
}
