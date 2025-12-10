use utils::{grid::*, quick_read};

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Laser(usize),
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
                    'S' | '|' => Cell::Laser(1),
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

            if !matches!(
                get_adjacent_cell(&grid, x, y, 0, -1, &rx, &ry),
                Cell::Laser(_)
            ) {
                continue;
            };

            match &cell {
                Cell::Laser(_) => (),
                Cell::Empty => {
                    grid[y][x] = Cell::Laser(1);
                }
                Cell::Split => {
                    splitters += 1;
                    grid[y][x - 1] = Cell::Laser(1);
                    grid[y][x + 1] = Cell::Laser(1);
                }
            };
        }
    }

    return splitters;
}

fn decode_mistery_2(grid: &Grid<Cell>) -> usize {
    // Prepare iteration with RW grid
    let (rx, ry) = (0..(grid[0].len() as i32), 0..(grid.len() as i32));
    let mut grid = grid.clone();

    // Iterate trough every grid's coordinate.
    // Because the reads occur at the top row (y-1) and writes at the current
    // (y), we don't need two grids.
    for y in 1..grid.len() {
        for x in 0..grid[y].len() {
            let cell = grid[y][x].clone();

            // Top cell must be a laser, this value will be used when the
            // current cell is a splitter.
            let laser_strength = match get_adjacent_cell(&grid, x, y, 0, -1, &rx, &ry) {
                Cell::Laser(v) => v,
                _ => continue,
            };

            // The way to detect timelines is by counting how many times a
            // laser passed through the same way.
            let mut stack_laser = |ox: i32| {
                let laser_x = (x as i32 + ox) as usize;
                let ox_laser_strength = match grid[y][laser_x] {
                    Cell::Laser(strength) => strength,
                    _ => 0,
                };
                grid[y][laser_x] = Cell::Laser(laser_strength + ox_laser_strength);
            };

            match &cell {
                Cell::Empty | Cell::Laser(_) => {
                    stack_laser(0);
                }
                Cell::Split => {
                    stack_laser(-1);
                    stack_laser(1);
                }
            };
        }
    }

    // Finally count in the last rough

    /*
    // Visualization
    for row in &grid {
        for cell in row {
            print!(
                "{}",
                match cell {
                    Cell::Laser(strength) => format!("{:X}", strength % 16),
                    Cell::Empty => "·".to_string(),
                    Cell::Split => "^".to_string(),
                }
            );
        }
        println!();
    }
    */

    return grid
        .last()
        .unwrap()
        .iter()
        .filter_map(|cell| match cell {
            Cell::Laser(v) => Some(*v),
            _ => None,
        })
        .sum::<usize>();
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
