use anyhow::Result;
use ndarray::{s, Array2, ArrayView2, Zip};

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

#[inline]
fn win2ix(x: &ArrayView2<bool>) -> usize {
    x.iter().fold(0, |acc, &b| (acc << 1) ^ (b as usize))
}

fn enhance(
    img: &Array2<bool>,
    img_next: &mut Array2<bool>,
    algo: &[bool],
    n: usize,
    flip_inifinite: bool,
) {
    Zip::indexed(img.windows((3, 3))).for_each(|(i, j), window| {
        let (ni, nj) = (i + 1, j + 1);
        img_next[[ni, nj]] = algo[win2ix(&window)];
    });

    // Flip edge
    if flip_inifinite {
        let x = (n % 2) == 0;
        img_next.row_mut(0).fill(x);
        img_next.row_mut(img_next.nrows() - 1).fill(x);
        img_next.slice_mut(s![1..img_next.nrows() - 1, 0]).fill(x);
        img_next
            .slice_mut(s![1..img_next.nrows() - 1, img_next.ncols() - 1])
            .fill(x);
    }
}

#[allow(dead_code)]
fn draw(img: &Array2<bool>) {
    for row in img.map(|&x| if x { '#' } else { '.' }).rows() {
        println!("{}", row.to_vec().iter().collect::<String>());
    }
}

fn solve(input: &str, napply: usize) -> Result<usize> {
    let (algo, init_img) = input.split_once("\n\n").unwrap();
    let ncols = init_img.bytes().position(|x| x == b'\n').unwrap();
    let init_img = init_img
        .bytes()
        .filter(|x| *x != b'\n')
        .map(|x| x == b'#')
        .collect::<Vec<_>>();
    let nrows = init_img.len() / ncols;

    let shape = (nrows + (napply + 1) * 2, ncols + (napply + 1) * 2);

    let mut img = Array2::<bool>::from_elem(shape, false);
    img.slice_mut(s![
        napply + 1..napply + 1 + nrows,
        napply + 1..napply + 1 + ncols
    ])
    .assign(&Array2::from_shape_vec((nrows, ncols), init_img)?);

    let mut img_next = Array2::<bool>::from_elem(shape, false);

    let algo = algo.bytes().map(|x| x == b'#').collect::<Vec<_>>();

    let flip_inifinite = algo[0] && !algo[algo.len() - 1];

    for i in 0..napply {
        enhance(&img, &mut img_next, &algo, i, flip_inifinite);
        std::mem::swap(&mut img, &mut img_next);
    }

    let cnt = img.iter().filter(|&&x| x).count();
    Ok(cnt)
}

pub fn solve_a() -> Result<usize> {
    let input = include_str!("../input").trim();
    solve(input, 2)
}

pub fn solve_b() -> Result<usize> {
    let input = include_str!("../input").trim();
    solve(input, 50)
}
