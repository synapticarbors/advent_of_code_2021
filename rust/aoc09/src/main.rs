use std::collections::{HashMap, HashSet};

use anyhow::Result;
use ndarray::{Array, Array2};

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

const NEIGHBORS: [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

const fn add(lhs: usize, rhs: isize) -> Option<usize> {
    if rhs.is_negative() {
        lhs.checked_sub(rhs.wrapping_abs() as usize)
    } else {
        lhs.checked_add(rhs as usize)
    }
}

pub fn solve_a() -> Result<u64> {
    let (nrows, grid) =
        include_str!("../input")
            .lines()
            .fold((0usize, vec![]), |(mut n, mut acc), line| {
                acc.extend(line.chars().map(|c| c.to_digit(10).unwrap()));
                n += 1;
                (n, acc)
            });

    let ncols = grid.len() / nrows;
    let grid = Array::from_shape_vec((nrows, ncols), grid)?;

    let sum = grid.indexed_iter().fold(0u64, |mut acc, ((i, j), v)| {
        if NEIGHBORS.iter().all(|&(di, dj)| {
            let (ni, nj) = (
                add(i, di).unwrap_or(usize::MAX),
                add(j, dj).unwrap_or(usize::MAX),
            );
            v < grid.get((ni, nj)).unwrap_or(&u32::MAX)
        }) {
            acc += *v as u64 + 1;
        }
        acc
    });

    Ok(sum)
}

type Graph = HashMap<(usize, usize), HashSet<(usize, usize)>>;

fn dfs(g: &Graph, tmp: &mut Vec<(usize, usize)>, i: usize, j: usize, visited: &mut Array2<bool>) {
    visited[(i, j)] = true;
    tmp.push((i, j));

    if let Some(neighbors) = g.get(&(i, j)) {
        for &(ni, nj) in neighbors {
            if !visited[(ni, nj)] {
                dfs(g, tmp, ni, nj, visited);
            }
        }
    }
}

pub fn solve_b() -> Result<usize> {
    let (nrows, grid) =
        include_str!("../input")
            .lines()
            .fold((0usize, vec![]), |(mut n, mut acc), line| {
                acc.extend(line.chars().map(|c| c.to_digit(10).unwrap()));
                n += 1;
                (n, acc)
            });

    let ncols = grid.len() / nrows;
    let grid = Array::from_shape_vec((nrows, ncols), grid)?;

    // Build the adjacency list
    let adj = grid.indexed_iter().fold(
        HashMap::with_capacity(grid.len()),
        |mut acc: Graph, ((i, j), v)| {
            if *v == 9 {
                return acc;
            }

            for &(di, dj) in NEIGHBORS.iter() {
                let (ni, nj) = (
                    add(i, di).unwrap_or(usize::MAX),
                    add(j, dj).unwrap_or(usize::MAX),
                );

                if let Some(nv) = grid.get((ni, nj)) {
                    if *nv != 9 {
                        acc.entry((i, j)).or_default().insert((ni, nj));
                    }
                }
            }

            acc
        },
    );

    // Now find the connected components
    let mut visited = Array::from_elem((nrows, ncols), false);
    let mut ccmp = vec![];

    for &(i, j) in adj.keys() {
        if !visited[(i, j)] {
            let mut tmp = vec![];
            dfs(&adj, &mut tmp, i, j, &mut visited);
            ccmp.push(tmp);
        }
    }

    ccmp.sort_unstable_by_key(|x| x.len());

    let x = ccmp.iter().rev().take(3).map(|c| c.len()).product();

    Ok(x)
}
