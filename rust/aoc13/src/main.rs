use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32 as ni32, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

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

type Points = Vec<(i32, i32)>;
type Instructions = Vec<Instruction>;

#[derive(Debug)]
enum Instruction {
    Up(i32),
    Left(i32),
}

fn parse_points(s: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list1(newline, separated_pair(ni32, tag(","), ni32))(s)
}

fn parse_instructions(s: &str) -> IResult<&str, Instructions> {
    separated_list1(
        newline,
        map(
            separated_pair(
                preceded(tag("fold along "), alt((tag("x"), tag("y")))),
                tag("="),
                ni32,
            ),
            |(d, v)| match d {
                "y" => Instruction::Up(v),
                "x" => Instruction::Left(v),
                _ => unreachable!(),
            },
        ),
    )(s)
}

fn parse_input(s: &'static str) -> Result<(Points, Instructions)> {
    let (_, (points, instructions)) =
        separated_pair(parse_points, tag("\n\n"), parse_instructions)(s)?;

    Ok((points, instructions))
}

fn fold(p: &[(i32, i32)], i: &Instruction) -> Points {
    let mut np = match i {
        Instruction::Up(f) => p
            .iter()
            .map(|&(x, y)| if y > *f { (x, *f - (y - *f)) } else { (x, y) })
            .collect(),
        Instruction::Left(f) => p
            .iter()
            .map(|&(x, y)| if x > *f { (*f - (x - *f), y) } else { (x, y) })
            .collect::<Vec<_>>(),
    };
    np.sort_unstable();
    np.dedup();
    np
}

pub fn solve_a() -> Result<usize> {
    let (points, instructions) = parse_input(include_str!("../input"))?;
    let npoints = fold(&points, &instructions[0]).len();

    Ok(npoints)
}

pub fn solve_b() -> Result<String> {
    let (points, instructions) = parse_input(include_str!("../input"))?;

    let folded = instructions
        .iter()
        .fold(points, |acc, instr| fold(&acc, instr));

    let (xmax, ymax) = folded
        .iter()
        .fold((0, 0), |acc, &(x, y)| (acc.0.max(x), acc.1.max(y)));

    let nx = xmax as usize + 1;
    let ny = ymax as usize + 1;

    let pgrid = folded
        .iter()
        .fold(vec![vec![false; nx]; ny], |mut acc, &(x, y)| {
            acc[y as usize][x as usize] = true;
            acc
        })
        .iter()
        .fold(String::from("\n"), |mut acc, row| {
            for &e in row {
                acc.push(if e { '#' } else { ' ' });
            }
            acc.push('\n');
            acc
        });

    Ok(pgrid)
}
