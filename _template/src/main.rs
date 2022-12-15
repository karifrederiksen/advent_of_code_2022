#![feature(test)]
extern crate nom;
extern crate test;
use nom::{
    branch as B,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator as C,
    multi::{self as M},
    sequence::{self as S},
    IResult,
};

type Input = Vec<u32>;

fn parse(s: &str) -> Input {
    todo!()
}

fn answer_part1(inputs: &Input) -> usize {
    todo!()
}

fn answer_part2(inputs: &Input) -> usize {
    todo!()
}

fn main() {
    let inputs = parse(include_str!("inputs"));
    println!("Part 1 = {}", answer_part1(&inputs));
    println!("Part 2 = {}", answer_part2(&inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "";

    #[test]
    fn _part1() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(0, answer_part1(&inputs));
    }
    // #[test]
    // fn _part2() {
    //     let inputs = parse(EXAMPLE_INPUT);
    //     assert_eq!(0, answer_part2(&inputs));
    // }

    // #[bench]
    // fn bench_parse(b: &mut test::Bencher) {
    //     let input = include_str!("inputs");
    //     b.iter(|| {
    //         test::black_box(parse(input));
    //     });
    // }

    // #[bench]
    // fn bench_answer_part1(b: &mut test::Bencher) {
    //     let inputs = parse(include_str!("inputs"));
    //     b.iter(|| {
    //         test::black_box(answer_part1(&inputs));
    //     });
    // }

    // #[bench]
    // fn bench_answer_part2(b: &mut test::Bencher) {
    //     let inputs = parse(include_str!("inputs"));
    //     b.iter(|| {
    //         test::black_box(answer_part2(&inputs));
    //     });
    // }
}
