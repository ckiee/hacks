use std::{collections::HashMap, hash::{BuildHasher, Hash}, str::FromStr};

pub fn main() -> (i32, i32) {
    let input = String::from(include_str!("../inputs/5"));

    let data: Vec<Line> = input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.parse::<Line>().unwrap())
        .collect();

    (solve(data.clone(), true), solve(data.clone(), false))
}

#[derive(Debug, Clone)]
struct Line {
    from: (i32, i32),
    to: (i32, i32),
    orig: String,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" -> ");

        fn tuple2_from_str(s: &str) -> (i32, i32) {
            let mut iter = s.split(",").map(|x| x.parse::<i32>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        }

        Ok(Line {
            from: tuple2_from_str(split.next().unwrap()),
            to: tuple2_from_str(split.next().unwrap()),
            orig: s.to_string(),
        })
    }
}

trait IncrementInsert<K> {
    fn increment_insert(&mut self, k: K);
}

impl<K, S> IncrementInsert<K> for HashMap<K, i32, S>
where
    K: Eq + Hash + Clone,
    S: BuildHasher,
{
    fn increment_insert(&mut self, k: K) {
        match self.insert(k.clone(), 1) {
            Some(old) => { self.insert(k, old + 1); },
            None => {}
        };
    }
}

type LineMap = HashMap<(i32, i32), i32>;
fn solve(data: Vec<Line>, no_diagonal: bool) -> i32 {
    let mut line_map: LineMap = HashMap::with_capacity(188888);

    for line in data {
        let change_vector = (
            (line.to.0 - line.from.0).signum(),
            (line.to.1 - line.from.1).signum(),
        );
        if change_vector.0 != 0 && change_vector.1 != 0 && no_diagonal {
            continue
        }
        let mut coord = line.from.clone();
        // println!("{} (Cv{:?})", line.orig, change_vector);
        line_map.increment_insert(coord);
        while coord != line.to {
            coord.0 += change_vector.0;
            coord.1 += change_vector.1;
            // println!("  {:?}", coord);
            line_map.increment_insert(coord);
        }
    }


    line_map
        .iter()
        .filter(|(_, x)| **x > 1)
        .count()
        .try_into()
        .unwrap()
}
