use anyhow::Result;
use std::collections::HashMap;

type Template = HashMap<(u8, u8), usize>;
type Rules = HashMap<(u8, u8), u8>;

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

fn parse_input(s: &str) -> Result<(Template, Rules)> {
    let (tmpl, rules) = s
        .split_once("\n\n")
        .ok_or(anyhow::anyhow!("malformed input"))?;

    let mut template = Template::new();
    for w in tmpl.as_bytes().windows(2) {
        *template.entry((w[0], w[1])).or_default() += 1
    }

    let rules: Rules = rules
        .lines()
        .map(|line| {
            let x = line.as_bytes();
            ((x[0], x[1]), x[x.len() - 1])
        })
        .collect();

    Ok((template, rules))
}

fn step(template: &Template, rules: &Rules) -> Template {
    let mut tnew = Template::new();

    for (k, cnt) in template {
        if let Some(&c) = rules.get(k) {
            let &(a, b) = k;
            *tnew.entry((a, c)).or_default() += cnt;
            *tnew.entry((c, b)).or_default() += cnt;
            //
        } else {
            *tnew.entry(*k).or_default() += cnt;
        }
    }

    tnew
}

fn find_element_count_spread(chain: &Template) -> usize {
    let cnts: HashMap<u8, usize> = chain.iter().fold(HashMap::new(), |mut acc, ((a, b), n)| {
        *acc.entry(*a).or_default() += n;
        *acc.entry(*b).or_default() += n;
        acc
    });

    ((cnts.values().max().unwrap() - cnts.values().min().unwrap()) as f64 / 2.0).ceil() as usize
}

pub fn solve_a() -> Result<usize> {
    let (template, rules) = parse_input(include_str!("../input"))?;
    let chain = (0..10).fold(template, |acc, _| step(&acc, &rules));

    Ok(find_element_count_spread(&chain))
}

pub fn solve_b() -> Result<usize> {
    let (template, rules) = parse_input(include_str!("../input"))?;
    let chain = (0..40).fold(template, |acc, _| step(&acc, &rules));

    Ok(find_element_count_spread(&chain))
}
