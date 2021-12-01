macro_rules! drop_result {
    ($e:expr) => {{
        fn x() {
            let _ = $e();
            ()
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
    ]
}
