use anyhow::Result;

pub fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let soln_a = solve_a()?;
    eprintln!("Part A elapsed {:?}", start.elapsed());
    println!("solution part A: {:?}", soln_a);

    let start = std::time::Instant::now();
    let soln_b = solve_b()?;
    eprintln!("Part B elapsed {:?}", start.elapsed());
    println!("solution part B: {:?}", soln_b);

    Ok(())
}

fn parse_initial_state(s: &str) -> [u64; 9] {
    s.split(',').fold([0u64; 9], |mut acc, x| {
        let v = x.parse::<usize>().unwrap();
        acc[v] += 1;
        acc
    })
}

pub fn solve_a() -> Result<u64> {
    let mut state = parse_initial_state(include_str!("../input"));

    for _ in 0..80 {
        state.rotate_left(1);
        state[6] += state[8];
    }

    let fcnt = state.iter().sum();

    Ok(fcnt)
}

pub fn solve_b() -> Result<u64> {
    let mut state = parse_initial_state(include_str!("../input"));

    for _ in 0..256 {
        state.rotate_left(1);
        state[6] += state[8];
    }

    let fcnt = state.iter().sum();

    Ok(fcnt)
}
