use std::ops::Range;
use utils::quick_read;

type Input = (Vec<Range<usize>>, Vec<usize>);

fn parse_input(text: &str) -> Input {
    let mut ranges: Vec<Range<usize>> = Vec::new();
    let mut numbers: Vec<usize> = Vec::new();

    for line in text.lines() {
        let parts: Vec<&str> = line.split('-').collect();

        match parts.len() {
            1 => {
                if let Ok(number) = parts[0].parse::<usize>() {
                    numbers.push(number)
                }
            }
            2 => {
                let (l, r) = (parts[0].parse::<usize>(), parts[1].parse::<usize>());
                if l.is_ok() && r.is_ok() {
                    let (l, r) = (l.unwrap(), r.unwrap());
                    ranges.push(Range {
                        start: l,
                        end: r + 1, // Makes it inclusive
                    })
                }
            }
            _ => (),
        };
    }

    return (ranges, numbers);
}

fn decode_mistery_1(input: &Input) -> usize {
    0
}

fn main() {
    // Parse input
    let text = quick_read!("input.txt");
    let input = parse_input(&text);

    // Return solutions
    let password_1 = decode_mistery_1(&input);
    println!("Part 1: {password_1}");

    let password_2 = 0;
    println!("Part 2: {password_2}");
}
