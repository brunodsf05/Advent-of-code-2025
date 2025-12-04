use utils::quick_read;

type Cell = bool;
type Grid = Vec<Vec<Cell>>;

fn parse_input_into_grid(text: &str) -> Grid {
    Vec::new()
}

fn main() {
    // Parse input
    let text = quick_read!("input.txt");
    let grid = parse_input_into_grid(&text);
}
