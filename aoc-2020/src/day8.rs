use std::collections::HashSet;

const RAW_INPUT: &str = include_str!("../inputs/day8");

#[derive(Copy, Clone)]
enum Inst {
    Accumulate(i32),
    Nop(i32),
    Jump(i32),
}

pub fn run() {
    let data = parse();
    println!("Part 1: {}", solve(&data, None).0);
    part_2(&data);
}

fn part_2(data: &Vec<Inst>) {
    for i in 0..data.len() {
        let res = solve(&data, Some(i));
        if res.1 == data.len() {
            println!("Part 2: {}", res.0);
        }
    }
}


fn parse() -> Vec<Inst> {
    let data = RAW_INPUT.split("\n");
    let mut insts = Vec::with_capacity(601);
    for line in data {
        let mut iter = line.split(" ");
        let raw_inst = iter.next().unwrap();
        let argument = iter.next().unwrap().parse().unwrap();
        insts.push(match raw_inst {
            "acc" => Inst::Accumulate(argument),
            "nop" => Inst::Nop(argument),
            "jmp" => Inst::Jump(argument),
            _ => panic!("parse error"),
        });
    }
    insts
}

fn solve(data: &Vec<Inst>, swap_index: Option<usize>) -> (i32, usize) {
    let mut i = 0;
    let mut accu = 0;
    let mut already_seen = HashSet::new();
    while i < data.len() {
        let swapoo = &match &data[i] {
            Inst::Nop(x) => Inst::Jump(*x),
            Inst::Jump(x) => Inst::Nop(*x),
            x => *x,
        };
        let inst = if swap_index.is_some() && swap_index.unwrap() == i {
            swapoo
        } else {
            &data[i]
        };
        if already_seen.contains(&i) {
            return (accu, i);
        }
        already_seen.insert(i);
        match inst {
            Inst::Accumulate(x) => {
                accu += x;
                i += 1;
            }
            Inst::Nop(_x) => {
                i += 1;
            }
            Inst::Jump(x) => {
                let mut tmp_i = i as i32;
                tmp_i += x;
                i = tmp_i as usize;
            }
        }
    }
    (accu, i) // should never happen because of the other return stmt
}
