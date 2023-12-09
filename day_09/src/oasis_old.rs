use std::collections::VecDeque;

struct DerivativeIterator<'i> {
    last_value: i64,
    iterator: Box<dyn Iterator<Item = i64> + 'i>,
    peeked_values: VecDeque<i64>,
}

impl<'i> DerivativeIterator<'i> {
    pub fn new(last_value: i64, iterator: Box<dyn Iterator<Item = i64> + 'i>) -> Self {
        Self {
            last_value,
            iterator,
            peeked_values: VecDeque::new(),
        }
    }
    pub fn is_all_zero(&mut self) -> bool {
        loop {
            if let Some(next_delta) = self.next_delta_from_iterator() {
                self.peeked_values.push_back(next_delta);
                if next_delta != 0 {
                    return false;
                }
            } else {
                return true;
            }
        }
    }

    fn next_delta_from_peeked_values(&mut self) -> Option<i64> {
        if self.peeked_values.len() != 0 {
            println!("next from peeked (len={})", self.peeked_values.len());
        }
        self.peeked_values.pop_front().map(|next_value| {
            println!("peeked is {}", next_value);
            let delta = self.last_value - next_value;
            self.last_value = next_value;
            delta
        })
    }
    fn next_delta_from_iterator(&mut self) -> Option<i64> {
        self.iterator.next().map(|next_value| {
            let delta = self.last_value - next_value;
            self.last_value = next_value;
            delta
        })
    }
}

impl Iterator for DerivativeIterator<'_> {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_delta_from_peeked_values().or_else(|| self.next_delta_from_iterator())
    }
}

pub fn sum_and_extrapolate_values_for_input<'i>(input: &'i str) -> i64 {
    let mut sum = 0;
    for history in input.lines() {
        let mut initial_sequence = history.rsplit(' ').map(|s| s.parse::<i64>().unwrap());      
        let initial_sequence_first_value = initial_sequence.next().unwrap();
        let mut sequence_start_values = vec![initial_sequence_first_value];
        let mut current_sequence = DerivativeIterator::new(initial_sequence_first_value, Box::new(initial_sequence));

        loop {
            let current_sequence_first_value = current_sequence.next().expect("failed to get first value");
            sequence_start_values.push(current_sequence_first_value);

            if current_sequence_first_value == 0 && current_sequence.is_all_zero() {
                #[cfg(not(debug_assertions))]
                {
                    sum += sequence_start_values.into_iter().sum::<i64>();
                }
                #[cfg(debug_assertions)]
                {
                    let interpolated_value = sequence_start_values.into_iter().sum::<i64>();
                    println!("interpolated_value={:012} history=\"{}\"", interpolated_value, history);
                    sum += interpolated_value;
                }

                
                break;
            }

            current_sequence = DerivativeIterator::new(current_sequence_first_value, Box::new(current_sequence));
        }
    }
    sum
}