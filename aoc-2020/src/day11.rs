use std::collections::HashMap;

const RAW_INPUT: &str = include_str!("../inputs/day11");

pub fn run() {
    part_1(&mut parse());
    // part_2(&data);
}

#[derive(Debug, Copy, Clone)]
enum SeatState {
    EMPTY,
    OCCUPIED,
    FLOOR,
}

fn parse() -> HashMap<(i64, i64), SeatState> {
    let mut data = HashMap::new();
    RAW_INPUT.lines().enumerate().for_each(|(i, x)| {
        x.chars().enumerate().for_each(|(j, y)| {
            data.insert(
                (i as i64, j as i64),
                match y {
                    'L' => SeatState::EMPTY,
                    '#' => SeatState::OCCUPIED,
                    '.' => SeatState::FLOOR,
                    _ => panic!("invalid input"),
                },
            );
        })
    });
    data
}

fn get_occupied_neighbours(data: &HashMap<(i64, i64), SeatState>, target: &(i64, i64)) -> u8 {
    let mut occupied = 0;
    for x in -1..1 {
        for y in -1..1 {
            match data.get(&(x + target.0, y + target.1)) {
                Some(&SeatState::OCCUPIED) => {
                    occupied += 1;
                }
                _ => {}
            };
        }
    }
    println!("{},{} :: {}", target.0, target.1, occupied);
    occupied
}

fn visualize(data: &HashMap<(i64, i64), SeatState>) {
    for y in 0..10 {
        for x in 0..10 {
            print!(
                "{}",
                match data.get(&(y, x)).unwrap() {
                    SeatState::OCCUPIED => "#",
                    SeatState::EMPTY => "L",
                    SeatState::FLOOR => ".",
                }
            )
        }
        println!("");
    }
}

fn tick(data: &mut HashMap<(i64, i64), SeatState>) -> i32 {
    let clone = data.clone();
    let mut changes = 0;
    for (coords, status) in data.clone() {
        &mut data.insert(
            coords,
            match status {
                SeatState::EMPTY if get_occupied_neighbours(&clone, &coords) == 0 => {
                    changes += 1;
                    SeatState::OCCUPIED
                }
                SeatState::OCCUPIED if get_occupied_neighbours(&clone, &coords) >= 4 => {
                    changes += 1;
                    SeatState::EMPTY
                }
                // default to staying the same
                x => x,
            },
        );
    }
    visualize(&data);
    changes
}

fn part_1(data: &mut HashMap<(i64, i64), SeatState>) {
    while tick(data) != 0 {
        println!("----")
    }
}

fn part_2(data: &HashMap<(i64, i64), SeatState>) {}
