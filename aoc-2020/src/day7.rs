const RAW_INPUT: &str = include_str!("../inputs/day7");

#[derive(Debug)]
struct BagBag {
    quantity: usize,
    color: String,
}

pub fn run() {
    parse();
}

fn parse() {
    let data = RAW_INPUT.split("\n");
    // let mut bags = HashMap::new();
    for line in data {
        let mut split = line.split(" ");
        let color = format!("{} {}", split.next().unwrap(), split.next().unwrap()); // color color
        split.next(); // bags
        split.next(); // contain
        let raw_items = split.collect::<Vec<&str>>();
        let item_count = raw_items.len() / 4;
        let mut items = Vec::new();
        let mut raw_items_iter = raw_items.iter();

        for _ in 0..item_count {
            items.push(BagBag {
                quantity: raw_items_iter.next().unwrap().parse().unwrap(),
                color: format!(
                    "{} {}",
                    raw_items_iter.next().unwrap(),
                    raw_items_iter.next().unwrap()
                ),
            });
            raw_items_iter.next().unwrap();
        }
        println!("{:?}", items);

        // let items = vec![];
        // bags.insert(color, items);
    }
}
