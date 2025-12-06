use std::collections::HashMap;

use utils::quick_read;

fn decode_mistery_1(text: &str) -> i64 {
    let mut sum = 0;
    let mut number_columns: HashMap<usize, Vec<i64>> = HashMap::new();

    // "111 222 333\n444 555 666" -> ["111 222 333", "444 555 666"]
    for line in text.lines() {
        // ["111 222 333"] -> ["111", "222", "333"]
        for (x, word) in line.split_whitespace().enumerate() {
            let column = number_columns.entry(x).or_insert(vec![]);

            sum += match word.trim() {
                "+" => column.iter().sum::<i64>(),
                "*" => column.iter().product::<i64>(),
                other => {
                    if let Ok(number) = other.parse::<i64>() {
                        column.push(number);
                    }
                    0i64
                }
            };
        }
    }

    return sum;
}

fn main() {
    // Parse input
    let text = quick_read!("input.txt");

    // Return solutions
    let password_1 = decode_mistery_1(&text);
    println!("Part 1: {password_1}");

    let password_2 = 0;
    println!("Part 2: {password_2}");
}
