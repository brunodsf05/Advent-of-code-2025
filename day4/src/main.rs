mod shared;

use shared::*;
use utils::quick_read;

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

const VALID_TO_FORKLIFT: i32 = 4;

fn decode_mistery_1(grid: &Grid) -> i32 {
    let rows_range = 0..grid.len() as i32;
    let mut forkliftable = 0;
    for (y, row) in grid.iter().enumerate() {
        let cols_range = 0..row.len() as i32;

        for (x, cell) in row.iter().enumerate() {
            let adjacent_rolls = count_adjacents_cell(&grid, x, y, &cols_range, &rows_range);

            if *cell == true && adjacent_rolls < VALID_TO_FORKLIFT {
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
