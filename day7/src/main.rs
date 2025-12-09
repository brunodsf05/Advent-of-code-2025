use utils::{grid::*, quick_read};

#[derive(Clone, PartialEq)]
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
    let mut splitters = 0;

    // Prepare iteration with RW grid
    let (rx, ry) = (0..(grid[0].len() as i32), 0..(grid.len() as i32));
    let mut grid = grid.clone();

    // Iterate trough every grid's coordinate.
    // Because the reads occur at the top row (y-1) and writes at the current
    // (y), we don't need two grids.
    for y in 1..grid.len() {
        for x in 0..grid[y].len() {
            let cell = grid[y][x].clone();

            if get_adjacent_cell(&grid, x, y, 0, -1, &rx, &ry) != Cell::Laser {
                continue;
            };

            match &cell {
                Cell::Laser => (),
                Cell::Empty => {
                    grid[y][x] = Cell::Laser;
                }
                Cell::Split => {
                    splitters += 1;
                    grid[y][x - 1] = Cell::Laser;
                    grid[y][x + 1] = Cell::Laser;
                }
            };
        }
    }

    return splitters;
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
