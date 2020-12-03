use std::env;

mod day1;
mod day2;
mod day3;

fn main() {
    let day = env::args().nth(1).unwrap();
    match day.as_str() {
        "day1" => day1::run(),
		"day2" => day2::run(),
		"day3" => day3::run(),
        _ => {
            panic!("unknown day argument");
        }
    }
}
