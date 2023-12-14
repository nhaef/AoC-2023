use crate::maze::*;

struct MapWalker<'m> {
    map: &'m Vec<Vec<u8>>,
    position: MapPoint,
    direction: Direction,
    steps: u32,
}

impl<'m> MapWalker<'m> {
    pub fn new(map: &'m Vec<Vec<u8>>, position: MapPoint, direction: Direction) -> Self {
        Self {
            map,
            position,
            direction,
            steps: 0,
        }
    }
}

impl MapWalker<'_> {
    fn walk(&mut self) {
        self.steps += 1;
        match self.direction {
            Direction::NORTH => {
                self.position.1 -= 1;
                match read_map_tile(self.map, &self.position).unwrap() {
                    &PIPE_SOUTH_WEST => self.direction = Direction::WEST,
                    &PIPE_NORTH_SOUTH => self.direction = Direction::NORTH,
                    &PIPE_SOUTH_EAST => self.direction = Direction::EAST,
                    &PIPE_START => (),
                    t => panic!("unexpected tile {}", t),
                }
            }
            Direction::SOUTH => {
                self.position.1 += 1;
                match read_map_tile(self.map, &self.position).unwrap() {
                    &PIPE_NORTH_WEST => self.direction = Direction::WEST,
                    &PIPE_NORTH_SOUTH => self.direction = Direction::SOUTH,
                    &PIPE_NORTH_EAST => self.direction = Direction::EAST,
                    &PIPE_START => (),
                    t => panic!("unexpected tile {}", t),
                }
            }
            Direction::WEST => {
                self.position.0 -= 1;
                match read_map_tile(self.map, &self.position).unwrap() {
                    &PIPE_SOUTH_EAST => self.direction = Direction::SOUTH,
                    &PIPE_WEST_EAST => self.direction = Direction::WEST,
                    &PIPE_NORTH_EAST => self.direction = Direction::NORTH,
                    &PIPE_START => (),
                    t => panic!("unexpected tile {}", t),
                }
            }
            Direction::EAST => {
                self.position.0 += 1;
                match read_map_tile(self.map, &self.position).unwrap() {
                    &PIPE_SOUTH_WEST => self.direction = Direction::SOUTH,
                    &PIPE_WEST_EAST => self.direction = Direction::EAST,
                    &PIPE_NORTH_WEST => self.direction = Direction::NORTH,
                    &PIPE_START => (),
                    t => panic!("unexpected tile {}", t),
                }
            }
        }
    }
}

pub fn get_steps_to_farthest_point(input: &str) -> u32 {
    let (start_point, map) = read_start_point_and_map(input);

    let mut direction = None;
    // check north
    if let Some(&PIPE_SOUTH_WEST | &PIPE_NORTH_SOUTH | &PIPE_SOUTH_EAST) =
        read_map_tile(&map, &MapPoint(start_point.0, start_point.1 - 1))
    {
        direction = Some(Direction::NORTH);
    }
    // check south
    else if let Some(&PIPE_NORTH_WEST | &PIPE_NORTH_SOUTH | &PIPE_NORTH_EAST) =
        read_map_tile(&map, &MapPoint(start_point.0, start_point.1 + 1))
    {
        direction = Some(Direction::SOUTH);
    }
    // check west
    else if let Some(&PIPE_SOUTH_EAST | &PIPE_WEST_EAST | &PIPE_NORTH_EAST) =
        read_map_tile(&map, &MapPoint(start_point.0 - 1, start_point.1))
    {
        direction = Some(Direction::WEST);
    }

    let direction = direction.expect("could not find start direction");
    let mut walker = MapWalker::new(&map, start_point.clone(), direction);

    loop {
        walker.walk();

        if walker.position == start_point {
            return walker.steps / 2;
        }
    }
}
