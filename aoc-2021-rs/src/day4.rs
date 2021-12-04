use cli_table::{print_stdout as print_table, Cell, Color, Style, Table, TableStruct};

pub fn main() -> (u32, u32) {
    let mut data = parse();
    // print_grids(&data.1);
    let mut bingo_scores = vec![];
    let mut winner_grids = vec![];

    'main: for draw in data.0 {
        // println!("");
        // println!("Draw is {}", draw);
        // println!("");
        apply_draw_to_grids(draw, &mut data.1);
        // print_grids(&data.1);
        for (idx, grid) in data.1.iter().enumerate() {
            if get_grid_bingo(&grid) && !winner_grids.contains(&idx) {
                winner_grids.push(idx);
                bingo_scores.push(grid_unmarked_sum(grid) * draw);
            }
            if winner_grids.len() == data.1.len() {
                break 'main
            }
        }
    }

    (*bingo_scores.first().unwrap(), *bingo_scores.last().unwrap())
}

fn apply_draw_to_grids(draw: u32, grids: &mut Vec<Grid>) {
    for grid in grids {
        for row in grid {
            for idx in 0..5 {
                let pixel = row.get_mut(idx).unwrap();
                pixel.marked = pixel.value == draw || pixel.marked;
            }
        }
    }
}

fn parse() -> Parsed {
    let input = String::from(include_str!("../inputs/4"));
    let mut lines = input.split("\n").into_iter().peekable();

    let draws: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut grids: Vec<Grid> = vec![];

    loop {
        if lines.next().is_none() || lines.peek().is_none() {
            break;
        }
        let mut grid: Grid = vec![];
        for _ in 0..5 {
            grid.push(
                lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                    .map(|x| Pixel {
                        marked: false,
                        value: x,
                    })
                    .collect::<Vec<Pixel>>(),
            );
        }
        grids.push(grid);
    }

    (draws, grids)
}

fn print_grids(grids: &Vec<Grid>) {
    for (idx, grid) in grids.iter().enumerate() {
        println!("Grid #{}", idx);
        print_table(grid_into_table(grid)).unwrap();
    }
}

fn grid_unmarked_sum(grid: &Grid) -> u32 {
    grid.iter()
        .map(|row| {
            row.iter()
                .filter(|pixel| !pixel.marked)
                .map(|pixel| pixel.value)
                .sum::<u32>()
        })
        .sum()
}

fn get_grid_bingo(grid: &Grid) -> bool {
    let mut has_bingo = false;
    for row in grid {
        has_bingo = has_bingo || row.iter().all(|pixel| pixel.marked);
    }
    for col_idx in 0..5 {
        let mut col = vec![];
        for row in grid {
            col.push(row.get(col_idx).unwrap());
        }
        has_bingo = has_bingo || col.iter().all(|pixel| pixel.marked);
    }
    has_bingo
}

fn grid_into_table(grid: &Grid) -> TableStruct {
    fn get_cell_bg(pixel: &Pixel) -> Option<Color> {
        if pixel.marked {
            Some(Color::White)
        } else {
            None
        }
    }
    fn get_cell_fg(pixel: &Pixel) -> Option<Color> {
        if pixel.marked {
            Some(Color::Black)
        } else {
            None
        }
    }

    grid.clone()
        .iter()
        .map(|row| {
            row.iter()
                .map(|pixel| {
                    pixel
                        .value
                        .cell()
                        .background_color(get_cell_bg(&pixel))
                        .foreground_color(get_cell_fg(&pixel))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .table()
}

#[derive(Debug, Clone)]
struct Pixel {
    marked: bool,
    value: u32,
}

type Grid = Vec<Vec<Pixel>>;

type Parsed = (Vec<u32>, Vec<Grid>);
