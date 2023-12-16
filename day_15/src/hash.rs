pub fn get_hash_sum_of_init_sequence_steps(input: &str) -> usize {
    let line = input.lines().next().expect("failed to read first line");
    line.split(',').fold(0, |acc, step| acc + hash(step))
}

pub fn hash(value: &str) -> usize {
    let mut state = 0;
    for char in value.chars() {
        state += (char as u8) as usize;
        state *= 17;
        state %= 256;
    }
    state
}
