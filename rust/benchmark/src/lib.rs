macro_rules! drop_result {
    ($e:expr) => {{
        fn x() {
            let _ = $e();
        }

        x
    }};
}

macro_rules! soln {
    ($x:expr, $y:expr) => {{
        Soln {
            func: drop_result!($x),
            name: $y,
        }
    }};
}

pub struct Soln {
    pub func: fn() -> (),
    pub name: &'static str,
}

pub fn solvers() -> &'static [Soln] {
    &[
        soln!(aoc01::solve_a, "aoc01a"),
        soln!(aoc01::solve_b, "aoc01b"),
        soln!(aoc02::solve_a, "aoc02a"),
        soln!(aoc02::solve_b, "aoc02b"),
        soln!(aoc03::solve_a, "aoc03a"),
        soln!(aoc03::solve_b, "aoc03b"),
        soln!(aoc04::solve_a, "aoc04a"),
        soln!(aoc04::solve_b, "aoc04b"),
        soln!(aoc05::solve_a, "aoc05a"),
        soln!(aoc05::solve_b, "aoc05b"),
        soln!(aoc06::solve_a, "aoc06a"),
        soln!(aoc06::solve_b, "aoc06b"),
        soln!(aoc07::solve_a, "aoc07a"),
        soln!(aoc07::solve_b, "aoc07b"),
        soln!(aoc08::solve_a, "aoc08a"),
        soln!(aoc08::solve_b, "aoc08b"),
    ]
}
