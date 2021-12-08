use std::collections::HashSet;

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
    let c = include_str!("../input")
        .lines()
        .flat_map(|line| line.split_once('|').unwrap().1.split_whitespace())
        .filter(|x| matches!(x.len(), 2 | 3 | 4 | 7))
        .count();

    Ok(c)
}

// segments:
//    0000
//   1    5
//   1    5
//    3333
//   2    4
//   2    4
//    6666
//
//    Given 10 samples that represent each digit, there are the following counts that should appear
//    0 => 8
//    1 => 6
//    2 => 4
//    3 => 7
//    4 => 9
//    5 => 8
//    6 => 7
//
//    So we can map segments 1, 2, and 4 uniquely based on counts
//    segment 0 is the difference between the example of len 3 and 2
//    segment 5 is the segment with count 8 that is not segment 0
//    segment 3 is the segment that is in the sample of length 4 that isn't segment 1, 5 or 4
//    segment 6 is the remaining unassigned segment

pub fn solve_b() -> Result<u64> {
    let s = include_str!("../input")
        .lines()
        .map(|line| {
            let (patterns, digits) = line.split_once('|').unwrap();
            let mut patterns = patterns.split_whitespace().collect::<Vec<_>>();

            let mut segments = [0u8; 7];
            let segcnts = patterns.iter().fold([0u8; 7], |mut acc, pattern| {
                pattern
                    .chars()
                    .for_each(|c| acc[(c as u8 - b'a') as usize] += 1);
                acc
            });

            segments[1] = segcnts.iter().position(|&x| x == 6).unwrap() as u8 + b'a';
            segments[2] = segcnts.iter().position(|&x| x == 4).unwrap() as u8 + b'a';
            segments[4] = segcnts.iter().position(|&x| x == 9).unwrap() as u8 + b'a';

            patterns.sort_unstable_by_key(|x| x.len());

            let pl3: HashSet<_> = patterns[1].bytes().collect();
            let pl2: HashSet<_> = patterns[0].bytes().collect();
            segments[0] = *pl3.difference(&pl2).next().unwrap();

            let cnt8 = segcnts
                .iter()
                .enumerate()
                .filter(|(_, &x)| x == 8)
                .map(|(i, _)| i as u8 + b'a')
                .collect::<Vec<_>>();
            segments[5] = if cnt8[0] == segments[0] {
                cnt8[1]
            } else {
                cnt8[0]
            };

            segments[3] = patterns[2]
                .bytes()
                .find(|&x| (x != segments[1]) && (x != segments[4]) && (x != segments[5]))
                .unwrap();

            segments[6] = (b'a'..=b'g')
                .find(|&x| !segments[..5].contains(&x))
                .unwrap();

            digits
                .split_whitespace()
                .map(|d| match d.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    6 if !d.contains(segments[3] as char) => 0,
                    6 if !d.contains(segments[5] as char) => 6,
                    6 => 9,
                    5 if d.contains(segments[1] as char) => 5,
                    5 if d.contains(segments[4] as char) => 3,
                    5 => 2,
                    _ => unreachable!(),
                })
                .enumerate()
                .fold(0u64, |acc, (i, v)| acc + v * 10u64.pow(3 - i as u32))
        })
        .sum();

    Ok(s)
}
