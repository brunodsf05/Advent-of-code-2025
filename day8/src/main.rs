use std::fmt;
use utils::quick_read;

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "X={},Y={},Z={}", self.x, self.y, self.z)
    }
}

fn parse_input_into_points(text: &str) -> Vec<Point> {
    text.lines()
        .map(|l| {
            let tokens: Vec<&str> = l.split(',').collect();
            Point {
                x: tokens[0].parse::<i64>().unwrap(),
                y: tokens[1].parse::<i64>().unwrap(),
                z: tokens[2].parse::<i64>().unwrap(),
            }
        })
        .collect()
}

fn decode_mistery_1(points: &Vec<Point>) -> usize {
    0
}

fn decode_mistery_2(points: &Vec<Point>) -> usize {
    0
}

fn main() {
    // Parse input
    let text = quick_read!("input.txt");
    let points = parse_input_into_points(&text);
    points.iter().for_each(|p| println!("{p}"));

    // Return solutions
    let password_1 = decode_mistery_1(&points);
    println!("Part 1: {password_1}");

    let password_2 = decode_mistery_2(&points);
    println!("Part 2: {password_2}");
}
