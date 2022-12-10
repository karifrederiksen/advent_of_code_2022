#![feature(test)]

extern crate test;

#[derive(Clone, Copy)]
enum Instr {
    // two cycles
    Addx(i32),
    // one cycle
    NoOp,
}

type Input = Vec<Instr>;

fn parse(s: &str) -> Input {
    s.lines()
        .map(|s| {
            if s == "noop" {
                Instr::NoOp
            } else if let Some(("addx", n)) = s.split_once(" ") {
                let n = n.parse::<i32>().unwrap();
                Instr::Addx(n)
            } else {
                panic!("unexpected input");
            }
        })
        .collect::<Vec<_>>()
}

fn answer_part1(inputs: &Input) -> i32 {
    let mut pos: i32 = 1;
    let mut out: i32 = 0;
    let mut cycle: i32 = 1;

    for instr in inputs.iter().copied() {
        if cycle % 40 == 20 {
            out += pos * cycle;
        }
        match instr {
            Instr::NoOp => {
                cycle += 1;
            }
            Instr::Addx(n) => {
                if cycle % 40 == 19 {
                    out += pos * (cycle + 1);
                }
                pos += n;
                cycle += 2;
            }
        }
    }
    out
}

fn char_for_cycle(pos: i32, cycle: i32) -> char {
    let pos_1 = (cycle - 2) % 40;
    let pos_2 = (cycle - 1) % 40;
    let pos_3 = cycle % 40;
    if pos == pos_1 || pos == pos_2 || pos == pos_3 {
        '#'
    } else {
        '.'
    }
}

fn answer_part2(inputs: &Input) -> String {
    let mut pos: i32 = 1;
    let mut cycle: i32 = 1;
    let mut buf = String::with_capacity(inputs.len() * 2);

    for instr in inputs.iter().copied() {
        match instr {
            Instr::NoOp => {
                buf.push(char_for_cycle(pos, cycle));
                cycle += 1;
            }
            Instr::Addx(n) => {
                buf.push(char_for_cycle(pos, cycle));
                buf.push(char_for_cycle(pos, cycle + 1));
                pos += n;
                cycle += 2;
            }
        }
    }
    buf
}

fn main() {
    let inputs = parse(include_str!("inputs"));
    println!("Part 1 = {}", answer_part1(&inputs));
    let answer_2 = answer_part2(&inputs);
    println!("Part 2 = ");
    for (idx, c) in answer_2.chars().into_iter().enumerate() {
        print!("{}", c);
        if idx % 40 == 39 {
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn _part1() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(13140, answer_part1(&inputs));
    }
    #[test]
    fn _part2() {
        use assert_str::*;
        let eoutput_example = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        let expected_output = eoutput_example
            .lines()
            .flat_map(|x| x.chars())
            .collect::<String>();
        let output = answer_part2(&parse(EXAMPLE_INPUT));
        assert_str_trim_eq!(expected_output, output);
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
