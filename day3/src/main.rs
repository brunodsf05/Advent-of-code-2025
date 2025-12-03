use utils::quick_read;

type Bank = Vec<i32>;

fn parse_input_into_banks(text: &str) -> Vec<Bank> {
    text.lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_string().parse::<i32>().ok())
                .collect::<Bank>()
        })
        .collect()
}

fn decode_mistery_1(banks: &Vec<Bank>) -> i32 {
    banks
        .iter()
        .map(|bank| {
            let length = bank.len();
            let mut max_joltage = 0;

            for (l_idx, l_num) in bank[..(length - 1)].iter().enumerate() {
                let offset = l_idx + 1;
                for r_num in bank[offset..length].iter() {
                    max_joltage = max_joltage.max(l_num * 10 + r_num);
                }
            }

            max_joltage
        })
        .sum()
}

fn main() {
    // Parse input
    let text = quick_read!("input.txt");
    let banks = parse_input_into_banks(&text);

    // Return solutions
    let maxjoltage_1 = decode_mistery_1(&banks);
    println!("Part 1: {maxjoltage_1}")
}
