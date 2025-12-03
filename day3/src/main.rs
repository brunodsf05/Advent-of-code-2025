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

fn main() {
    // Parse input
    let text = quick_read!("input.txt");
    let banks = parse_input_into_banks(&text);
}
