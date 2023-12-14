use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Instruction {
    Left,
    Right,
}

impl Instruction {
    pub fn get<'v, V>(&self, (left, right): &'v (V, V)) -> &'v V {
        match self {
            Self::Left => left,
            Self::Right => right,
        }
    }
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            v => panic!("unknown instruction {}", v),
        }
    }
}

struct InstructionStateMachine<'ism, S> {
    state: S,
    step: usize,
    instruction_cursor: usize,
    instructions: &'ism [Instruction],
    transition_rules: &'ism HashMap<S, (S, S)>,
}

impl<'ism, S> InstructionStateMachine<'ism, S>
where
    S: std::hash::Hash + Eq + Clone,
{
    pub fn new(
        initial_state: S,
        instructions: &'ism [Instruction],
        transition_rules: &'ism HashMap<S, (S, S)>,
    ) -> Self {
        Self {
            state: initial_state,
            step: 0,
            instruction_cursor: 0,
            instructions,
            transition_rules,
        }
    }

    pub fn step(&mut self) -> usize {
        let instruction = &self.instructions[self.instruction_cursor];
        self.instruction_cursor = (self.instruction_cursor + 1) % self.instructions.len();
        self.state = instruction
            .get(self.transition_rules.get(&self.state).unwrap())
            .clone();
        self.step += 1;
        self.step
    }
}

pub fn find_aaa_to_zzz_steps<'i>(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .expect(&format!("Failed to read first line of input {}", input))
        .chars()
        .map(|c| c.into())
        .collect::<Vec<Instruction>>();
    lines
        .next()
        .expect(&format!("Failed to skip second line of input {}", input));

    type State = [u8; 3];
    let mut transition_rules = HashMap::<[u8; 3], ([u8; 3], [u8; 3])>::new();
    for line in lines {
        let label = line[..3].as_bytes();
        let label_left = line[7..10].as_bytes();
        let label_right = line[12..15].as_bytes();

        let label = TryInto::<State>::try_into(label).unwrap();
        let label_left = TryInto::<State>::try_into(label_left).unwrap();
        let label_right = TryInto::<State>::try_into(label_right).unwrap();

        transition_rules.insert(label, (label_left, label_right));
    }

    let initial_state = TryInto::<State>::try_into("AAA".as_bytes()).unwrap();
    let final_state = TryInto::<State>::try_into("ZZZ".as_bytes()).unwrap();
    let mut state_machine =
        InstructionStateMachine::new(initial_state, &instructions, &transition_rules);

    while *&state_machine.state != final_state {
        state_machine.step();
    }

    state_machine.step
}

pub fn find_a_to_z_steps<'i>(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .expect(&format!("Failed to read first line of input {}", input))
        .chars()
        .map(|c| c.into())
        .collect::<Vec<Instruction>>();
    lines
        .next()
        .expect(&format!("Failed to skip second line of input {}", input));

    type State = [u8; 3];
    let mut transition_rules = HashMap::<State, (State, State)>::new();
    for line in lines {
        let label = line[..3].as_bytes();
        let label_left = line[7..10].as_bytes();
        let label_right = line[12..15].as_bytes();

        let label = TryInto::<State>::try_into(label).unwrap();
        let label_left = TryInto::<State>::try_into(label_left).unwrap();
        let label_right = TryInto::<State>::try_into(label_right).unwrap();

        transition_rules.insert(label, (label_left, label_right));
    }

    // This solution does a few assumptions about the puzzle data,
    // which haven't been described in the AoC puzzle.
    // - each state machine only sees a single unique final state
    // - the first occurence of the final state is always at loop_end - loop_start
    //
    // Because these assumptions seem to be true, a simplified equation can be used:
    // steps = x1 * (loop_end_1 - loop_start_1)

    let initial_state_byte = 'A' as u8;
    let final_state_interval = transition_rules
        .keys()
        .filter(|l| l[2] == initial_state_byte)
        .map(|l| {
            let mut state_machine =
                InstructionStateMachine::new(l.clone(), &instructions, &transition_rules);
            // detect loop in state machine
            let mut visited_internal_states = HashMap::<(usize, State), usize>::new();
            loop {
                let current_interal_state = (state_machine.instruction_cursor, state_machine.state);
                match visited_internal_states.get(&current_interal_state) {
                    None => {
                        visited_internal_states.insert(current_interal_state, state_machine.step);
                    }
                    Some(v) => return state_machine.step - *v,
                }
                state_machine.step();
            }
        })
        .collect::<Vec<_>>();

    least_common_multiple(final_state_interval)
}

fn least_common_multiple(values: Vec<usize>) -> usize {
    values
        .clone()
        .into_iter()
        .reduce(|acc, x| acc * (x / greatest_common_divisor(acc, x)))
        .unwrap()
}

fn greatest_common_divisor(mut a: usize, mut b: usize) -> usize {
    loop {
        if a > b {
            a -= b
        } else if b > a {
            b -= a
        } else {
            return a;
        }
    }
}
