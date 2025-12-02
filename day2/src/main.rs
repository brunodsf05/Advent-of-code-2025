use std::ops::Range;
use utils::quick_read;

fn parse_text_into_ranges(text: &str) -> Vec<Range<u64>> {
    text.split(',')
        .map(|s| {
            s.split_once('-')
                .and_then(|(l, r)| if r.contains('-') { None } else { Some((l, r)) })
                .map_or_else(
                    || panic!("{s} range has an invalid format"),
                    |(l, r)| {
                        let parse = |s: &str| s.trim().parse::<u64>().unwrap();
                        Range::<u64> {
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
    println!("{ranges:#?}")
}
