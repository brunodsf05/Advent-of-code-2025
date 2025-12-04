use std::ops::Range;

use utils::quick_read;

type Cell = bool;
type Row = Vec<Cell>;
type Grid = Vec<Row>;

fn parse_input_into_grid(text: &str) -> Grid {
    text.lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '@' => true,
                    _ => false,
                })
                .collect::<Row>()
        })
        .collect()
}

fn get_adjacent_cell(
    grid: &Grid,
    px: usize,
    py: usize,
    ox: i32,
    oy: i32,
    rx: &Range<i32>,
    ry: &Range<i32>,
) -> Cell {
    fn sum(p: usize, o: i32) -> i32 {
        p as i32 + o
    }

    let (x, y) = (sum(px, ox), sum(py, oy));

    if rx.contains(&x) && ry.contains(&y) {
        let (x, y) = (x as usize, y as usize);
        return grid[y][x];
    }

    return false;
}

fn decode_mistery_1(grid: &Grid) -> i32 {
    let rows_range = 0..grid.len() as i32;
    let mut forkliftable = 0;
    for (y, row) in grid.iter().enumerate() {
        let cols_range = 0..row.len() as i32;

        for (x, cell) in row.iter().enumerate() {
            let mut adjacent_rolls = 0;

            if *cell == false {
                continue; // Skip
            }

            let mut add = |cell: bool| {
                adjacent_rolls += match cell {
                    true => 1,
                    _ => 0,
                };
            };

            // 0 1 2
            // 3 @ 4
            // 5 6 7
            #[rustfmt::skip] add(get_adjacent_cell(&grid, x, y, -1, -1, &cols_range, &rows_range)); // 0
            #[rustfmt::skip] add(get_adjacent_cell(&grid, x, y,  0, -1, &cols_range, &rows_range)); // 1
            #[rustfmt::skip] add(get_adjacent_cell(&grid, x, y,  1, -1, &cols_range, &rows_range)); // 2
            #[rustfmt::skip] add(get_adjacent_cell(&grid, x, y, -1,  0, &cols_range, &rows_range)); // 3
            #[rustfmt::skip] add(get_adjacent_cell(&grid, x, y,  1,  0, &cols_range, &rows_range)); // 4
            #[rustfmt::skip] add(get_adjacent_cell(&grid, x, y, -1,  1, &cols_range, &rows_range)); // 5
            #[rustfmt::skip] add(get_adjacent_cell(&grid, x, y,  0,  1, &cols_range, &rows_range)); // 6
            #[rustfmt::skip] add(get_adjacent_cell(&grid, x, y,  1,  1, &cols_range, &rows_range)); // 7

            if adjacent_rolls < 4 {
                forkliftable += 1;
            }
        }
    }

    forkliftable
}

fn main() {
    // Parse input
    let text = quick_read!("input.txt");
    let grid = parse_input_into_grid(&text);

    // Return solutions
    let password_1 = decode_mistery_1(&grid);
    println!("Part 1: {password_1}");
}
