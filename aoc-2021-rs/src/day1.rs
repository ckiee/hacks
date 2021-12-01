pub fn main() -> (u32, u32) {
    (solve(1), solve(3))
}

fn solve(window_size: usize) -> u32 {
    let input = String::from(include_str!("../inputs/1"));

    let data: Vec<u32> = input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    fn window_sum(data: &Vec<u32>, idx: usize, window_size: usize) -> u32 {
        let mut sum = 0;
        for x in &data[idx..idx + window_size] {
            sum += x;
        }
        sum
    }

    let mut larger = 0;
    let mut last = 0;
    for i in 0..(data.len() - window_size) {
        let now = window_sum(&data, i, window_size);
        if last < now {
            larger += 1;
        }
        last = now;
    }

    larger
}
