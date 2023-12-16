use std::thread::{self, JoinHandle};

#[derive(Debug, Clone)]
enum Tile {
    Empty {
        is_energized: bool,
    },
    MirrorLeftUp {
        is_energized_from_left_up: bool,
        is_energized_from_right_down: bool,
    },
    MirrorLeftDown {
        is_energized_from_left_down: bool,
        is_energized_from_right_up: bool,
    },
    SplitterUpDown {
        is_energized: bool,
    },
    SplitterLeftRight {
        is_energized: bool,
    },
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn parse_from_input(input: &str) -> Self {
        let mut tiles = vec![];
        for line in input.lines() {
            let mut row = vec![];
            for char in line.chars() {
                row.push(match char {
                    '.' => Tile::Empty {
                        is_energized: false,
                    },
                    '/' => Tile::MirrorLeftUp {
                        is_energized_from_left_up: false,
                        is_energized_from_right_down: false,
                    },
                    '\\' => Tile::MirrorLeftDown {
                        is_energized_from_left_down: false,
                        is_energized_from_right_up: false,
                    },
                    '|' => Tile::SplitterUpDown {
                        is_energized: false,
                    },
                    '-' => Tile::SplitterLeftRight {
                        is_energized: false,
                    },
                    _ => panic!("encountered unexpected char {}", char),
                });
            }
            tiles.push(row);
        }
        Self { tiles }
    }

    fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(y).and_then(|row| row.get_mut(x))
    }

    fn spawn_light(&mut self, light_source: (Direction, usize, usize)) {
        let mut light_sources = vec![light_source];
        loop {
            if let Some((direction, mut x, mut y)) = light_sources.pop() {
                // simulate light
                match direction {
                    Direction::Left => loop {
                        let current_tile = match self.get_tile_mut(x, y) {
                            None => break, // tile is out of bounds
                            Some(tile) => tile,
                        };
                        match current_tile {
                            Tile::Empty { is_energized }
                            | Tile::SplitterLeftRight { is_energized } => *is_energized = true,
                            Tile::MirrorLeftUp {
                                is_energized_from_right_down,
                                ..
                            } => {
                                if !*is_energized_from_right_down {
                                    // this mirror has never been energized from this angle
                                    *is_energized_from_right_down = true;
                                    light_sources.push((Direction::Down, x, y + 1));
                                }
                                break;
                            }
                            Tile::MirrorLeftDown {
                                is_energized_from_right_up,
                                ..
                            } => {
                                if !*is_energized_from_right_up {
                                    // this mirror has never been energized from this angle
                                    *is_energized_from_right_up = true;
                                    light_sources.push((Direction::Up, x, y.overflowing_sub(1).0));
                                }
                                break;
                            }
                            Tile::SplitterUpDown { is_energized } => {
                                if !*is_energized {
                                    // this splitter has never been energized before
                                    *is_energized = true;
                                    light_sources.push((Direction::Up, x, y.overflowing_sub(1).0));
                                    light_sources.push((Direction::Down, x, y + 1));
                                }
                                break;
                            }
                        }
                        x = x.overflowing_sub(1).0;
                    },
                    Direction::Right => loop {
                        let current_tile = match self.get_tile_mut(x, y) {
                            None => break, // tile is out of bounds
                            Some(tile) => tile,
                        };
                        match current_tile {
                            Tile::Empty { is_energized }
                            | Tile::SplitterLeftRight { is_energized } => *is_energized = true,
                            Tile::MirrorLeftUp {
                                is_energized_from_left_up,
                                ..
                            } => {
                                if !*is_energized_from_left_up {
                                    // this mirror has never been energized from this angle
                                    *is_energized_from_left_up = true;
                                    light_sources.push((Direction::Up, x, y.overflowing_sub(1).0));
                                }
                                break;
                            }
                            Tile::MirrorLeftDown {
                                is_energized_from_left_down,
                                ..
                            } => {
                                if !*is_energized_from_left_down {
                                    // this mirror has never been energized from this angle
                                    *is_energized_from_left_down = true;
                                    light_sources.push((Direction::Down, x, y + 1));
                                }
                                break;
                            }
                            Tile::SplitterUpDown { is_energized } => {
                                if !*is_energized {
                                    // this splitter has never been energized before
                                    *is_energized = true;
                                    light_sources.push((Direction::Up, x, y.overflowing_sub(1).0));
                                    light_sources.push((Direction::Down, x, y + 1));
                                }
                                break;
                            }
                        }
                        x += 1;
                    },
                    Direction::Up => loop {
                        let current_tile = match self.get_tile_mut(x, y) {
                            None => break, // tile is out of bounds
                            Some(tile) => tile,
                        };
                        match current_tile {
                            Tile::Empty { is_energized }
                            | Tile::SplitterUpDown { is_energized } => *is_energized = true,
                            Tile::MirrorLeftUp {
                                is_energized_from_right_down,
                                ..
                            } => {
                                if !*is_energized_from_right_down {
                                    // this mirror has never been energized from this angle
                                    *is_energized_from_right_down = true;
                                    light_sources.push((Direction::Right, x + 1, y));
                                }
                                break;
                            }
                            Tile::MirrorLeftDown {
                                is_energized_from_left_down,
                                ..
                            } => {
                                if !*is_energized_from_left_down {
                                    // this mirror has never been energized from this angle
                                    *is_energized_from_left_down = true;
                                    light_sources.push((
                                        Direction::Left,
                                        x.overflowing_sub(1).0,
                                        y,
                                    ));
                                }
                                break;
                            }
                            Tile::SplitterLeftRight { is_energized } => {
                                if !*is_energized {
                                    // this splitter has never been energized before
                                    *is_energized = true;
                                    light_sources.push((
                                        Direction::Left,
                                        x.overflowing_sub(1).0,
                                        y,
                                    ));
                                    light_sources.push((Direction::Right, x + 1, y));
                                }
                                break;
                            }
                        }
                        y = y.overflowing_sub(1).0;
                    },
                    Direction::Down => loop {
                        let current_tile = match self.get_tile_mut(x, y) {
                            None => break, // tile is out of bounds
                            Some(tile) => tile,
                        };
                        match current_tile {
                            Tile::Empty { is_energized }
                            | Tile::SplitterUpDown { is_energized } => *is_energized = true,
                            Tile::MirrorLeftUp {
                                is_energized_from_left_up,
                                ..
                            } => {
                                if !*is_energized_from_left_up {
                                    // this mirror has never been energized from this angle
                                    *is_energized_from_left_up = true;
                                    light_sources.push((
                                        Direction::Left,
                                        x.overflowing_sub(1).0,
                                        y,
                                    ));
                                }
                                break;
                            }
                            Tile::MirrorLeftDown {
                                is_energized_from_right_up,
                                ..
                            } => {
                                if !*is_energized_from_right_up {
                                    // this mirror has never been energized from this angle
                                    *is_energized_from_right_up = true;
                                    light_sources.push((Direction::Right, x + 1, y));
                                }
                                break;
                            }
                            Tile::SplitterLeftRight { is_energized } => {
                                if !*is_energized {
                                    // this splitter has never been energized before
                                    *is_energized = true;
                                    light_sources.push((
                                        Direction::Left,
                                        x.overflowing_sub(1).0,
                                        y,
                                    ));
                                    light_sources.push((Direction::Right, x + 1, y));
                                }
                                break;
                            }
                        }
                        y += 1;
                    },
                }
            } else {
                // no more light sources to simulate
                break;
            }
        }
    }

    fn get_total_energized_tiles(self) -> usize {
        self.tiles
            .into_iter()
            .flatten()
            .filter(|tile| match tile {
                Tile::Empty { is_energized } => *is_energized,
                Tile::MirrorLeftUp {
                    is_energized_from_left_up: is_energized_from_left,
                    is_energized_from_right_down: is_energized_from_right,
                } => *is_energized_from_left || *is_energized_from_right,
                Tile::MirrorLeftDown {
                    is_energized_from_left_down: is_energized_from_left,
                    is_energized_from_right_up: is_energized_from_right,
                } => *is_energized_from_left || *is_energized_from_right,
                Tile::SplitterUpDown { is_energized } => *is_energized,
                Tile::SplitterLeftRight { is_energized } => *is_energized,
            })
            .count()
    }
}

