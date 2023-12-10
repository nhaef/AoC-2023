pub const PIPE_START: u8 = b'S';
pub const PIPE_WEST_EAST: u8 = b'-';
pub const PIPE_NORTH_SOUTH: u8 = b'|';
pub const PIPE_NORTH_EAST: u8 = b'L';
pub const PIPE_NORTH_WEST: u8 = b'J';
pub const PIPE_SOUTH_EAST: u8 = b'F';
pub const PIPE_SOUTH_WEST: u8 = b'7';

#[derive(Debug, Clone, PartialEq)]
pub struct MapPoint(pub usize, pub usize);

#[derive(Debug, Clone)]
pub enum Direction {
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

pub fn read_map_tile<'m>(map: &'m Vec<Vec<u8>>, pos: &MapPoint) -> Option<&'m u8> {
    map.get(pos.1).and_then(|line| line.get(pos.0))
}

pub fn read_map_tile_mut<'m>(map: &'m mut Vec<Vec<u8>>, pos: &MapPoint) -> Option<&'m mut u8> {
    map.get_mut(pos.1).and_then(|line| line.get_mut(pos.0))
}

pub fn read_start_point_and_map(input: &str) -> (MapPoint, Vec<Vec<u8>>) {
    let mut map = vec![];
    let mut start_point = None;
    for line in input.lines() {
        let mut map_line = vec![];
        for char in line.chars().map(|c| c as u8) {
            if start_point.is_none() && char == PIPE_START {
                start_point = Some(MapPoint(map_line.len(), map.len()));
            }
            map_line.push(char);
        }
        map.push(map_line);
    }
    let start_point = start_point.expect("could not find start_point");
    (start_point, map)
}
