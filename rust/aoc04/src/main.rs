use std::convert::TryInto;

use anyhow::{anyhow, Result};

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

const BOARD_SIZE: usize = 25;
const BOARD_DIM: usize = 5;

#[derive(Debug)]
struct Board {
    nums: [i32; BOARD_SIZE],
    row_marked: [i32; BOARD_DIM],
    col_marked: [i32; BOARD_DIM],
}

impl Board {
    fn from_lines(lines: &[&str]) -> Result<Board> {
        let nums: [i32; BOARD_SIZE] = lines
            .iter()
            .skip(1)
            .flat_map(|l| l.trim().split_whitespace().map(|x| x.parse::<i32>()))
            .collect::<Result<Vec<i32>, _>>()?
            .as_slice()
            .try_into()?;

        let row_marked = [0i32; BOARD_DIM];
        let col_marked = [0i32; BOARD_DIM];

        Ok(Board {
            nums,
            row_marked,
            col_marked,
        })
    }

    fn update(&mut self, n: i32) {
        for (ix, v) in self.nums.iter_mut().enumerate() {
            if *v != n {
                continue;
            }
            let i = ix / BOARD_DIM;
            let j = ix % BOARD_DIM;

            *v = -1;

            self.row_marked[i] += 1;
            self.col_marked[j] += 1;
        }
    }

    fn has_won(&self) -> bool {
        self.row_marked.contains(&5) | self.col_marked.contains(&5)
    }

    fn sum_unmarked(&self) -> i64 {
        self.nums
            .iter()
            .filter_map(|x| if *x != -1 { Some(*x as i64) } else { None })
            .sum::<i64>()
    }
}

fn build_game(input: &str) -> Result<(Vec<i32>, Vec<Board>)> {
    let mut iter = input.lines();
    let sequence = iter
        .next()
        .ok_or(anyhow!("failed"))?
        .split(',')
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;

    let board = iter
        .collect::<Vec<_>>()
        .chunks(6)
        .fold(vec![], |mut acc, lines| {
            acc.push(Board::from_lines(lines).unwrap());
            acc
        });

    Ok((sequence, board))
}

pub fn solve_a() -> Result<i64> {
    let input = include_str!("../input");
    let (sequence, mut boards) = build_game(input)?;

    for num in sequence.iter() {
        for board in boards.iter_mut() {
            board.update(*num);
            if board.has_won() {
                return Ok(board.sum_unmarked() * (*num as i64));
            }
        }
    }

    Ok(0)
}

pub fn solve_b() -> Result<i64> {
    let input = include_str!("../input");
    let (sequence, mut boards) = build_game(input)?;

    let mut have_won = vec![];
    let mut winning_num = vec![];

    for num in sequence.iter() {
        for (bi, board) in boards.iter_mut().enumerate() {
            if have_won.contains(&bi) {
                continue;
            }
            board.update(*num);
            if board.has_won() {
                have_won.push(bi);
                winning_num.push(num);
            }
        }
    }

    let lw_ix = have_won.pop().unwrap();
    let lw_num = winning_num.pop().unwrap();
    Ok(boards[lw_ix].sum_unmarked() * (*lw_num as i64))
}
