use std::collections::HashMap;

const RAW_INPUT: &str = include_str!("../inputs/day7");

#[derive(Debug)]
struct Bags {
    quantity: i32,
    color: String,
}

pub fn run() {
    let data = parse();
    part_1(&data);
    part_2(&data);
}

fn parse() -> HashMap<String, Vec<Bags>> {
    let data = RAW_INPUT.split("\n");
    let mut bags = HashMap::new();
    for line in data {
        let mut split = line.split(" ");
        let color = format!("{} {}", split.next().unwrap(), split.next().unwrap()); // color color
        split.next(); // bags
        split.next(); // contain
        let raw_items = split.collect::<Vec<&str>>();
        let item_count = raw_items.len() / 4;
        let mut items = Vec::new();
        let mut raw_items_iter = raw_items.iter();

        for _ in 0..item_count {
            items.push(Bags {
                quantity: raw_items_iter.next().unwrap().parse().unwrap(),
                color: format!(
                    "{} {}",
                    raw_items_iter.next().unwrap(),
                    raw_items_iter.next().unwrap()
                ),
            });
            raw_items_iter.next().unwrap();
        }
        bags.insert(color, items);
    }
    bags
}
const TARGET_COLOR: &str = "shiny gold";

fn part_1(data: &HashMap<String, Vec<Bags>>) {
    let mut score = 0;
    fn recurse(v: &Vec<Bags>, data: &HashMap<String, Vec<Bags>>) -> i32 {
        let mut score = 0;
        for x in v {
            if x.color == TARGET_COLOR {
                score += 1;
            } else {
                score += recurse(data.get(&x.color).unwrap(), data);
            }
        }
        score
    }
    for (k, v) in data {
        if recurse(&v, data) >= 1 {
            score += 1;
        }
    }
    println!("Part 1: {}", score);
}

fn part_2(data: &HashMap<String, Vec<Bags>>) {
    let mut score = 0;
    fn recurse(v: &Vec<Bags>, data: &HashMap<String, Vec<Bags>>) -> i32 {
        let mut score = 0;
        for x in v {
            score += x.quantity;
            score += recurse(data.get(&x.color).unwrap(), data) * x.quantity;
        }
        score
    }
    for (k, v) in data {
        if k == TARGET_COLOR {
            score += recurse(&v, &data);
        }
    }
    println!("Part 2: {}", score);
}
