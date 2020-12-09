const RAW_INPUT: &str = include_str!("../inputs/day9");

pub fn run() {
    let data = parse();
    let p1 = part_1(&data).expect("no solution for part 1");
    let p2 = part_2(&data, p1).expect("no solution for part 2");
    println!("Part 1,2: {},{}", p1, p2);
}

fn parse() -> Vec<u64> {
    RAW_INPUT
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>()
}
const PRE: usize = 25;
fn part_1(data: &Vec<u64>) -> Option<&u64> {
    for (i, x) in data.iter().enumerate().skip(PRE) {
        let before = &data[i - PRE..i];
        let mut found = false;
        for y in before {
            for z in before {
                if y + z == *x {
                    found = true;
                }
            }
        }
        if !found {
            return Some(x);
        }
    }
    None
}

fn part_2(data: &Vec<u64>, p1: &u64) -> Option<u64> {
    for (i, _x) in data.iter().enumerate() {
        for (j, _y) in data.iter().enumerate() {
            // println!("({},{}) {}-{}", i, j, i, i + j);
            if i + j >= data.len() {
                break;
            }
            let partial = &data[i..i + j];
            let sum = partial.iter().sum::<u64>();
            if sum > *p1 {
                break;
            }
            if partial.len() != 1 && sum == *p1 {
                let mut sorted = partial.to_vec();
                sorted.sort();
                return Some(sorted.first().unwrap() + sorted.last().unwrap());
            }
        }
    }
    None
}
