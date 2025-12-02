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

fn main() {
    // Parse input
    let text = quick_read!("input.txt");
    let ranges = parse_text_into_ranges(&text);
}
