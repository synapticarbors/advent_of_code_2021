use anyhow::Result;

#[derive(Debug, Clone)]
enum Value {
    Number(i64),
    Variable(usize),
}

#[derive(Debug, Clone)]
enum Instruction {
    Inp(Value),
    Add(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
    Mod(Value, Value),
    Eql(Value, Value),
}

#[derive(Debug, Default)]
struct Program {
    data: [i64; 4],
    input: i64,
}

fn nth_digit(x: u64, n: u64) -> i64 {
    ((x / 10u64.pow(n as u32)) % 10) as i64
}

impl Program {
    fn extract_value(&self, x: Value) -> i64 {
        match x {
            Value::Variable(a) => self.data[a],
            Value::Number(a) => a,
        }
    }

    fn evaluate(&mut self, instructions: Vec<Instruction>) {
        instructions.into_iter().for_each(|i| {
            self.eval_instruction(i);
        });
    }

    fn eval_instruction(&mut self, inst: Instruction) {
        match inst {
            Instruction::Inp(Value::Variable(a)) => self.data[a] = self.input,
            Instruction::Add(Value::Variable(a), b) => self.data[a] += self.extract_value(b),
            Instruction::Mul(Value::Variable(a), b) => self.data[a] *= self.extract_value(b),
            Instruction::Div(Value::Variable(a), b) => self.data[a] /= self.extract_value(b),
            Instruction::Mod(Value::Variable(a), b) => self.data[a] %= self.extract_value(b),
            Instruction::Eql(Value::Variable(a), b) => {
                let x = self.data[a];
                let y = self.extract_value(b);
                self.data[a] = if x == y { 1 } else { 0 };
            }

            _ => (),
        }
    }
}

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

fn convert(s: &str) -> Value {
    match s {
        "w" => Value::Variable(0),
        "x" => Value::Variable(1),
        "y" => Value::Variable(2),
        "z" => Value::Variable(3),
        _ => Value::Number(s.parse::<i64>().unwrap()),
    }
}

fn parse(s: &str) -> Result<Vec<Instruction>> {
    let x = s
        .lines()
        .map(|line| {
            let p = line.split_whitespace().collect::<Vec<&str>>();
            match p[0] {
                "inp" => Instruction::Inp(convert(p[1])),
                "add" => Instruction::Add(convert(p[1]), convert(p[2])),
                "mul" => Instruction::Mul(convert(p[1]), convert(p[2])),
                "div" => Instruction::Div(convert(p[1]), convert(p[2])),
                "mod" => Instruction::Mod(convert(p[1]), convert(p[2])),
                "eql" => Instruction::Eql(convert(p[1]), convert(p[2])),
                _ => unreachable!(),
            }
        })
        .collect();
    Ok(x)
}

fn get_alu_modules(s: &str) -> Result<Vec<Vec<Instruction>>> {
    let m = parse(s)?
        .into_iter()
        .fold(Vec::<Vec<Instruction>>::with_capacity(14), |mut acc, x| {
            match x {
                Instruction::Inp(_) => {
                    acc.push(vec![x]);
                }
                _ => acc.last_mut().unwrap().push(x),
            }
            acc
        });

    Ok(m)
}

pub fn solve_a() -> Result<u64> {
    // Solution by-hand, but validate
    let soln = 99911993949684;

    let modules = get_alu_modules(include_str!("../input"))?;

    let mut prog = Program::default();
    for i in (0..=13).rev() {
        let m = modules[13 - i].clone();
        prog.input = nth_digit(soln, i as u64);

        prog.evaluate(m);
    }

    assert_eq!(0, prog.data[3]);

    Ok(soln)
}

pub fn solve_b() -> Result<u64> {
    // Solution by-hand, but validate
    let soln = 62911941716111;
    let modules = get_alu_modules(include_str!("../input"))?;

    let mut prog = Program::default();
    for i in (0..=13).rev() {
        let m = modules[13 - i].clone();
        prog.input = nth_digit(soln, i as u64);

        prog.evaluate(m);
    }

    assert_eq!(0, prog.data[3]);

    Ok(soln)
}

/*
Solve by hand with suggestions from r/adventcode
(CHECK, OFFSET) from rows 5 and 15 of each alu module
(12, 6)
(10, 6)
(13, 3)
(-11, 11)
(13, 9)
(-1, 3)
(10, 13)
(11, 6)
(0, 14)
(10, 10)
(-5, 12)
(-16, 10)
(-7, 11)
(-11, 15)

CHECK > 0 -> PUSH input + offset
CHECK <= 0 -> POP + CHECK

PUSH input[0] + 6
PUSH input[1] + 6
PUSH input[2] + 3
POP: input[3] == POP - 11
PUSH: input[4] + 9
POP: input[5] == POP - 1
PUSH input[6] + 13
PUSH input[7] + 6
POP: input[8] == POP
PUSH input[9] + 10
POP: input[10] == POP - 5
POP: input[11] == POP - 16
POP: input[12] == POP - 7
POP: input[13] == POP - 11


input[3] == input[2] - 8
input[5] == input[4] + 8
input[8] == input[7] + 6
input[10] == input[9] + 5
input[11] = input[6] - 3
input[12] == input[1] - 1
input[13] == input[0] - 5

MAX:
0:  9
1:  9
2:  9
3:  1
4:  1
5:  9
6:  9
7:  3
8:  9
9:  4
10: 9
11: 6
12: 8
13: 4

99911993949684

MIN:
0:  6
1:  2
2:  9
3:  1
4:  1
5:  9
6:  4
7:  1
8:  7
9:  1
10: 6
11: 1
12: 1
13: 1

62911941716111

*/
