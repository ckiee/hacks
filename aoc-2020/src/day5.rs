
// use colored::*;
const RAW_INPUT: &str = include_str!("../inputs/day5");
#[derive(Debug)]
enum Direction {
    Front,
    Back,
    Left,
    Right,
}

fn parse() -> Vec<Vec<Direction>> {
    RAW_INPUT
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    'F' => Direction::Front,
                    'B' => Direction::Back,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("invalid"),
                })
                .collect::<Vec<Direction>>()
        })
        .collect::<Vec<Vec<Direction>>>()
}

pub fn run() {
    let data = parse();
    let mut used_seat_ids = Vec::new();

    for line in data {
        let mut available_y_min = 0;
        let mut available_y_max = 127;
        let mut available_x_min = 0;
        let mut available_x_max = 7;
        for dir in line {
            let y_area = available_y_max - available_y_min;
            let x_area = available_x_max - available_x_min;

            match dir {
                Direction::Front => {
                    available_y_max -= y_area / 2 + 1;
                }
                Direction::Back => {
                    available_y_min += y_area / 2 + 1;
                }
                Direction::Left => {
                    available_x_max -= x_area / 2 + 1;
                }
                Direction::Right => {
                    available_x_min += x_area / 2 + 1;
                }
            };
            println!(
                "{:?}: {}-{} ({}) \t\t{}-{} ({})",
                dir,
                available_y_min,
                available_y_max,
                y_area,
                available_x_min,
                available_x_max,
                x_area
            );
        }
        assert_eq!(available_x_max, available_x_min); // failed to narrow down if these asserts break
        assert_eq!(available_y_max, available_y_min);
        let seat_id = available_y_min * 8 + available_x_min;
        used_seat_ids.push(seat_id);
        println!("------");
    }

    used_seat_ids.sort();
    println!("highest (pt1): {}", used_seat_ids.last().unwrap());
    for (i, x) in used_seat_ids.iter().enumerate() {
        let maybe_next = used_seat_ids.get(i + 1);
        if maybe_next.is_none() {
            break;
        }
		let next = maybe_next.unwrap();
		if *next != (x + 1) {
			println!("our seat (pt2): {}", x +1);
		}
    }
}
