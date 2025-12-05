use std::ops::Range;
use utils::quick_read;

type Input = (Vec<Range<usize>>, Vec<usize>);

fn parse_input(text: &str) -> Input {
    (Vec::new(), Vec::new())
}

fn main() {
    // Parse input
    let text = quick_read!("input.txt");

    // Return solutions
    let password_1 = 0;
    println!("Part 1: {password_1}");

    let password_2 = 0;
    println!("Part 2: {password_2}");
}
