pub fn main() -> (i64, i64) {
    let input = String::from(include_str!("../inputs/7"));

    let data: Vec<i64> = input
        .split(",")
        .map(|line| line.trim().parse::<i64>().unwrap())
        .collect();

    (solve(&data, false), solve(&data, true))
}
fn solve(data: &Vec<i64>, summed_fuel: bool) -> i64 {
    let mut min_cost = i64::MAX;
    let max_alignment = data.iter().max().unwrap();

    for alignment in 0..*max_alignment {
        let cost = data
            .clone()
            .iter()
            .map(|x| {
                let distance = (alignment - x).abs();
                if summed_fuel {
                    distance * (distance + 1) / 2
                } else {
                    distance
                }
            })
            .sum::<i64>();
        if cost < min_cost {
            min_cost = cost;
        }
    }

    min_cost
}
