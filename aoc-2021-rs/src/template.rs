pub fn main() -> (u32, u32) {
    let input = String::from(include_str!("../inputs/2"));

    let data: Vec<u32> = input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    (solve_1(data.clone()), 0)
}
fn solve_1(data: Vec<u32>) -> u32 {
    0
}
