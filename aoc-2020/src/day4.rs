use std::collections::HashMap;

// use colored::*;
use regex::Regex;

const RAW_INPUT: &str = include_str!("../inputs/day4");

pub fn run() {
    part_1();
    part_2();
}

fn parse() -> Vec<HashMap<String, String>> {
    let data = RAW_INPUT.split("\n\n");
    let re = Regex::new(r"^(\w{3}):([\w\S]+)\s*").unwrap();
    let mut passports = Vec::new();
    for passport in data {
        let mut map = HashMap::new();
        let mut pos = 0;
        // println!("-----\n{}", passport);
        while pos < passport.len() - 1 {
            // if it errors here then you forgot \n suffix in input
            let cap = re.captures(&passport[pos..]).unwrap();
            // println!(
            //     "-----\nslice = {}, pos = {}",
            //     &passport[pos..].blue(),
            //     pos.to_string().red(),
            // );
            let key = cap.get(1).unwrap().as_str();
            let val = cap.get(2).unwrap().as_str();
            map.insert(key.to_string(), val.to_string());
            pos += cap.get(0).unwrap().as_str().len();
        }

        passports.push(map);
    }
    passports
}

fn part_1() {
    let passports = parse();
    let required_props = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut score = 0;

    for passport in passports {
        let mut props = Vec::new();
        for (k, _v) in passport.iter() {
            props.push(k);
        }
        // println!("{:?}", props);
        if required_props
            .iter()
            .all(|x| props.contains(&&(*x).to_string()))
        {
            score += 1;
        }
    }

    println!("Part 1: {}", score);
}

fn part_2() {
    let passports = parse();
    let required_props = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut score = 0;
    let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();
    let hcl_re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();

    for passport in passports {
        // println!("{:?}", props);
        /*
            byr (Birth Year) - four digits; at least 1920 and at most 2002.
            iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            hgt (Height) - a number followed by either cm or in:
                If cm, the number must be at least 150 and at most 193.
                If in, the number must be at least 59 and at most 76.
            hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            pid (Passport ID) - a nine-digit number, including leading zeroes.
            cid (Country ID) - ignored, missing or not.
        */
        // println!("pp={}", format!("{:?}", passport).blue());

        if required_props.iter().all(|x| {
            let val = passport.get(*x);
            let res = match *x {
                "byr" if val.is_some() => {
                    // assert_eq!(val.unwrap().len(), 4); //seems to never happen
                    let parsed = val.unwrap().parse::<i32>().unwrap();
                    parsed >= 1920 && parsed <= 2002
                }
                "iyr" if val.is_some() => {
                    // assert_eq!(val.unwrap().len(), 4); //seems to never happen
                    let parsed = val.unwrap().parse::<i32>().unwrap();
                    parsed >= 2010 && parsed <= 2020
                }
                "eyr" if val.is_some() => {
                    // assert_eq!(val.unwrap().len(), 4); //seems to never happen
                    let parsed = val.unwrap().parse::<i32>().unwrap();
                    parsed >= 2020 && parsed <= 2030
                }
                "hgt" if val.is_some() => {
                    if val.unwrap().ends_with("cm") {
                        let parsed = (&(val.unwrap())[..val.unwrap().len() - 2])
                            .parse::<i32>()
                            .unwrap();
                        parsed >= 150 && parsed <= 193
                    } else if val.unwrap().ends_with("in") {
                        let parsed = (&(val.unwrap())[..val.unwrap().len() - 2])
                            .parse::<i32>()
                            .unwrap();
                        parsed >= 59 && parsed <= 76
                    } else {
                        false
                    }
                }
                "hcl" if val.is_some() => hcl_re.find(val.unwrap()).is_some(),
                "ecl" if val.is_some() => match &val.unwrap()[..] {
                    "amb" => true,
                    "blu" => true,
                    "brn" => true,
                    "gry" => true,
                    "grn" => true,
                    "hzl" => true,
                    "oth" => true,
                    _ => false,
                },
                "pid" if val.is_some() => pid_re.find(val.unwrap()).is_some(),
                _ => {
                    // println!("{}","wtf".red());
                    false
                }
            };
            // println!("{}={}", x, if res { "yes".green() } else { "no".red() });
            res
        }) {
            score += 1;
        }
    }

    println!("Part 2: {}", score);
}
