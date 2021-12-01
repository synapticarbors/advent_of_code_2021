const NRUNS: usize = 50;

fn timeit(func: fn() -> ()) -> std::time::Duration {
    let start = std::time::Instant::now();
    (func)();
    start.elapsed()
}

fn main() {
    let times: Vec<_> = benchmark::solvers()
        .iter()
        .map(|s| (s.name, (0..NRUNS).map(|_| timeit(s.func)).min().unwrap()))
        .collect();

    times.iter().for_each(|t| println!("{}: {:?}", t.0, t.1));
    println!(
        "Total: {:?}",
        times.iter().map(|(_, t)| t).sum::<std::time::Duration>()
    );
}
