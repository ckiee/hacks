use std::{env::args, fmt::Write};

fn main() {
    // I don't like this library. Feels like something from Go with how you take a file as a lame String and the forced API
    let img = bmp::open(args().last().unwrap()).unwrap();
    println!("{:?}", img);
    // For each letter, the pixel coords that are on
    let mut letters: Vec<Vec<(u32, u32)>> = vec![vec![]; 26];
    for (x, y) in img.coordinates() {
        let pix = img.get_pixel(x, y);
        // separator lines
        if x != 0 && x % 4 == 3 {
            continue;
        }

        // on
        if pix.r == 156 {
            let inner = letters.get_mut((x / 4) as usize).unwrap();
            inner.push((x % 4, y));
        }
    }


    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    for (i, letter) in letters.iter().enumerate() {
        println!("Letter '{}'", alphabet.chars().collect::<Vec<_>>()[i]);
        let mut to_sort = vec![];
        for coord in letter {
            to_sort.push(match coord {
                (1, 2) => 1,
                (0, 0) => 2,
                (0, 4) => 3,
                (0, 2) => 4,
                (2, 3) => 5,
                (2, 1) => 6,
                (1, 0) => 7,
                (0, 1) => 8,
                (0, 3) => 9,
                (1, 4) => 10,
                (2, 0) => 11,
                (2, 2) => 12,
                (2, 4) => 13,
                _ => unimplemented!(),
            });
        }
        to_sort.sort();
        println!("{:?}", to_sort);
    }
}
