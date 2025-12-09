use std::fs;
use std::process::exit;

pub fn quick_read_impl(base: &str, path: &str) -> String {
    let full_path = std::path::Path::new(base).join(path);

    match fs::read_to_string(&full_path) {
        Ok(t) => t,
        Err(e) => {
            println!("Couldn't read {full_path:?}.\n{e:?}");
            exit(-1);
        }
    }
}

/// Reads a file, if there is an error, the program closes.
/// The path is relative to the proyect's `Cargo.toml` file. That means you can
/// reference the file relatively from a proyect and not from workspace's CWD.
#[macro_export]
macro_rules! quick_read {
    ($path:expr) => {{ $crate::quick_read_impl(env!("CARGO_MANIFEST_DIR"), $path) }};
}

pub mod grid {
    use std::ops::Range;

    /// Defines the grid cell type.
    pub trait EmptyCell {
        /// Returned when request is out-of-bounds.
        fn empty() -> Self;
    }

    pub type Row<T> = Vec<T>;
    pub type Grid<T> = Vec<Row<T>>;

    /// Gets a cell relatively from another.
    pub fn get_adjacent_cell<T: EmptyCell + Clone>(
        grid: &Grid<T>,
        px: usize,
        py: usize,
        ox: i32,
        oy: i32,
        rx: &Range<i32>,
        ry: &Range<i32>,
    ) -> T {
        // Helper to sum offsets
        fn sum(p: usize, o: i32) -> i32 {
            p as i32 + o
        }

        let x = sum(px, ox);
        let y = sum(py, oy);

        if rx.contains(&x) && ry.contains(&y) {
            let (xu, yu) = (x as usize, y as usize);
            return grid[yu][xu].clone();
        }

        T::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_read_hello() {
        let result = quick_read!("assets/hello.txt");
        assert_eq!(result.trim(), "Hello world!");
    }
}
