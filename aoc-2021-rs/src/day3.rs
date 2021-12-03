const MAX_POS: u32 = 12;

pub fn main() -> (u32, u32) {
    let input = String::from(include_str!("../inputs/3"));

    let data: Vec<u32> = input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    (solve_1(data.clone()), solve_2(data.clone()))
}

fn solve_1(data: Vec<u32>) -> u32 {
    get_common_bits(data.clone(), false) * get_common_bits(data.clone(), true)
}

fn solve_2(data: Vec<u32>) -> u32 {
    rolling_common_filter(data.clone(), false) * rolling_common_filter(data.clone(), true)
}

fn rolling_common_filter(data: Vec<u32>, invert: bool) -> u32 {
    let mut common_nums = data.clone();
    for pos in 0..MAX_POS {
        let common = get_common_bits(common_nums.clone(), invert);
        common_nums = common_nums.into_iter().filter(|x| {
            get_big_endian_bit(*x, pos) == get_big_endian_bit(common, pos)
        }).collect();
        if common_nums.len() == 1 {
            return *common_nums.first().unwrap();
        }
    }
    unreachable!();
}

fn get_common_bits(data: Vec<u32>, invert: bool) -> u32 {
    let mut res: u32 = 0;
    for pos in 0..MAX_POS {
        let mut one_count = 0;
        for num in &data {
            if get_bit(*num, pos) {
                one_count += 1;
            }
        }
        let is_one_common = one_count >= data.len() - one_count;
        if (invert && !is_one_common) || (!invert && is_one_common) {
            res |= 1 << pos;
        } else {
            res &= !(1 << pos);
        }
    }
    res
}


fn get_bit(num: u32, bit: u32) -> bool {
    (num & (1 << bit)) != 0
}

fn get_big_endian_bit(num: u32, bit: u32) -> bool {
    get_bit(num, MAX_POS - 1 - bit)
}
