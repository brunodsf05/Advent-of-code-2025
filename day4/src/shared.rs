use std::ops::Range;

pub type Cell = bool;
pub type Row = Vec<Cell>;
pub type Grid = Vec<Row>;

/// Gets a cell relatively from another
/// Example:
///
/// · @ ·
/// · # ·
/// · · ·
///
/// If we are at the center (1,1) which is # and our goal is to read the top
/// one "@", we must use an offset of (0,-1).
///
/// Note: Y axis top..bottom is 0..positive
pub fn get_adjacent_cell(
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

/// Counts how many rolls are adjacent to an position.
pub fn count_adjacents_cell(
    grid: &Grid,
    px: usize,
    py: usize,
    rx: &Range<i32>,
    ry: &Range<i32>,
) -> i32 {
    let mut adjacent_rolls = 0;

    let mut add = |cell: bool| {
        adjacent_rolls += match cell {
            true => 1,
            _ => 0,
        };
    };
    // 0 1 2
    // 3 @ 4
    // 5 6 7
    #[rustfmt::skip] add(get_adjacent_cell(&grid, px, py, -1, -1, &rx, &ry)); // 0
    #[rustfmt::skip] add(get_adjacent_cell(&grid, px, py,  0, -1, &rx, &ry)); // 1
    #[rustfmt::skip] add(get_adjacent_cell(&grid, px, py,  1, -1, &rx, &ry)); // 2
    #[rustfmt::skip] add(get_adjacent_cell(&grid, px, py, -1,  0, &rx, &ry)); // 3
    #[rustfmt::skip] add(get_adjacent_cell(&grid, px, py,  1,  0, &rx, &ry)); // 4
    #[rustfmt::skip] add(get_adjacent_cell(&grid, px, py, -1,  1, &rx, &ry)); // 5
    #[rustfmt::skip] add(get_adjacent_cell(&grid, px, py,  0,  1, &rx, &ry)); // 6
    #[rustfmt::skip] add(get_adjacent_cell(&grid, px, py,  1,  1, &rx, &ry)); // 7

    return adjacent_rolls;
}
