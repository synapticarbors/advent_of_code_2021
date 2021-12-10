use anyhow::Result;

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

enum Nav {
    Complete,
    Incomplete(Vec<char>),
    SyntaxErr(char),
}

fn check_syntax(s: &str) -> Nav {
    let mut brackets = vec![];

    for c in s.chars() {
        match c {
            '[' => brackets.push(']'),
            '{' => brackets.push('}'),
            '(' => brackets.push(')'),
            '<' => brackets.push('>'),
            ']' | '}' | ')' | '>' => {
                if Some(c) != brackets.pop() {
                    return Nav::SyntaxErr(c);
                }
            }
            _ => (),
        }
    }
    if brackets.is_empty() {
        Nav::Complete
    } else {
        Nav::Incomplete(brackets.into_iter().rev().collect())
    }
}

fn score_completion(x: &[char]) -> u64 {
    x.iter().fold(0u64, |acc, c| {
        (5 * acc)
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            }
    })
}

pub fn solve_a() -> Result<u64> {
    let score = include_str!("../input")
        .lines()
        .map(|line| {
            if let Nav::SyntaxErr(c) = check_syntax(line.trim()) {
                match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!(),
                }
            } else {
                0
            }
        })
        .sum::<u64>();

    Ok(score)
}

pub fn solve_b() -> Result<u64> {
    let mut scores = include_str!("../input")
        .lines()
        .map(|line| match check_syntax(line.trim()) {
            Nav::Incomplete(r) => score_completion(&r),
            _ => 0,
        })
        .filter(|&x| x > 0)
        .collect::<Vec<_>>();

    scores.sort_unstable();
    let score = scores[scores.len() / 2];

    Ok(score)
}
