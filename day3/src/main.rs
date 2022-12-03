#![feature(test)]

extern crate test;

type Input = Vec<Vec<u8>>;

fn prio(c: u8) -> u8 {
    if c > 96 {
        c - 96
    } else {
        c - 38
    }
}

fn parse(s: &str) -> Input {
    s.split("\n")
        .map(|line| line.as_bytes().iter().copied().map(prio).collect())
        .collect()
}

fn common_items(comp_a: &[u8], comp_b: &[u8]) -> Vec<u8> {
    let mut vals_a = [0u8; 52];
    for &a in comp_a {
        vals_a[(a - 1) as usize] += 1;
    }

    let mut vals_b = [0u8; 52];
    for &b in comp_b {
        vals_b[(b - 1) as usize] += 1;
    }

    let mut res = Vec::new();
    for i in 0..52 {
        if vals_a[i] > 0 && vals_b[i] > 0 {
            res.push((i as u8) + 1)
        }
    }
    res
}

fn answer_part1(inputs: &Input) -> usize {
    inputs
        .iter()
        .map(|line| {
            let i = line.len() / 2;
            common_items(&line[..i], &line[i..])
                .into_iter()
                .map(|n| n as u32)
                .sum::<u32>()
        })
        .sum::<u32>() as usize
}

fn answer_part2(inputs: &Input) -> usize {
    let mut groups: Vec<[&[u8]; 3]> = Vec::new();
    for i in 0..(inputs.len() / 3) {
        groups.push([&inputs[i * 3], &inputs[i * 3 + 1], &inputs[i * 3 + 2]]);
    }
    groups
        .into_iter()
        .map(|[a, b, c]| {
            common_items(&common_items(a, b), c)
                .into_iter()
                .map(|n| n as u32)
                .sum::<u32>()
        })
        .sum::<u32>() as usize
}

fn main() {
    let inputs = parse(include_str!("inputs"));
    println!("Part 1 = {}", answer_part1(&inputs));
    println!("Part 2 = {}", answer_part2(&inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn _prio() {
        assert_eq!(1, prio('a' as u8));
        assert_eq!(26, prio('z' as u8));
        assert_eq!(27, prio('A' as u8));
        assert_eq!(52, prio('Z' as u8));
    }
    #[test]
    fn _part1() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(157, answer_part1(&inputs));
    }
    #[test]
    fn _part2() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(70, answer_part2(&inputs));
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
