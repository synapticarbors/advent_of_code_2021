use anyhow::Result;
use nom::{bytes::complete::tag, character::complete::alpha1, sequence::separated_pair, IResult};
use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Node {
    Start,
    End,
    BigCave(usize),
    SmallCave(usize),
}

type Graph = HashMap<Node, HashSet<Node>>;

fn parse_line(s: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag("-"), alpha1)(s)
}

fn convert_cave<'a>(
    s: &'a str,
    curr_id: &mut usize,
    name2id: &mut HashMap<&'a str, usize>,
) -> Node {
    match s {
        "start" => Node::Start,
        "end" => Node::End,
        _ => {
            let is_big = s.chars().next().unwrap().is_uppercase();

            let id = if let Some(id) = name2id.get(s) {
                *id
            } else {
                *curr_id <<= 1;
                name2id.insert(s, *curr_id);
                *curr_id
            };

            if is_big {
                Node::BigCave(id)
            } else {
                Node::SmallCave(id)
            }
        }
    }
}

fn build_graph(s: &str) -> Graph {
    let mut name2id = HashMap::new();

    let (_, g) = s.lines().fold(
        (1, HashMap::new()),
        |(mut cid, mut acc): (usize, Graph), line| {
            let (_, (n1s, n2s)) = parse_line(line).unwrap();

            let cva = convert_cave(n1s, &mut cid, &mut name2id);
            let cvb = convert_cave(n2s, &mut cid, &mut name2id);

            acc.entry(cva.clone()).or_default().insert(cvb.clone());
            acc.entry(cvb).or_default().insert(cva);

            (cid, acc)
        },
    );

    g
}

fn dfs(g: &Graph, c: &Node, visited: usize, has_revisited: bool) -> usize {
    let mut npaths = 0;

    if let Some(neighbors) = g.get(c) {
        for nc in neighbors {
            match nc {
                Node::End => npaths += 1,
                Node::Start => (),
                Node::SmallCave(id) => {
                    if visited & id != *id {
                        let subv = visited | id;
                        let nsubpaths = dfs(g, nc, subv, has_revisited);
                        npaths += nsubpaths;
                    } else if !has_revisited {
                        let subv = visited | id;
                        let nsubpaths = dfs(g, nc, subv, true);
                        npaths += nsubpaths;
                    }
                }
                Node::BigCave(_) => {
                    let nsubpaths = dfs(g, nc, visited, has_revisited);
                    npaths += nsubpaths;
                }
            }
        }
    }

    npaths
}

pub fn solve_a() -> Result<usize> {
    let g = build_graph(include_str!("../input"));
    let npaths = dfs(&g, &Node::Start, 0, true);

    Ok(npaths)
}

pub fn solve_b() -> Result<usize> {
    let g = build_graph(include_str!("../input"));

    let npaths = dfs(&g, &Node::Start, 0, false);

    Ok(npaths)
}
