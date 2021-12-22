use anyhow::Result;
use ndarray::Array5;

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

fn get_initial_positions(s: &str) -> Vec<usize> {
    s.lines()
        .map(|line| {
            line.split_once(":")
                .unwrap()
                .1
                .trim()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<_>>()
}

fn practice_turn(p: &mut usize, score: &mut usize, dice: &mut impl Iterator<Item = usize>) {
    let nspaces: usize = dice.take(3).sum();
    *p = (*p + nspaces - 1) % 10 + 1;
    *score += *p;
}

pub fn solve_a() -> Result<usize> {
    let mut pos = get_initial_positions(include_str!("../input"));
    let mut p1score = 0usize;
    let mut p2score = 0usize;
    let mut dice = (1..=100).cycle();
    let mut nrolls = 0usize;

    loop {
        practice_turn(&mut pos[0], &mut p1score, &mut dice);
        nrolls += 3;
        if p1score >= 1000 {
            return Ok(p2score * nrolls);
        }

        practice_turn(&mut pos[1], &mut p2score, &mut dice);
        nrolls += 3;
        if p2score >= 1000 {
            return Ok(p1score * nrolls);
        }
    }
}

const QMOVES: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

pub fn solve_b() -> Result<usize> {
    let init_pos = get_initial_positions(include_str!("../input"));
    let mut p1wins = 0;
    let mut p2wins = 0;

    let mut state = Array5::<usize>::zeros((21, 21, 11, 11, 2));
    unsafe { *state.uget_mut([0, 0, init_pos[0], init_pos[1], 0]) += 1 }

    for p1score in 0..21 {
        for p2score in 0..21 {
            for p1pos in 1..=10 {
                for p2pos in 1..=10 {
                    for pid in 0..=1 {
                        let cnt = unsafe { *state.uget([p1score, p2score, p1pos, p2pos, pid]) };
                        if cnt == 0 {
                            continue;
                        }

                        for (n, dcnt) in QMOVES {
                            let (p1posx, p2posx) = if pid == 0 {
                                ((p1pos + n - 1) % 10 + 1, p2pos)
                            } else {
                                (p1pos, (p2pos + n - 1) % 10 + 1)
                            };

                            let (p1scorex, p2scorex) = if pid == 0 {
                                (p1score + p1posx, p2score)
                            } else {
                                (p1score, p2score + p2posx)
                            };

                            let pidx = (pid + 1) % 2;

                            if p1scorex >= 21 {
                                p1wins += cnt * dcnt;
                            } else if p2scorex >= 21 {
                                p2wins += cnt * dcnt;
                            } else {
                                let x = unsafe {
                                    state.uget_mut([p1scorex, p2scorex, p1posx, p2posx, pidx])
                                };
                                *x += cnt * dcnt;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(p1wins.max(p2wins))
}
