use anyhow::Result;
use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::complete::i32 as ni32,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Target {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
}

impl Target {
    fn overshot(&self, p: &Position) -> bool {
        p.0 > *self.x.end() || p.1 < *self.y.start()
    }
}

struct Velocity(isize, isize);
struct Position(isize, isize);

fn parse_range(s: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(ni32, tag(".."), ni32)(s)
}

fn parse_input(s: &str) -> Result<Target> {
    let (_, ((x1, x2), (y1, y2))) = preceded(
        tag("target area: "),
        separated_pair(
            preceded(tag("x="), parse_range),
            tag(", "),
            preceded(tag("y="), parse_range),
        ),
    )(s)
    .map_err(|e| e.map(|e| (e.input.to_string(), e.code)))?;

    Ok(Target {
        x: RangeInclusive::new(x1 as isize, x2 as isize),
        y: RangeInclusive::new(y1 as isize, y2 as isize),
    })
}

pub fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let soln_a = solve_a()?;
    eprintln!("Part A elapsed {:?}", start.elapsed());
    println!("solution part A: {}", soln_a);

    let start = std::time::Instant::now();
    let soln_b = solve_b()?;
    eprintln!("Part B elapsed {:?}", start.elapsed());
    println!("solution part B: {}", soln_b);

    Ok(())
}

fn test_trajectory(t: &Target, mut v: Velocity) -> bool {
    let mut p = Position(0, 0);
    loop {
        p.0 += v.0;
        p.1 += v.1;

        if t.x.contains(&p.0) && t.y.contains(&p.1) {
            return true;
        }

        if t.overshot(&p) {
            return false;
        }

        if v.0 > 0 {
            v.0 -= 1;
        }

        v.1 -= 1;
    }
}

pub fn solve_a() -> Result<isize> {
    let target = parse_input(include_str!("../input"))?;
    let vy_max = -target.y.start() - 1;
    let max_height = vy_max * (vy_max + 1) / 2;

    Ok(max_height)
}

pub fn solve_b() -> Result<usize> {
    let target = parse_input(include_str!("../input"))?;
    let max_y = *target.y.start();
    let max_x = *target.x.end();
    let valid_y = max_y..-max_y;
    let valid_x = 0..=max_x;

    let mut cnt = 0;
    for vx in valid_x {
        for vy in valid_y.clone() {
            if test_trajectory(&target, Velocity(vx, vy)) {
                cnt += 1
            }
        }
    }

    Ok(cnt)
}
