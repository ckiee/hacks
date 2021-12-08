pub fn main() -> (u32, u32) {
    let input = String::from(include_str!("../inputs/8"));

    let parsed = input
        .split("\n")
        .filter(|line| line.len() != 0)
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();

    let part_1 = parsed.iter().map(|pl| solve_for_line(pl.clone())).sum();

    (part_1, 0)
}

type ParsedLine = (Vec<String>, Vec<String>);
fn parse_line(line: &str) -> ParsedLine {
    let mut split = line.split(" | ");

    let possible_digits: Vec<String> = split
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.trim().to_string())
        .collect();
    let enabled_digits: Vec<String> = split
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.trim().to_string())
        .collect();

    (possible_digits, enabled_digits)
}

fn solve_for_line(data: ParsedLine) -> u32 {
    let unique_possibles = data
        .0
        .iter()
        .enumerate()
        .filter(|(_, x)| data.0.iter().filter(|y| y.len() == x.len()).count() == 1)
        .map(|(_, x)| x.len())
        .collect::<Vec<_>>();

    data.1
        .iter()
        .filter(|x| {
            unique_possibles
                .iter()
                .filter(|y_len| (**x).len() == **y_len)
                .count()
                > 0
        })
        .count() as u32
}
