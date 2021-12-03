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

fn flipnum(mut n: u64) -> u64 {
    let mut x = 1;
    while x <= n {
        n ^= x;
        x <<= 1;
    }

    n
}

pub fn solve_a() -> Result<u64> {
    let input = include_str!("../input");
    let width = input.lines().next().unwrap().len();

    let dec = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2))
        .collect::<Result<Vec<_>, _>>()?;

    let nnums = dec.len();

    let gamma = dec
        .iter()
        .fold(vec![0; width], |mut acc, x| {
            for (i, e) in acc.iter_mut().enumerate().take(width) {
                if x >> i & 1 == 0 {
                    *e += 1;
                }
            }
            acc
        })
        .iter()
        .enumerate()
        .map(|(i, x)| ((*x < nnums / 2) as u64) << i)
        .sum::<u64>();

    let epsilon = flipnum(gamma);

    Ok(gamma * epsilon)
}

pub fn solve_b() -> Result<u64> {
    let input = include_str!("../input");
    let width = input.lines().next().unwrap().len();

    let dec = input
        .lines()
        .map(|line| u64::from_str_radix(line, 2))
        .collect::<Result<Vec<_>, _>>()?;

    let oxygen = (0..width).rev().fold(dec.clone(), |mut candidates, i| {
        let nnums = candidates.len();

        if nnums == 1 {
            return candidates;
        }

        let ones_cnt = candidates.iter().filter(|&x| x >> i & 1 == 1).count();
        let target = if ones_cnt >= (nnums - ones_cnt) { 1 } else { 0 };

        candidates.retain(|&x| x >> i & 1 == target);

        candidates
    });

    assert_eq!(oxygen.len(), 1);
    let oxygen = oxygen[0];

    let co2 = (0..width).rev().fold(dec, |mut candidates, i| {
        let nnums = candidates.len();

        if nnums == 1 {
            return candidates;
        }

        let ones_cnt = candidates.iter().filter(|&x| x >> i & 1 == 1).count();
        let target = if ones_cnt >= (nnums - ones_cnt) { 0 } else { 1 };

        candidates.retain(|&x| x >> i & 1 == target);

        candidates
    });

    assert_eq!(co2.len(), 1);
    let co2 = co2[0];

    Ok(oxygen * co2)
}
