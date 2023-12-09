pub struct HistorySequence {
    pub is_all_zero: bool,
    values: Vec<i64>,
}

impl HistorySequence {
    pub fn get_first_value(&self) -> i64 {
        self.values[0]
    }
    pub fn derive(self) -> Self {
        let value_iterator = self.values.into_iter();
        value_iterator.clone().zip(value_iterator.skip(1)).map(|(next, prev)| next - prev).collect()
    }
}

impl FromIterator<i64> for HistorySequence {
    fn from_iter<T: IntoIterator<Item = i64>>(iter: T) -> Self {
        let mut is_all_zero = true; 
        let values = iter.into_iter().inspect(|val| is_all_zero = is_all_zero && (*val == 0)).collect();
        Self {
            is_all_zero,
            values,
        }
    }
}

pub fn sum_and_extrapolate_next_values_for_input(input: &str) -> i64 {
    let mut sum = 0;
    for history in input.lines() {
        let mut history_sequence = history.rsplit(' ').map(|value: &str| value.parse::<i64>().unwrap()).collect::<HistorySequence>();
        let mut history_sequence_first_values = vec![];
        loop {
            history_sequence_first_values.push(history_sequence.get_first_value());
            if history_sequence.is_all_zero {
                sum += history_sequence_first_values.into_iter().sum::<i64>();
                break;
            }
            history_sequence = history_sequence.derive();
        }
    }
    sum
}

pub fn sum_and_extrapolate_prev_values_for_input(input: &str) -> i64 {
    let mut sum = 0;
    for history in input.lines() {
        let mut history_sequence = history.split(' ').map(|value: &str| value.parse::<i64>().unwrap()).collect::<HistorySequence>();
        let mut history_sequence_first_values = vec![];
        loop {
            history_sequence_first_values.push(history_sequence.get_first_value());
            if history_sequence.is_all_zero {
                sum += history_sequence_first_values.into_iter().sum::<i64>();
                break;
            }
            history_sequence = history_sequence.derive();
        }
    }
    sum
}