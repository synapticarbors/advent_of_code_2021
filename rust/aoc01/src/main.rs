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

pub fn solve_a() -> Result<usize> {
    let x = include_str!("../input")
        .lines()
        .map(|l| l.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?
        .windows(2)
        .filter(|x| x[1] > x[0])
        .count();

    Ok(x)
}

pub fn solve_b() -> Result<usize> {
    let a = include_str!("../input")
        .lines()
        .map(|l| l.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    let x = a
        .windows(3)
        .zip(a[1..].windows(3))
        .filter(|g| g.1.iter().sum::<u32>() > g.0.iter().sum())
        .count();

    Ok(x)
}
