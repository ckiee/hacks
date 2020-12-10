const RAW_INPUT: &str = include_str!("../inputs/day10");

pub fn run() {
    let data = parse();
    part_1(&data);
    part_2(&data);
}

fn parse() -> Vec<i64> {
    let mut data = RAW_INPUT
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();
    data.sort();
    data
}
fn part_1(data: &Vec<i64>) {
    // let laptop_in = data.iter().max().unwrap() + 3;
    let mut one_diff: i64 = 0;
    let mut three_diff: i64 = 1;
    for (i, x) in data.iter().enumerate() {
        let last = if i == 0 { &0 } else { data.get(i - 1).unwrap() };
        println!("{} ({})", x, x - last);
        match x - last {
            1 => {
                one_diff += 1;
            }
            3 => {
                three_diff += 1;
            }
            _ => {}
        };
    }
    println!("Part 1: {}", three_diff * one_diff);
}

fn part_2(data: &Vec<i64>) {
    let mut current_combo = 0;
    let mut last = 0;
    let mut last_diff = 0;
    let mut two_combos = 0;
    let mut three_combos = 0;
    for x in data {
        let diff = x - last;
        if diff == last_diff && diff == 1 {
            current_combo += 1;
        // println!("x={},last={},{},{},combo={}", x,last,diff, last_diff, current_combo);
        } else if current_combo > 0 {
            if current_combo == 2 {
                two_combos += 1;
            } else if current_combo == 3 {
                three_combos += 1;
            } else {
				println!("wtf: {}", current_combo);
			}
        }
        last = *x;
        last_diff = diff;
    }
    println!("Part 2: {}", (2 as u64).pow(two_combos) + (3 as u64).pow(three_combos));
}
