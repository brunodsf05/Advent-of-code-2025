use utils::quick_read;

type Bank = Vec<i32>;

fn parse_input_into_banks(text: &str) -> Vec<Bank> {
    Vec::new()
}

fn main() {
    // Parse input
    let text = quick_read!("input.txt");
    let banks = parse_input_into_banks(&text);
}
