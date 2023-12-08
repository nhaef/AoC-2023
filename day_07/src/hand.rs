use core::panic;

pub type Bid = u32;
pub struct Hand<'s> {
    pub cards: &'s str,
    pub bid: Bid,
    pub ty: HandType,
}

impl Hand<'_> {
    pub fn cmp_1(&self, other: &Self) -> std::cmp::Ordering {
        match self.ty.cmp(&other.ty) {
            std::cmp::Ordering::Equal => {
                let self_chars = self.cards.chars();
                let other_chars = other.cards.chars();
                fn card_char_to_u8(card_char: char) -> u8 {
                    match card_char {
                        'A' => 14,
                        'K' => 13,
                        'Q' => 12,
                        'J' => 11,
                        'T' => 10,
                        '9' => 9,
                        '8' => 8,
                        '7' => 7,
                        '6' => 6,
                        '5' => 5,
                        '4' => 4,
                        '3' => 3,
                        '2' => 2,
                        _ => panic!("unexpected card_char {}", card_char),
                    }
                }
                for (self_char, other_char) in self_chars.zip(other_chars) {
                    match card_char_to_u8(self_char).cmp(&card_char_to_u8(other_char)) {
                        std::cmp::Ordering::Equal => continue,
                        ord => return ord,
                    }
                }
                std::cmp::Ordering::Equal
            }
            ord => ord,
        }
    }
    pub fn cmp_2(&self, other: &Self) -> std::cmp::Ordering {
        match self.ty.cmp(&other.ty) {
            std::cmp::Ordering::Equal => {
                let self_chars = self.cards.chars();
                let other_chars = other.cards.chars();
                fn card_char_to_u8(card_char: char) -> u8 {
                    match card_char {
                        'A' => 13,
                        'K' => 12,
                        'Q' => 11,
                        'T' => 10,
                        '9' => 9,
                        '8' => 8,
                        '7' => 7,
                        '6' => 6,
                        '5' => 5,
                        '4' => 4,
                        '3' => 3,
                        '2' => 2,
                        'J' => 1,
                        _ => panic!("unexpected card_char {}", card_char),
                    }
                }
                for (self_char, other_char) in self_chars.zip(other_chars) {
                    match card_char_to_u8(self_char).cmp(&card_char_to_u8(other_char)) {
                        std::cmp::Ordering::Equal => continue,
                        ord => return ord,
                    }
                }
                std::cmp::Ordering::Equal
            }
            ord => ord,
        }
    }
}

impl std::fmt::Display for Hand<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cards: {}  ty: {:<15}  bid: {}",
            self.cards, self.ty, self.bid
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl HandType {
    pub fn from_card_str_1(cards: &str) -> Self {
        let char_counts = cards
            .chars()
            .fold(std::collections::HashMap::new(), |mut acc, c| {
                if let Some(count) = acc.get_mut(&c) {
                    *count += 1;
                } else {
                    acc.insert(c, 1_u8);
                }
                acc
            })
            .into_values()
            .collect::<Vec<u8>>();

        let char_counts_len = char_counts.len();
        match char_counts_len {
            1 => Self::FiveOfAKind,
            2 => {
                if char_counts.contains(&3) {
                    Self::FullHouse
                } else {
                    Self::FourOfAKind
                }
            }
            3 => {
                if char_counts.contains(&3) {
                    Self::ThreeOfAKind
                } else {
                    Self::TwoPair
                }
            }
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => panic!("unexpected char_counts_len {}", char_counts_len),
        }
    }
    pub fn from_card_str_2(cards: &str) -> Self {
        let char_counts = cards
            .chars()
            .fold(std::collections::HashMap::new(), |mut acc, c| {
                if let Some(count) = acc.get_mut(&c) {
                    *count += 1;
                } else {
                    acc.insert(c, 1_u8);
                }
                acc
            });
        let jokers = match char_counts.get(&'J') {
            Some(v) => *v,
            None => 0,
        };
        let char_counts = char_counts.into_values().collect::<Vec<u8>>();

        let char_counts_len = char_counts.len();
        match char_counts_len {
            1 => Self::FiveOfAKind,
            2 => {
                if jokers != 0 {
                    Self::FiveOfAKind
                } else if char_counts.contains(&3) {
                    Self::FullHouse
                } else {
                    Self::FourOfAKind
                }
            }
            3 => {
                if jokers == 0 {
                    if char_counts.contains(&3) {
                        Self::ThreeOfAKind
                    } else {
                        Self::TwoPair
                    }
                } else if jokers == 1 {
                    if char_counts.contains(&3) {
                        Self::FourOfAKind
                    } else {
                        Self::FullHouse
                    }
                } else {
                    Self::FourOfAKind
                }
            }
            4 => {
                if jokers == 0 {
                    Self::OnePair
                } else {
                    Self::ThreeOfAKind
                }
            }
            5 => {
                if jokers == 0 {
                    Self::HighCard
                } else {
                    Self::OnePair
                }
            }
            _ => panic!("unexpected char_counts_len {}", char_counts_len),
        }
    }
}

impl std::fmt::Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandType::FiveOfAKind => "five of a kind".fmt(f),
            HandType::FourOfAKind => "four of a kind".fmt(f),
            HandType::FullHouse => "full house".fmt(f),
            HandType::ThreeOfAKind => "three of a kind".fmt(f),
            HandType::TwoPair => "two pair".fmt(f),
            HandType::OnePair => "one pair".fmt(f),
            HandType::HighCard => "high card".fmt(f),
        }
    }
}
