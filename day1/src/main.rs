use std::ops::Neg;
use utils::quick_read;

/// Converts multiline text into signed rotations.
///
/// Rules:
/// - First char: L → negative, R → positive; otherwise 0.
/// - Rest of line: integer; parse errors → 0.
/// - Signs on the integer are ignored except for direction.
///
/// Returns a vector of signed `i32` rotations.
fn parse_text_into_rotations(text: &str) -> Vec<i32> {
    text.lines()
        .map(|line| {
            let (direction, str_number) = line.trim().split_at(1);

            let is_negative = match direction.to_ascii_uppercase().as_str() {
                "L" => true,
                "R" => false,
                _ => return 0,
            };

            let number = match str_number.parse::<i32>() {
                Ok(n) => n,
                Err(_) => return 0,
            };

            if is_negative {
                number.neg()
            } else {
                number.abs()
            }
        })
        .filter(|&n| n != 0)
        .collect()
}

const RADIAL_SIZE: i32 = 100;
const RADIAL_START: i32 = 50;

/// We initialize a virtual dial with `RADIAL_SIZE` numbers.
/// The mistery is the number of times each rotation lands on `0`.
fn decode_mistery_1(rotations: &Vec<i32>) -> i32 {
    let mut pointer: i32 = RADIAL_START;
    let mut password: i32 = 0;

    for &r in rotations.iter() {
        pointer = (r + pointer).rem_euclid(100);

        if pointer == 0 {
            password += 1;
        };
    }

    return password;
}

/// We initialize a virtual dial with `RADIAL_SIZE` numbers.
/// The mistery is the number of times each rotation makes the dial's `pointer` click at `0`.
fn decode_mistery_2(rotations: &Vec<i32>) -> i32 {
    let mut pointer: i32 = RADIAL_START;
    let mut password: i32 = 0;

    // The number of click's at `0` is counted how many circles are drawn:
    // 1.  A circle is drawn by clicking from origin to origin.
    // 2.  Our circle's origin is `pointer`.
    // 3.  To count the `clicks` done at `0` and not `pointer`, the number
    //     of clicks must include the distance of `pointer` to `0`.
    for &r in rotations.iter() {
        let clicks_from_pointer_to_zero = if r >= 0 {
            pointer
        } else {
            (RADIAL_SIZE - pointer) % RADIAL_SIZE
        };
        let clicks = clicks_from_pointer_to_zero + r.abs();
        let full_cycles = clicks / RADIAL_SIZE;

        password += full_cycles;
        pointer = (r + pointer).rem_euclid(100);
    }

    return password;
}

fn main() {
    // Parse input
    let text = quick_read!("input.txt");
    let rotations = parse_text_into_rotations(&text);

    // Return solutions
    let password_1 = decode_mistery_1(&rotations);
    println!("Part 1: {password_1}");

    let password_2 = decode_mistery_2(&rotations);
    println!("Part 2: {password_2}");
}
