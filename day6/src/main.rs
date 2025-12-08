use std::{collections::HashMap, iter::Sum};

use itertools::Itertools;
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

fn decode_mistery_2(text: &str) -> usize {
    let length = text.lines().count();
    assert!(length >= 2, "Text must have two or more lines");
    let last_index = length - 1;

    // Parse lines into columns of numbers.
    // Contiguous non-zero numbers belong to one operator. (+ has 1, 24 and 35)
    // Zero marks where a new operator will be used.
    //
    // Example:
    //     """
    //     123 6
    //      45 78
    //     +   *
    //     """
    //     vec![1, 24, 35, 0, 67, 8, 0]
    //
    let numbers: Vec<usize> = text
        .lines()
        .enumerate()
        .fold(vec![], |mut numbers, (y, line)| {
            // Ignore operators lines and push zero for sum iteration
            if y == last_index {
                numbers.push(0);
                return numbers;
            }

            line.char_indices().for_each(|(x, char)| {
                // Make sure we have a column available to write
                if numbers.len() <= x {
                    numbers.resize(x + 1, 0);
                }

                // Get the current digit and append the current digit
                if let Some(number) = numbers.get_mut(x)
                    && let Some(digit) = char.to_digit(10)
                {
                    *number = *number * 10 + digit as usize;
                }
            });

            numbers
        });

    // Iterate trough each column.
    // A column consist of a number and a operator.
    // If an operator is not present, the last-parsed operator will be used.
    // A column with zero marks the usage of a new operator.
    enum Op {
        Sum,
        Mul,
    }

    let last_line = text.lines().last().unwrap();
    let sum: usize = numbers
        .iter()
        .enumerate()
        .fold(
            (0, 0, Op::Sum),
            |(mut sum, mut acc, mut op), (x, number)| {
                let char = last_line.chars().nth(x).unwrap_or(' ');

                (op, acc) = match char {
                    '+' => (Op::Sum, 0),
                    '*' => (Op::Mul, 1),
                    _ => (op, acc),
                };

                if *number == 0 {
                    sum += acc;
                } else {
                    acc = match op {
                        Op::Sum => acc + *number,
                        Op::Mul => acc * *number,
                    };
                }

                (sum, acc, op)
            },
        )
        .0;

    return sum;
}

fn main() {
    // Parse input
    let text = quick_read!("input.txt");

    // Return solutions
    let password_1 = decode_mistery_1(&text);
    println!("Part 1: {password_1}");

    let password_2 = decode_mistery_2(&text);
    println!("Part 2: {password_2}");
}
