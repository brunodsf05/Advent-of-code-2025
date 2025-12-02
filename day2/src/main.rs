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

/// Examples:
///     "123123123" -> true because "123" == "123" == "123"
///     "111111"    -> true because // Composed of "1"s
fn is_text_made_of_sequence(text: &str) -> bool {
    let l = text.len();

    // The algorithm detects repeated sequences by comparing all blocks of text
    // side by side.
    // At the beginning, the text is split into a few big blocks, and as we go
    // it gets divided into more and smaller blocks until making each block
    // contain one character
    //
    // Example:
    //     With a string "6767676767" which is 10 characters long...
    //     1.  Let's divide it in two (10/2=5) -> `67676~76767`
    //     2.  As you can see both parts are different, so let's divide the
    //         string again in more smaller block:
    //         1. Three blocks is impossible because (10/3=3.33) is a float
    //         2. Four blocks is impossible because (10/4=2.5) is a float
    //         3. Five blocks is the answer because (10/5=2) is an integer
    //     3. If we compare all five blocks `67~67~67~67~67` we find out
    //        that are al equal!

    let mut block_num: usize = 1;

    while block_num < l {
        // Slice text into smaller blocks
        loop {
            block_num += 1;
            if l % block_num == 0 {
                break;
            }
        }

        // Count how many block are equal
        let block_size: usize = l / block_num;
        let mut offset: usize = block_size;
        let mut record = 1;
        for _ in 1..block_num {
            let (a, b) = (&text[..block_size], &text[offset..offset + block_size]);
            if a == b {
                record += 1;
            };
            offset += block_size;
        }

        // If all blocks are equal, then we've found the sequence
        if record == block_num {
            return true;
        }
    }

    return false;
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

    let password_2 = decode_mistery(&ranges, is_text_made_of_sequence);
    println!("Part 2: {password_2}");
}
