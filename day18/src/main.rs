#![feature(test)]
extern crate nom;
extern crate test;
use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator as C,
    multi::{self as M},
    sequence::{self as S},
    IResult,
};

type V3 = (i32, i32, i32);

type Input = Vec<V3>;

fn parse_i32(s: &str) -> IResult<&str, i32> {
    C::map_res(digit1, |n: &str| n.parse::<i32>())(s)
}

fn parse(s: &str) -> Input {
    let parse_v3 = C::map(
        S::tuple((parse_i32, tag(","), parse_i32, tag(","), parse_i32)),
        |(x, _, y, _, z)| (x, y, z),
    );
    M::separated_list1(newline, parse_v3)(s).unwrap().1
}

fn neighbor_positions((x, y, z): V3) -> [V3; 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

fn answer_part1(inputs: &Input) -> usize {
    let inputs: HashSet<V3> = inputs.iter().copied().collect();
    inputs
        .iter()
        .copied()
        .map(|pos| {
            neighbor_positions(pos)
                .into_iter()
                .filter(|pos| !inputs.contains(pos))
                .count()
        })
        .sum()
}

fn in_bounds(min: V3, max: V3, next: &V3) -> bool {
    min.0 <= next.0
        && max.0 >= next.0
        && min.1 <= next.1
        && max.1 >= next.1
        && min.2 <= next.2
        && max.2 >= next.2
}

fn find_reachable(inputs: &Input) -> HashSet<V3> {
    let droplets: HashSet<V3> = inputs.iter().copied().collect();
    let (min, max) = droplets.iter().fold(
        (
            (i32::MAX, i32::MAX, i32::MAX),
            (i32::MIN, i32::MIN, i32::MIN),
        ),
        |(min, max), next| {
            let next_min = (min.0.min(next.0), min.1.min(next.1), min.2.min(next.2));
            let next_max = (max.0.max(next.0), max.1.max(next.1), max.2.max(next.2));
            (next_min, next_max)
        },
    );

    let min = (min.0 - 1, min.1 - 1, min.2 - 1);
    let max = (max.0 + 1, max.1 + 1, max.2 + 1);
    let mut outputs: HashSet<V3> = Default::default();
    let mut prev_layer: HashSet<V3> = Default::default();
    let mut next_layer: HashSet<V3> = Default::default();

    next_layer.insert(min);

    while next_layer.len() > 0 {
        let tmp = prev_layer;
        prev_layer = next_layer;
        next_layer = tmp;
        next_layer.clear();

        for pos in prev_layer.iter().copied() {
            let neighbors = neighbor_positions(pos);
            for pos in neighbors.into_iter() {
                if in_bounds(min, max, &pos)
                    && !prev_layer.contains(&pos)
                    && !droplets.contains(&pos)
                    && !outputs.contains(&pos)
                {
                    next_layer.insert(pos);
                    outputs.insert(pos);
                }
            }
        }
    }

    outputs
}

fn answer_part2(inputs: &Input) -> usize {
    let reachable = find_reachable(inputs);
    inputs
        .iter()
        .copied()
        .map(|pos| {
            neighbor_positions(pos)
                .into_iter()
                .filter(|pos| reachable.contains(pos))
                .count()
        })
        .sum()
}

fn main() {
    let inputs = parse(include_str!("inputs"));
    println!("Part 1 = {}", answer_part1(&inputs));
    println!("Part 2 = {}", answer_part2(&inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn _part1() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(64, answer_part1(&inputs));
    }
    #[test]
    fn _part2() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(58, answer_part2(&inputs));
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
