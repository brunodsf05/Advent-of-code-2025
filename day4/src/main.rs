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

fn main() {
    // Parse input
    let text = quick_read!("input.txt");
    let grid = parse_input_into_grid(&text);
}
