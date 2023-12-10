use crate::maze::*;

const WALL: u8 = b'X';
const WATER: u8 = b'w';

struct MapWalker<'m> {
    map: &'m mut Vec<Vec<u8>>,
    position: MapPoint,
    direction: Direction,
    steps: u32,
    cw_turns: i32,
    cw_water_spawns: Vec<MapPoint>,
    ccw_water_spawns: Vec<MapPoint>,
}

impl<'m> MapWalker<'m> {
    pub fn new(map: &'m mut Vec<Vec<u8>>, position: MapPoint, direction: Direction) -> Self {
        Self {
            map,
            position,
            direction,
            steps: 0,
            cw_turns: 0,
            cw_water_spawns: vec![],
            ccw_water_spawns: vec![],
        }
    }
}

impl MapWalker<'_> {
    fn walk(&mut self) {
        self.steps += 1;
        let current_tile_ref = match self.direction {
            Direction::NORTH => {
                self.cw_water_spawns.push(MapPoint(self.position.0 + 1, self.position.1));
                self.ccw_water_spawns.push(MapPoint(self.position.0 - 1, self.position.1));
                self.position.1 -= 1;
                let current_tile_ref = read_map_tile_mut(self.map, &self.position).unwrap();
                match *current_tile_ref {
                    PIPE_SOUTH_WEST => {
                        self.cw_water_spawns.push(MapPoint(self.position.0 + 1, self.position.1));
                        self.direction = Direction::WEST;
                        self.cw_turns -= 1;
                    },
                    PIPE_NORTH_SOUTH => self.direction = Direction::NORTH,
                    PIPE_SOUTH_EAST => {
                        self.ccw_water_spawns.push(MapPoint(self.position.0 - 1, self.position.1));
                        self.direction = Direction::EAST;
                        self.cw_turns += 1;
                    },
                    PIPE_START => (),
                    t => panic!("unexpected tile {}", t),
                }
                current_tile_ref
            },
            Direction::SOUTH => {
                self.cw_water_spawns.push(MapPoint(self.position.0 - 1, self.position.1));
                self.ccw_water_spawns.push(MapPoint(self.position.0 + 1, self.position.1));
                self.position.1 += 1;
                let current_tile_ref = read_map_tile_mut(self.map, &self.position).unwrap();
                match *current_tile_ref {
                    PIPE_NORTH_WEST => {
                        self.cw_water_spawns.push(MapPoint(self.position.0 - 1, self.position.1));
                        self.direction = Direction::WEST;
                        self.cw_turns += 1;
                    },
                    PIPE_NORTH_SOUTH => self.direction = Direction::SOUTH,
                    PIPE_NORTH_EAST => {
                        self.ccw_water_spawns.push(MapPoint(self.position.0 + 1, self.position.1));
                        self.direction = Direction::EAST;
                        self.cw_turns -= 1;
                    },
                    PIPE_START => (),
                    t => panic!("unexpected tile {}", t),
                }
                current_tile_ref
            },
            Direction::WEST => {
                self.cw_water_spawns.push(MapPoint(self.position.0, self.position.1 - 1));
                self.ccw_water_spawns.push(MapPoint(self.position.0, self.position.1 + 1));
                self.position.0 -= 1;
                let current_tile_ref = read_map_tile_mut(self.map, &self.position).unwrap();
                match *current_tile_ref {
                    PIPE_SOUTH_EAST => {
                        self.cw_water_spawns.push(MapPoint(self.position.0, self.position.1 - 1));
                        self.direction = Direction::SOUTH;
                        self.cw_turns -= 1;
                    },
                    PIPE_WEST_EAST => self.direction = Direction::WEST,
                    PIPE_NORTH_EAST => {
                        self.ccw_water_spawns.push(MapPoint(self.position.0, self.position.1 + 1));
                        self.direction = Direction::NORTH;
                        self.cw_turns += 1;
                    },
                    PIPE_START => (),
                    t => panic!("unexpected tile {}", t),
                }
                current_tile_ref
            },
            Direction::EAST => {
                self.cw_water_spawns.push(MapPoint(self.position.0, self.position.1 + 1));
                self.ccw_water_spawns.push(MapPoint(self.position.0, self.position.1 - 1));
                self.position.0 += 1;
                let current_tile_ref = read_map_tile_mut(self.map, &self.position).unwrap();
                match *current_tile_ref {
                    PIPE_SOUTH_WEST => {
                        self.cw_water_spawns.push(MapPoint(self.position.0, self.position.1 + 1));
                        self.direction = Direction::SOUTH;
                        self.cw_turns += 1;
                    },
                    PIPE_WEST_EAST => self.direction = Direction::EAST,
                    PIPE_NORTH_WEST => {
                        self.ccw_water_spawns.push(MapPoint(self.position.0, self.position.1 - 1));
                        self.direction = Direction::NORTH;
                        self.cw_turns -= 1;
                    },
                    PIPE_START => (),
                    t => panic!("unexpected tile {}", t),
                }
                current_tile_ref
            },
        };
        *current_tile_ref = WALL;
    }
}

pub fn get_enclosed_tiles(input: &str) -> u32 {
    let (start_point, mut map) = read_start_point_and_map(input);
    
    let mut direction = None;
    // check north
    if let Some(&PIPE_SOUTH_WEST | &PIPE_NORTH_SOUTH | &PIPE_SOUTH_EAST) = read_map_tile(&map, &MapPoint(start_point.0, start_point.1 - 1)) {
        direction = Some(Direction::NORTH);
    }
    // check south
    else if let Some(&PIPE_NORTH_WEST | &PIPE_NORTH_SOUTH | &PIPE_NORTH_EAST) = read_map_tile(&map, &MapPoint(start_point.0, start_point.1 + 1)) {
        direction = Some(Direction::SOUTH);
    }
    // check west
    else if let Some(&PIPE_SOUTH_EAST | &PIPE_WEST_EAST | &PIPE_NORTH_EAST) = read_map_tile(&map, &MapPoint(start_point.0 - 1, start_point.1)) {
        direction = Some(Direction::WEST);
    }

    let direction = direction.expect("could not find start direction");
    let mut walker: MapWalker<'_> = MapWalker::new(&mut map, start_point.clone(), direction);

    loop {
        walker.walk();

        if walker.position == start_point {
            let water_spawns = if walker.cw_turns > 0 { walker.cw_water_spawns } else { walker.ccw_water_spawns };
            let mut flooded_tiles: u32 = 0;
            // print_map(&map);
            for water_spawn in water_spawns {
                flood(&mut map, &water_spawn, &mut flooded_tiles);
            }
            // print_map(&map);
            return flooded_tiles;
        }
    }
}

// recursive flood
fn flood(map: &mut Vec<Vec<u8>>, pos: &MapPoint, flooded_tiles: &mut u32) {
    if let Some(tile) = read_map_tile_mut(map, pos) {
        if *tile != WALL && *tile != WATER {
            *tile = WATER;
            *flooded_tiles += 1;
            flood(map, &MapPoint(pos.0 - 1, pos.1), flooded_tiles);
            flood(map, &MapPoint(pos.0 + 1, pos.1), flooded_tiles);
            flood(map, &MapPoint(pos.0, pos.1 - 1), flooded_tiles);
            flood(map, &MapPoint(pos.0, pos.1 + 1), flooded_tiles);
        }
    }
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<u8>>) {
    for line in map {
        let mut line_str = String::new();
        for tile in line {
            line_str.push(char::from(*tile));
        }
        println!("map: {}", line_str);
    }
}
