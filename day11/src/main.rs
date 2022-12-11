#![feature(test)]
#![feature(iter_intersperse)]
extern crate nom;
extern crate test;
use nom::{
    branch as B,
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator as C,
    multi::{self as M},
    sequence::{self as S},
    IResult,
};
type N = u128;

#[derive(Clone, Copy, Debug)]
enum Op {
    N(N),
    Old,
    Mul,
    Add,
}

fn eval_op(ops: &[Op], old: N) -> N {
    let mut n: N = match ops[0] {
        Op::N(n) => n,
        Op::Old => old,
        _ => panic!("unexpected initial op"),
    };
    let mut i = 1;
    while i < ops.len() {
        let n2 = match ops[i + 1] {
            Op::N(n) => n,
            Op::Old => old,
            _ => panic!("unexpected op"),
        };
        n = match ops[i] {
            Op::Mul => n * n2,
            Op::Add => n + n2,
            _ => panic!("unexpected number"),
        };
        i += 2;
    }
    n
}

#[derive(Debug, Clone)]
struct Monkey {
    id: u8,
    starting_items: Vec<N>,
    ops: Vec<Op>,
    test_divisible: N,
    if_true: u8,
    if_false: u8,
    inspection_count: u64,
}

impl Monkey {
    fn run_1(&mut self, mails: &mut Vec<(u8, N)>) {
        self.inspection_count += self.starting_items.len() as u64;
        for old in self.starting_items.drain(0..) {
            let new = eval_op(&self.ops, old) / 3;
            let next_id = if new % self.test_divisible == 0 {
                self.if_true
            } else {
                self.if_false
            };
            mails.push((next_id, new))
        }
    }
    fn run_2(&mut self, mails: &mut Vec<(u8, N)>, common_divisor: N) {
        self.inspection_count += self.starting_items.len() as u64;
        for old in self.starting_items.drain(0..) {
            let new = eval_op(&self.ops, old) % common_divisor;
            let next_id = if new % self.test_divisible == 0 {
                self.if_true
            } else {
                self.if_false
            };
            mails.push((next_id, new))
        }
    }
}

type Input = Vec<Monkey>;

fn parse_u8(s: &str) -> IResult<&str, u8> {
    C::map_res(digit1, |n: &str| n.parse::<u8>())(s)
}
fn parse_n(s: &str) -> IResult<&str, N> {
    C::map_res(digit1, |n: &str| n.parse::<N>())(s)
}

fn parse_op(s: &str) -> IResult<&str, Op> {
    B::alt((
        C::map(tag("old"), |_| Op::Old),
        C::map(parse_n, Op::N),
        C::map(tag("*"), |_| Op::Mul),
        C::map(tag("+"), |_| Op::Add),
    ))(s)
}

fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
    let (s, id) = S::delimited(tag("Monkey "), parse_u8, S::pair(tag(":"), newline))(s)?;

    let (s, starting_items) = S::delimited(
        tag("  Starting items: "),
        M::separated_list1(tag(", "), parse_n),
        newline,
    )(s)?;

    let (s, ops) = S::delimited(
        tag("  Operation: new = "),
        M::separated_list1(space1, parse_op),
        newline,
    )(s)?;

    let (s, test_divisible) = S::delimited(tag("  Test: divisible by "), parse_n, newline)(s)?;

    let (s, if_true) = S::delimited(tag("    If true: throw to monkey "), parse_u8, newline)(s)?;

    let (s, if_false) = S::preceded(tag("    If false: throw to monkey "), parse_u8)(s)?;

    Ok((
        s,
        Monkey {
            id,
            starting_items,
            ops,
            test_divisible,
            if_true,
            if_false,
            inspection_count: 0,
        },
    ))
}

fn parse(s: &str) -> Input {
    let mut monkeys = match M::separated_list1(S::pair(newline, newline), parse_monkey)(s) {
        Ok((_, x)) => x,
        Err(e) => {
            panic!("parsing failed: {}", e)
        }
    };
    monkeys.sort_by(|a, b| a.id.cmp(&b.id));
    monkeys
}

fn answer_part1(inputs: &Input) -> u64 {
    let mut mails: Vec<(u8, N)> = Vec::new();
    let mut monkeys = inputs.clone();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            monkeys[i].run_1(&mut mails);
            for (id, n) in mails.drain(0..) {
                monkeys[id as usize].starting_items.push(n);
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    monkeys[..2]
        .into_iter()
        .map(|m| m.inspection_count)
        .fold::<u64, _>(1, |a, b| a * b)
}

fn answer_part2(inputs: &Input) -> u64 {
    let mut mails: Vec<(u8, N)> = Vec::new();
    let mut monkeys = inputs.clone();

    let common_divisor = monkeys
        .iter()
        .map(|x| x.test_divisible)
        .fold(1, |a, b| a * b);

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            monkeys[i].run_2(&mut mails, common_divisor);
            for (id, n) in mails.drain(0..) {
                monkeys[id as usize].starting_items.push(n);
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    monkeys[..2]
        .into_iter()
        .map(|m| m.inspection_count)
        .fold::<u64, _>(1, |a, b| a * b)
}

fn main() {
    let inputs = parse(include_str!("inputs"));
    println!("Part 1 = {}", answer_part1(&inputs));
    println!("Part 2 = {}", answer_part2(&inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn _part1() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(10_605, answer_part1(&inputs));
    }

    #[test]
    fn _part2() {
        let inputs = parse(EXAMPLE_INPUT);
        let expect: u64 = 2_713_310_158;
        assert_eq!(expect, answer_part2(&inputs));
    }

    #[bench]
    fn bench_parse(b: &mut test::Bencher) {
        let input = include_str!("inputs");
        b.iter(|| {
            test::black_box(parse(input));
        });
    }

    #[bench]
    fn bench_answer_part1(b: &mut test::Bencher) {
        let inputs = parse(include_str!("inputs"));
        b.iter(|| {
            test::black_box(answer_part1(&inputs));
        });
    }

    #[bench]
    fn bench_answer_part2(b: &mut test::Bencher) {
        let inputs = parse(include_str!("inputs"));
        b.iter(|| {
            test::black_box(answer_part2(&inputs));
        });
    }
}
