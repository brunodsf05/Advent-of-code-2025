mod shared;

use shared::*;
use std::mem::swap;
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

fn decode_mistery_2(grid: &Grid) -> i32 {
    let rows_range = 0..grid.len() as i32;
    let mut forkliftable = 1;
    let mut removed = 0;

    let (mut grid_read, mut grid_write) = (grid.clone(), grid.clone());

    while forkliftable > 0 {
        forkliftable = 0;

        // We use two buffers to make sure each roll is
        // deleted after iterating trough all the lists and not after
        // each cell read.
        //
        // This avoids a bug where removing a roll immediately would let the
        // algorithm see that removal while still scanning the current pass,
        // causing cascaded deletions that should only happen on the next pass.

        for (y, row) in (&grid_read).iter().enumerate() {
            let cols_range = 0..row.len() as i32;

            for (x, cell) in row.iter().enumerate() {
                let adjacent_rolls =
                    count_adjacents_cell(&grid_read, x, y, &cols_range, &rows_range);

                if *cell == true && adjacent_rolls < VALID_TO_FORKLIFT {
                    forkliftable += 1;
                    grid_write[y][x] = false;
                }
            }
        }

        grid_read = grid_write;
        grid_write = grid_read.clone();

        removed += forkliftable;
    }

    removed
}

fn main() {
    // Parse input
    let text = quick_read!("input.txt");
    let grid = parse_input_into_grid(&text);

    // Return solutions
    let password_1 = decode_mistery_1(&grid);
    println!("Part 1: {password_1}");

    let password_2 = decode_mistery_2(&grid);
    println!("Part 2: {password_2}");
}
