use crate::hash::hash;

pub fn get_focussing_power(input: &str) -> usize {
    let line = input.lines().next().expect("failed to read first line");
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
    'steps: for step in line.split(',') {
        if let Some((label, _)) = step.split_once('-') {
            let hash = hash(label);
            let current_box = &mut boxes[hash];
            for i in 0..current_box.len() {
                if current_box[i].0 == label {
                    current_box.remove(i);
                    continue 'steps;
                }
            }
        }
        if let Some((label, focal_length)) = step.split_once('=') {
            let hash = hash(label);
            let focal_length = focal_length
                .parse::<usize>()
                .expect("failed to parse focal length");
            let current_box = &mut boxes[hash];
            for i in 0..current_box.len() {
                if current_box[i].0 == label {
                    current_box[i].1 = focal_length;
                    continue 'steps;
                }
            }
            current_box.push((label, focal_length));
        }
    }
    boxes
        .into_iter()
        .enumerate()
        .map(|(box_number, box_content)| {
            (box_number + 1)
                * box_content
                    .into_iter()
                    .enumerate()
                    .fold(0, |acc, (box_slot, (_, focal_length))| {
                        acc + ((box_slot + 1) * focal_length)
                    })
        })
        .sum()
}
