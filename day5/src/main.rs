use std::{collections::HashSet, ops::Range};
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
    let (ranges, numbers) = input;
    let mut fresh = 0;

    for n in numbers {
        for r in ranges {
            if r.contains(&n) {
                fresh += 1;
                break;
            }
        }
    }

    return fresh;
}

fn decode_mistery_2(input: &Input) -> usize {
    let (ranges, _) = input;

    #[derive(Debug)]
    enum End {
        L(usize),
        R(usize),
    }

    // Flatten the list of ranges into Ends.
    // [(10..=14) (3..=5) (16..=20) (12..=18)]
    // [L10 R14 L3 R5 L16 R20 L12 R18]

    let mut ends: Vec<End> = ranges
        .iter()
        .flat_map(|r| [End::L(r.start), End::R(r.end)])
        .collect();

    // Sort these ends, firts by number (0..N) then for direction (L,R).
    // From now, think of this list as numbered parenthesis.
    // [L3 R5 L10 L12 R14 L16 R18 R20]
    ends.sort_unstable_by(|a, b| match (a, b) {
        (End::L(x), End::L(y)) => x.cmp(y),
        (End::R(x), End::R(y)) => x.cmp(y),
        (End::L(x), End::R(y)) => x.cmp(y).then(std::cmp::Ordering::Less),
        (End::R(x), End::L(y)) => x.cmp(y).then(std::cmp::Ordering::Greater),
    });

    // To remove the overlapping ends, we can delete the end that is not of
    // depth zero.
    //
    // The depth is basically how much a pair of ends (L and R) is nested
    // inside another. Example is that for [L1 L2 R3 R4]:
    // 1.  L1 and R4 depth is 0.
    // 2.  L2 and R3 depth is 1 because is inside L1 and R4.
    //
    // By the way, the ends convert to flat numbers to make the next step
    // shorter.
    //
    // [(L3 R5) (L10 (L12 R14) (L16 R18) R20)]
    // [(L3 R5) (L10 R20)]
    // [3 5 10 20]
    let non_overlapping_ends: Vec<usize> = ends
        .iter()
        .fold((0usize, Vec::new()), |(mut depth, mut out), end| {
            match end {
                End::L(v) => {
                    if depth == 0 {
                        out.push(*v);
                    }
                    depth += 1;
                }
                End::R(v) => {
                    depth -= 1;
                    if depth == 0 {
                        out.push(*v);
                    }
                }
            }
            (depth, out)
        })
        .1;

    let mut fresh = 0;

    // Because we don't have any overlapping ends, we can finally calculate how
    // many numbers are inside in all ranges.
    //
    // [3 5 10 20]
    // [(3..=5) (10..=20)]
    // [(3..6) (10..21)]
    // [(6-3) (21 - 10)]
    // [(3) (11)]
    // sum = 14
    for ends in non_overlapping_ends.chunks_exact(2) {
        let (l, r) = (ends[0], ends[1]);
        fresh += r - l;
    }

    return fresh;
}

fn main() {
    // Parse input
    let text = quick_read!("input.txt");
    let input = parse_input(&text);

    // Return solutions
    let password_1 = decode_mistery_1(&input);
    println!("Part 1: {password_1}");

    let password_2 = decode_mistery_2(&input);
    println!("Part 2: {password_2}");
}
