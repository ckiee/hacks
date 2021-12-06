
pub fn main() -> (u64, u64) {
    let input = String::from(include_str!("../inputs/6"));

    let data: Vec<u64> = input
        .split(",")
        .map(|x| x.trim_end().parse::<u64>().unwrap())
        .collect();

    (simulate(&data, 80), simulate(&data, 256))
}

fn get_expected() -> Vec<Vec<u64>> {
    let input = String::from(include_str!("../inputs/6-exp"));

    let data: Vec<Vec<u64>> = input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.split(",")
                .map(|x| x.trim_end().parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();
    data
}

fn simulate(data: &Vec<u64>, sim_days: u64) -> u64 {
    let mut counts = vec![0u64; 10];

    for fish in data {
        let ptr = counts.get_mut(*fish as usize).unwrap();
        *ptr += 1;
    }

    // let exp = get_expected();
    let mut last_breed_count = 0;
    for _ in 0..sim_days {
        // assert_eq!(&exp.get(day as usize).unwrap()[..8], &counts[..8]);
        // println!("#{}  {:?}", day, counts);

        {
            let zero_ptr = counts.get_mut(0).unwrap();
            *zero_ptr = 0;
            let six_ptr = counts.get_mut(7).unwrap();
            *six_ptr += last_breed_count;
            let eight_ptr = counts.get_mut(9).unwrap();
            *eight_ptr = last_breed_count;
        }

        for idx in 1..9 {
            counts.swap(idx, idx - 1);
        }

        {
            let nine = counts.get(9).unwrap().clone();
            let eight_ptr = counts.get_mut(8).unwrap();
            *eight_ptr += nine;
        }
        last_breed_count = counts.get(0).unwrap().clone();
    }

    {
        let nine_ptr = counts.get_mut(9).unwrap();
        *nine_ptr = 0;
    }

    counts.iter().sum()
}
