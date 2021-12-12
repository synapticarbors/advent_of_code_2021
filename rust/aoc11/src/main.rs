use std::collections::VecDeque;

use anyhow::Result;

type Grid = [u8; 100];

const N: usize = 10;
const NX: isize = 10;
const N2: u64 = (N * N) as u64;

const DELTAS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (1, 1),
    (1, -1),
    (-1, 0),
    (-1, 1),
    (-1, -1),
];

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

fn step(g: &mut Grid) -> u64 {
    let mut nflashes = 0;

    let mut visited = [false; 100];
    let mut to_check = VecDeque::new();

    g.iter_mut().enumerate().for_each(|(ix, e)| {
        *e += 1;
        if *e > 9 {
            visited[ix] = true;
            to_check.push_back(ix);
        }
    });

    while !to_check.is_empty() {
        if let Some(ix) = to_check.pop_front() {
            let (j, i) = ((ix / N) as isize, (ix % N) as isize);

            DELTAS
                .iter()
                .filter(|(di, dj)| (i + di >= 0) && (i + di < NX) && (j + dj >= 0) && (j + dj < NX))
                .map(|(di, dj)| (i + di, j + dj))
                .for_each(|(ni, nj)| {
                    let nix = ni as usize + N * nj as usize;
                    g[nix] += 1;
                    if (g[nix] > 9) && !visited[nix] {
                        visited[nix] = true;
                        to_check.push_back(nix);
                    }
                });
        }
    }

    g.iter_mut().for_each(|e| {
        if *e > 9 {
            nflashes += 1;
            *e = 0
        }
    });

    nflashes
}

fn parse_grid(s: &str) -> Grid {
    s.lines().enumerate().fold([0; 100], |mut acc, (j, line)| {
        for (i, c) in line.chars().enumerate() {
            acc[i + N * j] = c.to_digit(10).unwrap() as u8;
        }
        acc
    })
}

pub fn solve_a() -> Result<u64> {
    let mut g = parse_grid(include_str!("../input"));
    let flashes: u64 = (0..100).map(|_| step(&mut g)).sum();

    Ok(flashes)
}

pub fn solve_b() -> Result<u64> {
    let mut g = parse_grid(include_str!("../input"));

    let mut cnt = 1;
    while step(&mut g) != N2 {
        cnt += 1
    }

    Ok(cnt)
}
