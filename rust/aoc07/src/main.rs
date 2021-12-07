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
    // solution is calculating the median of the starting positions
    let mut x = include_str!("../input")
        .split(',')
        .map(|e| e.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    x.sort_unstable();

    let n = x.len();

    let m = match n {
        n if n % 2 == 0 => (x[n / 2] + x[(n - 1) / 2]) / 2,
        _ => x[n / 2],
    };

    let c = x.iter().map(|e| (e - m).abs()).sum::<i64>();

    Ok(c)
}

pub fn solve_b() -> Result<i64> {
    // C(a) = (|x_i - a| + 1) * |x_i - a| / 2
    // dC(a)/da = sgn(a - x)/2 + a - x = 0
    // a = mean(x) - sgn(a - x) / (2*n)
    // = mean(x) - (a - x) / (2n *|a - x|)
    // = mean(x) +/- 0.5
    let x = include_str!("../input")
        .split(',')
        .map(|e| e.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let n = x.len();

    let m = (x.iter().sum::<i64>() as f64) / (n as f64);
    let mc = m.ceil() as i64;
    let mf = m.floor() as i64;

    let (c1, c2) = x.iter().fold((0, 0), |mut acc, e| {
        let dxc = (e - mc).abs();
        let dxf = (e - mf).abs();
        acc.0 += dxc * (dxc + 1) / 2;
        acc.1 += dxf * (dxf + 1) / 2;
        acc
    });

    let c = c1.min(c2);

    Ok(c)
}
