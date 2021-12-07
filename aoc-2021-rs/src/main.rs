use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

macro_rules! day_perf_measure {
    ($($mod:path),+) => {
        fn main() {
            $(
                {
                    use $mod as base;
                    let now = Instant::now();
                    let result = base::main();
                    println!("{} [{:?}]: {:?}", stringify!($mod), now.elapsed(), result);
                }
            )*
        }
    }
}

day_perf_measure!(day1, day2, day3, day4, day5, day6, day7);
