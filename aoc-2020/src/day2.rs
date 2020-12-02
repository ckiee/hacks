use regex::Regex;

const RAW_INPUT: &str = include_str!("../inputs/day2");

pub fn run() {
    part_1();
    part_2();
}

fn part_1() {
    let lines = RAW_INPUT.split('\n');
    let re = Regex::new(r"(?m)^(\d+)-(\d+) ([[:alpha:]]): ([[:alpha:]]+)").unwrap();
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

fn part_2() {}
