// use colored::*;

const RAW_INPUT: &str = include_str!("../inputs/day3");

pub fn run() {
    println!("Part 1: {}", solve_for_slope((1, 3)));
    part_2();
}

fn solve_for_slope(slope: (usize, usize)) -> i32 {
    let data = RAW_INPUT
        .split('\n')
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();
    // println!("data {:?}", lines);
    let mut coords: (usize, usize) = (0, 0);
    let mut tree_ouch = 0;
    while coords.0 < data.len() - 1 {
        coords.0 += slope.0;
        coords.1 += slope.1;

        if data[coords.0][coords.1 % data[coords.0].len()] {
            tree_ouch += 1;
        }
    }
    // visualize(&data, &coords); performance
    tree_ouch
}

fn part_2() {
    let slopes: [(usize, usize); 5] = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let res = slopes
        .iter()
        .map(|slope| solve_for_slope(*slope))
        .fold(1, |acc, x| acc * x);
    println!("Part 2: {}", res);
}

// this doesnt account for horiz offscreen
// fn visualize(data: &Vec<Vec<bool>>, highlight_pos: &(usize, usize)) {
//     for (i, line) in data.iter().enumerate() {
//         for (j, val) in line.iter().enumerate() {
//             print!(
//                 "{}",
//                 if highlight_pos.0 == i && highlight_pos.1 == j {
//                     "=".red()
//                 } else if *val {
//                     "#".blue()
//                 } else {
//                     ".".blue()
//                 }
//             );
//         }
//         println!("");
//     }
// }
