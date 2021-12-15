use anyhow::Result;
use pathfinding::directed::dijkstra;

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

const NEIGHBORS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn get_neighbors(x: i32, y: i32, cm: &[Vec<u8>], nrows: i32, ncols: i32) -> Vec<((i32, i32), u32)> {
    NEIGHBORS
        .iter()
        .filter(|&(dx, dy)| (x + dx) >= 0 && (y + dy) >= 0 && (x + dx) < ncols && (y + dy) < nrows)
        .map(|&(dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            ((nx, ny), cm[ny as usize][nx as usize] as u32)
        })
        .collect()
}

fn get_neighbors2(
    x: i32,
    y: i32,
    cm: &[Vec<u8>],
    nrows: i32,
    ncols: i32,
) -> Vec<((i32, i32), u32)> {
    let (cmncols, cmnrows) = (cm[0].len() as i32, cm.len() as i32);
    NEIGHBORS
        .iter()
        .filter(|&(dx, dy)| (x + dx) >= 0 && (y + dy) >= 0 && (x + dx) < ncols && (y + dy) < nrows)
        .map(|&(dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            let (nxx, nyy) = (nx % cmncols, ny % cmnrows);
            let cost = (cm[nyy as usize][nxx as usize] as u32
                + (nx / cmncols) as u32
                + (ny / cmnrows) as u32
                - 1)
                % 9
                + 1;
            ((nx, ny), cost)
        })
        .collect()
}

pub fn solve_a() -> Result<u32> {
    let grid: Vec<Vec<u8>> = include_str!("../input")
        .lines()
        .fold(Vec::new(), |mut acc, line| {
            acc.push(line.bytes().map(|c| c - b'0').collect::<Vec<_>>());
            acc
        });

    let (ncols, nrows) = (grid[0].len(), grid.len());
    let target = (ncols as i32 - 1, nrows as i32 - 1);
    let (ncols, nrows) = (ncols as i32, nrows as i32);

    let (_, cost) = dijkstra::dijkstra(
        &(0, 0),
        |&(x, y)| get_neighbors(x, y, &grid, nrows, ncols),
        |&p| p == target,
    )
    .unwrap();

    Ok(cost)
}

pub fn solve_b() -> Result<u32> {
    let grid: Vec<Vec<u8>> = include_str!("../input")
        .lines()
        .fold(Vec::new(), |mut acc, line| {
            acc.push(line.bytes().map(|c| c - b'0').collect::<Vec<_>>());
            acc
        });

    let (ncols, nrows) = (5 * grid[0].len(), 5 * grid.len());
    let target = (ncols as i32 - 1, nrows as i32 - 1);
    let (ncols, nrows) = (ncols as i32, nrows as i32);

    let (_, cost) = dijkstra::dijkstra(
        &(0, 0),
        |&(x, y)| get_neighbors2(x, y, &grid, nrows, ncols),
        |&p| p == target,
    )
    .unwrap();

    Ok(cost)
}
