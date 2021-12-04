[Advent of Code 2021](https://adventofcode.com/2021)

After getting my 50 stars in [Advent of Code 2020](https://adventofcode.com/2020) using [Rust](https://www.rust-lang.org/), I'm back at it again in 2021.
I still consider myself somewhat of a rust novice, so this code will probably be non-idiomatic and generally non-optimal. I spend most of my days writing
Python, so this is just a fun exercise for the end of the year.

- [2021](https://github.com/synapticarbors/advent_of_code_2021)
- [2020](https://github.com/synapticarbors/advent_of_code_2020)

Last year I tried to avoid using external crates (although I did make use of [peg](https://crates.io/crates/peg) and [regex](https://crates.io/crates/regexh)
on a few occassions). This year I might try out a few more just to explore the ecosystem. 

Individual days can be run using:

```bash
$ cd rust/aoc01
$ cargo run --release
```

or to benchmark all of the solutions:

```bash
$ cd rust/benchmark
$ cargo run --release --bin bench
```
