use crate::card::Card;

pub fn get_total_winning_number_points(input: &str) -> u32 {
    let cards: Vec<Card> = input.lines().map(|line| Card::from_str(line)).collect();
    cards
        .into_iter()
        .fold(0, |acc, card| acc + card.calculate_point_value())
}

pub fn get_total_scratchcards_with_copies(input: &str) -> u32 {
    let mut cards = vec![];
    let mut copies = vec![];
    for line in input.lines() {
        cards.push(Card::from_str(line));
        copies.push(1_u32);
    }

    cards.into_iter().enumerate().fold(0, |acc, (i, card)| {
        let this_card_copies = copies[i];
        let j_min = i + 1;
        let j_max = j_min + card.calculate_num_matches();
        for j in j_min..j_max {
            copies[j] += this_card_copies
        }
        acc + this_card_copies
    })
}
