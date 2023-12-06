use crate::{cube_set::CubeSet, game::Game};

pub fn solve_game_list_1(input: &str) -> u32 {
    let mut id_sum = 0;
    let elf_bag = CubeSet::new(12, 13, 14);
    for line in input.lines() {
        let game = Game::from_str(line);
        // println!("debug game {:#?}", game);
        if game.is_possible_with_bag(&elf_bag) {
            id_sum += game.id
        }
    }
    id_sum
}

pub fn solve_game_list_2(input: &str) -> u32 {
    let mut power_sum = 0;
    for line in input.lines() {
        let game = Game::from_str(line);
        power_sum += game.power_of_smallest_cube_set();
    }
    power_sum
}
