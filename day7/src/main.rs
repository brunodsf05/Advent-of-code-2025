use utils::quick_read;

fn decode_mistery_1(text: &str) -> usize {
    0
}

fn decode_mistery_2(text: &str) -> usize {
    0
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