pub fn get_number_of_energized_tiles(input: &str) -> usize {
    let mut map = Map::parse_from_input(input);
    map.spawn_light((Direction::Right, 0, 0));
    map.get_total_energized_tiles()
}

pub fn get_max_number_of_energized_tiles(input: &str) -> usize {
    let map = Map::parse_from_input(input);
    let map_height = map.tiles.len();
    let map_width = map.tiles[0].len();

    let light_sources_down = (0..map_width)
        .map(|x| (Direction::Down, x, 0))
        .collect::<Vec<_>>();
    let light_sources_up = (0..map_width)
        .map(|x| (Direction::Up, x, map_height - 1))
        .collect::<Vec<_>>();
    let light_sources_right = (0..map_height)
        .map(|y| (Direction::Right, 0, y))
        .collect::<Vec<_>>();
    let light_sources_left = (0..map_height)
        .map(|y| (Direction::Left, map_width - 1, y))
        .collect::<Vec<_>>();

    let light_sources_down_join_handle =
        get_max_number_of_energized_tiles_for_light_sources(map.clone(), light_sources_down);
    let light_sources_up_join_handle =
        get_max_number_of_energized_tiles_for_light_sources(map.clone(), light_sources_up);
    let light_sources_right_join_handle =
        get_max_number_of_energized_tiles_for_light_sources(map.clone(), light_sources_right);
    let light_sources_left_join_handle =
        get_max_number_of_energized_tiles_for_light_sources(map.clone(), light_sources_left);

    usize::max(
        light_sources_down_join_handle
            .join()
            .expect("failed to join thread handle"),
        usize::max(
            light_sources_up_join_handle
                .join()
                .expect("failed to join thread handle"),
            usize::max(
                light_sources_right_join_handle
                    .join()
                    .expect("failed to join thread handle"),
                light_sources_left_join_handle
                    .join()
                    .expect("failed to join thread handle"),
            ),
        ),
    )

    // let max_energized_tiles_down = thread::spawn(move || {
    //     let map = map.clone();
    //     let mut max_energized_tiles = 0;
    //     for x in 0..map_width {
    //         let mut map = map.clone();
    //         map.spawn_light((Direction::Down, x, 0));
    //         let energized_tiles = map.get_total_energized_tiles();
    //         max_energized_tiles = usize::max(max_energized_tiles, energized_tiles);
    //     }
    // });

    // let max_energized_tiles_up = thread::spawn(move || {
    //     let map = map.clone();
    //     let mut max_energized_tiles = 0;
    //     for x in 0..map_width {
    //         let mut map = map.clone();
    //         map.spawn_light((Direction::Up, x, map_height - 1));
    //         let energized_tiles = map.get_total_energized_tiles();
    //         max_energized_tiles = usize::max(max_energized_tiles, energized_tiles);
    //     }
    // });

    // for y in 0..map_height {
    //     let mut map = map.clone();
    //     map.spawn_light((Direction::Right, 0, y));
    //     let energized_tiles = map.get_total_energized_tiles();
    //     max_energized_tiles = usize::max(max_energized_tiles, energized_tiles);;
    // }

    // for y in 0..map_height {
    //     let mut map = map.clone();
    //     map.spawn_light((Direction::Left, map_width - 1, y));
    //     let energized_tiles = map.get_total_energized_tiles();
    //     max_energized_tiles = usize::max(max_energized_tiles, energized_tiles);
    // }
}

fn get_max_number_of_energized_tiles_for_light_sources(
    map: Map,
    light_sources: Vec<(Direction, usize, usize)>,
) -> JoinHandle<usize> {
    thread::spawn(move || {
        let mut max_energized_tiles = 0;
        for light_source in light_sources {
            let mut map = map.clone();
            map.spawn_light(light_source);
            max_energized_tiles = usize::max(max_energized_tiles, map.get_total_energized_tiles());
        }
        max_energized_tiles
    })
}
