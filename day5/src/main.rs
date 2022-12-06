#![feature(test)]

extern crate test;

type Stacks = [Vec<u8>; u8::MAX as usize];

struct Input {
    stacks: Stacks,
    moves: Vec<(u8, u8, u8)>,
}

fn perform_move(stacks: &mut Stacks, n: u8, from: u8, to: u8) {
    let from = &mut stacks[(from - 1) as usize];
    let n = n as usize;

    let mut to_move: Vec<_> = from.drain((from.len() - n)..).collect();
    stacks[(to - 1) as usize].append(&mut to_move);
}

fn extract_container(s: &str) -> Option<u8> {
    let s = s.as_bytes();
    if s[0] == b'[' && s[2] == b']' {
        Some(s[1])
    } else {
        None
    }
}

fn parse(s: &str) -> Input {
    let (part1, part2) = s
        .split_once("\n\n")
        .or_else(|| s.split_once("\r\n\r\n"))
        .unwrap();

    let mut stacks: Stacks = array_init::array_init(|_| Vec::<u8>::new());

    for line in part1.lines().rev().skip(1) {
        let mut line = line;
        let mut i = 0;
        while line.len() > 0 {
            let (container, rest_line) = line.split_at(3);
            if let Some(container) = extract_container(container) {
                stacks[i].push(container);
            }
            line = &rest_line[1..];
            i += 1;
        }
    }

    let moves: Vec<(u8, u8, u8)> = part2
        .lines()
        .map(|line| {
            let line = &line[("move ".len())..];
            let (count, line) = line.split_once(" ").unwrap();
            let count = count.parse::<u8>().unwrap();
            let line = &line[("from ".len()..)];
            let (from, line) = line.split_once(" ").unwrap();
            let from = from.parse::<u8>().unwrap();
            let to = &line["to ".len()..];
            let to = to.parse::<u8>().unwrap();
            (count, from, to)
        })
        .collect();

    Input { stacks, moves }
}

fn answer_part1(inputs: &Input) -> String {
    let mut stacks = inputs.stacks.clone();
    for &(n, from, to) in &inputs.moves {
        for _ in 0..n {
            perform_move(&mut stacks, 1, from, to);
        }
    }
    let mut s = String::with_capacity(stacks.len());
    for v in stacks.into_iter() {
        if let Some(&c) = v.last() {
            s.push(c as char);
        }
    }
    s
}

fn answer_part2(inputs: &Input) -> String {
    let mut stacks = inputs.stacks.clone();
    for &(n, from, to) in &inputs.moves {
        perform_move(&mut stacks, n, from, to);
    }
    let mut s = String::with_capacity(stacks.len());
    for v in stacks.into_iter() {
        if let Some(&c) = v.last() {
            s.push(c as char);
        }
    }
    s
}

fn main() {
    let inputs = parse(include_str!("inputs"));
    println!("Part 1 = {}", answer_part1(&inputs));
    println!("Part 2 = {}", answer_part2(&inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn _part1() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!("CMZ".to_string(), answer_part1(&inputs));
    }
    #[test]
    fn _part2() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!("MCD".to_string(), answer_part2(&inputs));
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
