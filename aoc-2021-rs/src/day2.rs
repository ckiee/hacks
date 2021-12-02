use std::str::FromStr;

pub fn main() -> (u32, u32) {
    let input = String::from(include_str!("../inputs/2"));

    let data: Vec<Command> = input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.parse::<Command>().unwrap())
        .collect();

    (solve_1(data.clone()), solve_2(data.clone()))
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug, PartialEq, Clone)]
struct Command(Direction, u32);

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" ");

        Ok(Command(
            match iter.next().ok_or("invalid line")? {
                "up" => Ok(Direction::Up),
                "down" => Ok(Direction::Down),
                "forward" => Ok(Direction::Forward),
                other => Err(format!("unknown instr '{}'", other)),
            }?,
            iter.next().ok_or("invalid line")?.parse::<u32>().unwrap(),
        ))
    }
}

fn solve_1(data: Vec<Command>) -> u32 {
    let mut pos: (u32, u32) = (0, 0); // (horiz, depth)

    for cmd in data {
        match cmd.0 {
            Direction::Forward => {
                pos.0 += cmd.1;
            }
            Direction::Down => {
                pos.1 += cmd.1;
            }
            Direction::Up => {
                pos.1 -= cmd.1;
            }
        };
    }

    pos.0 * pos.1
}

fn solve_2(data: Vec<Command>) -> u32 {
    let mut pos: (u32, u32, u32) = (0, 0, 0); // (horiz, depth, aim)

    for cmd in data {
        match cmd.0 {
            Direction::Forward => {
                pos.0 += cmd.1;
                pos.1 += pos.2 * cmd.1;
            }
            Direction::Down => {
                pos.2 += cmd.1;
            }
            Direction::Up => {
                pos.2 -= cmd.1;
            }
        };
    }

    pos.0 * pos.1
}
