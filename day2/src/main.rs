use std::ops::Range;
use utils::quick_read;

type Int = u64;

fn parse_text_into_ranges(text: &str) -> Vec<Range<Int>> {
    text.split(',')
        .map(|s| {
            s.split_once('-')
                .and_then(|(l, r)| if r.contains('-') { None } else { Some((l, r)) })
                .map_or_else(
                    || panic!("{s} range has an invalid format"),
                    |(l, r)| {
                        let parse = |s: &str| s.trim().parse::<Int>().unwrap();
                        Range::<Int> {
                            start: parse(l),
                            end: parse(r),
                        }
                    },
                )
        })
        .collect()
}

/// Examples:
///     "123123" -> true  because "123" == "123"
///     "11"     -> true  because   "1" == "1"
///     "1234"   -> false because  "12" == "34"
fn is_text_half_repeated(text: &str) -> bool {
    let l = text.len();

    if l & 1 == 1 {
        return false;
    }

    let half = l / 2;
    &text[..half] == &text[half..]
}

/// The mistery is the sum of the invalid ids
/// Invalidator is a function that returns true when invalid
fn decode_mistery<T>(ranges: &Vec<Range<Int>>, invalidator: T) -> Int
where
    T: Fn(&str) -> bool,
{
    ranges
        .iter()
        .flat_map(|r| r.start..r.end)
        .filter(|n| invalidator(&n.to_string()))
        .sum()
}

fn main() {
    // Parse input
    let text = quick_read!("input.txt");
    let ranges = parse_text_into_ranges(&text);

    // Return solutions
    let password_1 = decode_mistery(&ranges, is_text_half_repeated);
    println!("Part 1: {password_1}");
}
