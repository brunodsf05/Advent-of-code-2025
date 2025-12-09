use utils::{grid::*, quick_read};

enum Cell {
    Laser,
    Empty,
    Split,
}

impl EmptyCell for Cell {
    fn empty() -> Self {
        Self::Empty
    }
}

fn parse_input_into_grid(text: &str) -> Grid<Cell> {
    text.lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'S' | '|' => Cell::Laser,
                    '^' => Cell::Split,
                    _ => Cell::empty(),
                })
                .collect::<Row<Cell>>()
        })
        .collect()
}

fn decode_mistery_1(grid: &Grid<Cell>) -> usize {
    let mut grid = grid.clone();
    0
}

fn decode_mistery_2(grid: &Grid<Cell>) -> usize {
    0
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
