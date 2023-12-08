use crate::hand::{Hand, Bid, HandType};

pub fn input_to_hands_1<'s>(input: &'s str) -> Vec<Hand<'s>> {
    input.lines().map(|line| {
        let (cards, bid) = line.split_once(' ').expect(&format!("Could not find delimiter ' ' in line {}", line));
        let bid = bid.parse::<Bid>().expect(&format!("Failed to parse bid {}", bid));
        let ty = HandType::from_card_str_1(cards);
        Hand {
            cards,
            bid,
            ty
        }
    })
    .collect()
}

pub fn input_to_hands_2<'s>(input: &'s str) -> Vec<Hand<'s>> {
    input.lines().map(|line| {
        let (cards, bid) = line.split_once(' ').expect(&format!("Could not find delimiter ' ' in line {}", line));
        let bid = bid.parse::<Bid>().expect(&format!("Failed to parse bid {}", bid));
        let ty = HandType::from_card_str_2(cards);
        Hand {
            cards,
            bid,
            ty
        }
    })
    .collect()
}

pub fn get_total_winnings_1(mut hands: Vec<Hand>) -> u32 {
    // sort
    for pass in (0..hands.len()).rev() {
        for i in 0..pass {
            let i_next = i + 1;
            if hands[i].cmp_1(&hands[i_next]) == std::cmp::Ordering::Greater {
                hands.swap(i, i_next);
            }
        }
    }
    // calculate winnings
    hands.into_iter().enumerate().fold(0, |acc, (i, hand)| acc + (i as u32 + 1) * hand.bid)
}

pub fn get_total_winnings_2(mut hands: Vec<Hand>) -> u32 {
    // sort
    for pass in (0..hands.len()).rev() {
        for i in 0..pass {
            let i_next = i + 1;
            if hands[i].cmp_2(&hands[i_next]) == std::cmp::Ordering::Greater {
                hands.swap(i, i_next);
            }
        }
    }
    // calculate winnings
    hands.into_iter().enumerate().fold(0, |acc, (i, hand)| acc + (i as u32 + 1) * hand.bid)
}