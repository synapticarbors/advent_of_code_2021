use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32 as ni32, newline},
    combinator::map,
    multi::{count, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, Clone)]
struct Cuboid {
    is_on: bool,
    x1: isize,
    x2: isize,
    y1: isize,
    y2: isize,
    z1: isize,
    z2: isize,
    removed: Vec<Cuboid>,
}

impl Cuboid {
    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        if self.x2 < other.x1
            || self.y2 < other.y1
            || self.z2 < other.z1
            || self.x1 > other.x2
            || self.y1 > other.y2
            || self.z1 > other.z2
        {
            return None;
        }

        Some(Cuboid {
            is_on: self.is_on,
            x1: self.x1.max(other.x1),
            x2: self.x2.min(other.x2),
            y1: self.y1.max(other.y1),
            y2: self.y2.min(other.y2),
            z1: self.z1.max(other.z1),
            z2: self.z2.min(other.z2),
            removed: vec![],
        })
    }

    fn volume(&self) -> isize {
        (self.x2 - self.x1 + 1) * (self.y2 - self.y1 + 1) * (self.z2 - self.z1 + 1)
            - self.removed.iter().map(|c| c.volume()).sum::<isize>()
    }

    fn subtract(&mut self, other: &Cuboid) {
        if let Some(other) = self.intersection(other) {
            for r in &mut self.removed {
                r.subtract(&other);
            }
            self.removed.push(other);
        }
    }
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

type IRange = (i32, i32);

fn parse_range(s: &str) -> IResult<&str, IRange> {
    separated_pair(ni32, tag(".."), ni32)(s)
}

fn parse_ranges(s: &str) -> IResult<&str, (IRange, IRange, IRange)> {
    map(
        count(
            preceded(
                alt((tag(","), tag(""))),
                preceded(alt((tag("x="), tag("y="), tag("z="))), parse_range),
            ),
            3usize,
        ),
        |v| (v[0], v[1], v[2]),
    )(s)
}

fn parse_instruction(s: &str) -> IResult<&str, Cuboid> {
    let (s, (switch, (r1, r2, r3))) =
        separated_pair(alt((tag("on"), tag("off"))), tag(" "), parse_ranges)(s)?;

    let is_on = switch == "on";

    let x = Cuboid {
        is_on,
        x1: r1.0 as isize,
        x2: r1.1 as isize,
        y1: r2.0 as isize,
        y2: r2.1 as isize,
        z1: r3.0 as isize,
        z2: r3.1 as isize,
        removed: vec![],
    };

    Ok((s, x))
}

fn parse_input(s: &str) -> Result<Vec<Cuboid>> {
    let (_, x) = separated_list1(newline, parse_instruction)(s)
        .map_err(|e| e.map(|e| (e.input.to_string(), e.code)))?;

    Ok(x)
}

pub fn solve_a() -> Result<isize> {
    let reboot_steps = parse_input(include_str!("../input"))?;
    let (dmin, dmax) = (-50, 50);

    let mut cuboid_union: Vec<Cuboid> = vec![];
    for rs in reboot_steps {
        if rs.x1 < dmin
            || rs.x2 > dmax
            || rs.y1 < dmin
            || rs.y2 > dmax
            || rs.z1 < dmin
            || rs.z2 > dmax
        {
            continue;
        }
        for x in &mut cuboid_union {
            x.subtract(&rs);
        }
        if rs.is_on {
            cuboid_union.push(rs.clone());
        }
    }

    let x = cuboid_union.iter().map(|c| c.volume()).sum();

    Ok(x)
}

pub fn solve_b() -> Result<isize> {
    let reboot_steps = parse_input(include_str!("../input"))?;

    let mut cuboid_union: Vec<Cuboid> = vec![];
    for rs in reboot_steps {
        for x in &mut cuboid_union {
            x.subtract(&rs);
        }
        if rs.is_on {
            cuboid_union.push(rs.clone());
        }
    }

    let x = cuboid_union.iter().map(|c| c.volume()).sum();

    Ok(x)
}
