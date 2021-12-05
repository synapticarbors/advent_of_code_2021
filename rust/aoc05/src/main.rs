use std::collections::HashMap;

use anyhow::Result;
use nom::{bytes::complete::tag, sequence::separated_pair, IResult};

pub fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let soln_a = solve_a()?;
    eprintln!("Part A elapsed {:?}", start.elapsed());
    println!("solution part A: {}", soln_a);

    let start = std::time::Instant::now();
    let soln_b = solve_b()?;
    eprintln!("Part B elapsed {:?}", start.elapsed());
    println!("solution part B: {:?}", soln_b);

    Ok(())
}

type Point = (i32, i32);
type Segment = (Point, Point);

fn parse_line(s: &str) -> IResult<&str, Segment> {
    use nom::character::complete::i32;
    separated_pair(
        separated_pair(i32, tag(","), i32),
        tag(" -> "),
        separated_pair(i32, tag(","), i32),
    )(s)
}

pub fn solve_a() -> Result<usize> {
    let icnt = include_str!("../input")
        .lines()
        .fold(HashMap::new(), |mut acc: HashMap<(i32, i32), i32>, line| {
            let (_, ((x1, y1), (x2, y2))) = parse_line(line).unwrap();
            if (x1 == x2) || (y1 == y2) {
                for i in x1.min(x2)..=x1.max(x2) {
                    for j in y1.min(y2)..=y1.max(y2) {
                        *acc.entry((i, j)).or_default() += 1;
                    }
                }
            }

            acc
        })
        .values()
        .filter(|&&v| v > 1)
        .count();

    Ok(icnt)
}

pub fn solve_b() -> Result<usize> {
    let icnt = include_str!("../input")
        .lines()
        .fold(HashMap::new(), |mut acc: HashMap<(i32, i32), i32>, line| {
            let (_, ((x1, y1), (x2, y2))) = parse_line(line).unwrap();
            let dx = if x1 == x2 { 0 } else { (x2 - x1).signum() };
            let dy = if y1 == y2 { 0 } else { (y2 - y1).signum() };

            let npts = (x2 - x1).abs().max((y2 - y1).abs());
            let (mut i, mut j) = (x1, y1);

            for _ in 0..=npts {
                *acc.entry((i, j)).or_default() += 1;
                i += dx;
                j += dy;
            }

            acc
        })
        .values()
        .filter(|&&v| v > 1)
        .count();

    Ok(icnt)
}
