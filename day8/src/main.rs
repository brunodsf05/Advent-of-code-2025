use std::fmt;
use utils::quick_read;

// --- Data Structures ---
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

/// Represents a connection between two points.
/// Stores indeces of a list of points.
struct Edge(usize, usize);

/// Stores an `Edge` with the squared length between two points.
struct Vector {
    e: Edge,
    l: i64,
}

// --- Formatting ---

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "X={},Y={},Z={}", self.x, self.y, self.z)
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L{}-R{}", self.0, self.1)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.e, self.l)
    }
}

// --- Data processing ---

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

fn generate_vectors(points: &Vec<Point>) -> Vec<Vector> {
    let mut vectors: Vec<Vector> = vec![];

    for li in 0..points.len() - 1 {
        let lp = &points[li];
        for ri in (li + 1)..points.len() {
            let rp = &points[ri];
            let squared_distanced =
                (lp.x - rp.x).pow(2) + (lp.y - rp.y).pow(2) + (lp.z - rp.z).pow(2);

            vectors.push(Vector {
                e: Edge(li, ri),
                l: squared_distanced,
            });
        }
    }

    vectors.sort_by_key(|v| v.l);

    return vectors;
}

// --- Solutions ---

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
    let vectors = generate_vectors(&points);
    vectors.iter().for_each(|v| println!("{v}"));

    // Return solutions
    let password_1 = decode_mistery_1(&points);
    println!("Part 1: {password_1}");

    let password_2 = decode_mistery_2(&points);
    println!("Part 2: {password_2}");
}
