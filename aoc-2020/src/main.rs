use std::env;

mod day1;

fn main() {
    let day = env::args().nth(1).unwrap();
    match day.as_str() {
        "day1" => {
            day1::run();
        }
        _ => {
            panic!("unknown day argument");
        }
    }
}
