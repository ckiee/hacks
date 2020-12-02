use regex::Regex;

const RAW_INPUT: &str = include_str!("../inputs/day2");
const REGEX: &str = r"(?m)^(\d+)-(\d+) ([[:alpha:]]): ([[:alpha:]]+)";

pub fn run() {
    part_1();
    part_2();
}

fn part_1() {
    let lines = RAW_INPUT.split('\n');
    let re = Regex::new(REGEX).unwrap();
    let mut match_count = 0;
    for line in lines {
        let cap = re.captures(line).unwrap();
        let min = cap.get(1).unwrap().as_str().parse().unwrap();
        let max = cap.get(2).unwrap().as_str().parse().unwrap();
        let letter = cap.get(3).unwrap().as_str().chars().nth(0).unwrap();
        let pw = cap.get(4).unwrap().as_str();
        let mut letter_count = 0;
        for x in pw.chars() {
            if x == letter {
                letter_count += 1;
            }
        }
        if letter_count >= min && letter_count <= max {
            match_count += 1;
        }
    }
    println!("Part 1: {}", match_count)
}

fn part_2() {
    let lines = RAW_INPUT.split('\n');
    let re = Regex::new(REGEX).unwrap();
    let mut match_count = 0;

    for line in lines {
        let cap = re.captures(line).unwrap();
        let expected_pos = cap.get(1).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let unexpected_pos = cap.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let letter = cap.get(3).unwrap().as_str().chars().nth(0).unwrap();
        let pw: Vec<char> = cap.get(4).unwrap().as_str().chars().collect();
        let mut letter_count = 0;
        for x in &[unexpected_pos, expected_pos] {
            if pw.get(*x).unwrap() == &letter {
                letter_count += 1;
            }
        }
        if letter_count == 1 {
            match_count += 1;
        }
    }
    println!("Part 2: {}", match_count)
}
