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

fn parse(s: &str) -> Result<(usize, Vec<u8>)> {
    let g = s
        .lines()
        .fold((0usize, vec![]), |(mut nrows, mut acc), line| {
            for c in line.chars() {
                acc.push(match c {
                    'v' => 1,
                    '>' => 2,
                    '.' => 0,
                    _ => unreachable!(),
                });
            }
            nrows += 1;
            (nrows, acc)
        });

    Ok(g)
}

fn step(g: &mut [u8], gnext: &mut [u8], ncols: usize, nrows: usize) -> bool {
    let mut has_changed = false;
    assert_eq!(g.len(), gnext.len());
    gnext.copy_from_slice(g);
    for (i, v) in g.iter().enumerate() {
        if *v != 2 {
            continue;
        }

        let (x, y) = (i % ncols, i / ncols);
        let inext = (x + 1) % ncols + y * ncols;
        let nv = g[inext];
        if nv == 0 {
            gnext.swap(i, inext);
            has_changed = true;
        }
    }

    g.copy_from_slice(gnext);
    for (i, v) in g.iter().enumerate() {
        if *v != 1 {
            continue;
        }

        let (x, y) = (i % ncols, i / ncols);
        let inext = x + ((y + 1) * ncols) % (nrows * ncols);
        let nv = g[inext];
        if nv == 0 {
            gnext.swap(i, inext);
            has_changed = true;
        }
    }

    has_changed
}

#[allow(dead_code)]
fn draw(x: &[u8], ncols: usize) {
    let s: String = x
        .iter()
        .map(|a| match a {
            0 => '.',
            1 => 'v',
            2 => '>',
            _ => unreachable!(),
        })
        .collect();

    println!(
        "{}",
        s.chars()
            .collect::<Vec<char>>()
            .chunks(ncols)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

pub fn solve_a() -> Result<usize> {
    let (nrows, mut g) = parse(include_str!("../input"))?;
    let ncols = g.len() / nrows;
    let mut gnext = vec![0; g.len()];

    //draw(&g, ncols);

    let mut cnt = 0;
    let x = loop {
        cnt += 1;
        if !step(&mut g, &mut gnext, ncols, nrows) {
            break cnt;
        }

        std::mem::swap(&mut g, &mut gnext);
        //println!("\n\n=========");
        //draw(&g, ncols);
    };

    Ok(x)
}

pub fn solve_b() -> Result<usize> {
    Ok(0)
}
