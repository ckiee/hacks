const RAW_INPUT: &str = include_str!("../inputs/day1");

pub fn run() {
    part_1();
    part_2();
}

fn part_1() {
    let input: Vec<i32> = RAW_INPUT
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap()).collect();
	
    for x in &input {
        for y in &input {
            if x + y == 2020 {
                println!("Day 1.1 Answer: {}", x * y);
            }
        }
    }
}

fn part_2() {
    let input: Vec<i32> = RAW_INPUT
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap()).collect();
	
    for x in &input {
        for y in &input {
            for z in &input {
                if x + y + z == 2020 {
                    println!("Day 1.2 Answer: {}", x * y * z);
                }
            }
        }
    }
}
