use std::collections::HashMap;

const RAW_INPUT: &str = include_str!("../inputs/day6");

pub fn run() {
    let data = RAW_INPUT.split("\n\n");
    let mut pt1_score = 0;
    let mut pt2_score = 0;

    for group in data {
        let lines = group.split("\n");
        let line_count = lines.clone().count();
        let chars = lines.flat_map(|x| x.chars());
        let mut state = HashMap::with_capacity(24);

        for x in chars {
            if state.contains_key(&x) {
                state.insert(x, state.get(&x).unwrap() + 1);
            } else {
                state.insert(x, 1);
            }
        }

        pt1_score += state.len();
        pt2_score += state.iter().filter(|(_, i)| **i == line_count).count();
    }

    println!("Part 1: {}", pt1_score);
    println!("Part 2: {}", pt2_score);
}
