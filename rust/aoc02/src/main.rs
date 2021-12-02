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

pub fn solve_a() -> Result<i64> {
    let (h, d) = include_str!("../input")
        .lines()
        .fold((0, 0), |mut acc, line| {
            if let Some((instr, n)) = line.split_once(' ') {
                let d = n.parse::<i64>().unwrap();
                match (instr, d) {
                    ("forward", d) => acc.0 += d,
                    ("down", d) => acc.1 += d,
                    ("up", d) => acc.1 -= d,
                    _ => unreachable!(),
                }
            }
            acc
        });

    Ok(h * d)
}

pub fn solve_b() -> Result<i64> {
    let (h, d, _) = include_str!("../input")
        .lines()
        .fold((0, 0, 0), |mut acc, line| {
            if let Some((instr, n)) = line.split_once(' ') {
                let d = n.parse::<i64>().unwrap();
                match instr {
                    "forward" => {
                        acc.0 += d;
                        acc.1 += d * acc.2;
                    }
                    "down" => acc.2 += d,
                    "up" => acc.2 -= d,
                    _ => unreachable!(),
                }
            }
            acc
        });

    Ok(h * d)
}
