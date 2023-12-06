use std::{collections::HashSet, hash::RandomState};

#[derive(Debug)]
pub struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

impl Card {
    pub fn from_str(card: &str) -> Self {
        let (card_head, card_content) = card
            .split_once(':')
            .expect(&format!("Could not find delimiter ':' in card {}", card));
        let (_, id) = card_head.split_once(' ').expect(&format!(
            "Could not find delimiter ' ' in card_head {}",
            card_head
        ));
        let id = id
            .trim_start()
            .parse::<u32>()
            .expect(&format!("Could not parse id {}", id));
        let (winning_numbers, your_numbers) = card_content.split_once('|').expect(&format!(
            "Could not find delimiter '|' in card_content {}",
            card_content
        ));
        let winning_numbers = winning_numbers
            .trim()
            .split(' ')
            .filter(|w| !w.is_empty())
            .map(|w| w.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .expect(&format!(
                "Could not parse winning_numbers {}",
                winning_numbers
            ));
        let your_numbers = your_numbers
            .trim()
            .split(' ')
            .filter(|w| !w.is_empty())
            .map(|w| w.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .expect(&format!("Could not parse your_numbers {}", your_numbers));

        Self {
            id,
            winning_numbers,
            your_numbers,
        }
    }

    pub fn calculate_num_matches(self) -> usize {
        let winning_numbers: HashSet<u32, RandomState> = HashSet::from_iter(self.winning_numbers);
        self.your_numbers
            .iter()
            .filter(|your_number| winning_numbers.contains(&your_number))
            .count()
    }

    pub fn calculate_point_value(self) -> u32 {
        match usize::checked_sub(self.calculate_num_matches(), 1) {
            None => 0,
            Some(p) => u32::pow(2, p as u32),
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Card {}: {} | {}",
            self.id,
            self.winning_numbers
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" "),
            self.your_numbers
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        )
    }
}
