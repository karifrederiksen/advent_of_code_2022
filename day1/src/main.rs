#![feature(test)]

extern crate test;

type Input = Vec<Vec<u32>>;

fn parse(s: &str) -> Input {
    s.split("\n\n")
        .map(|t| {
            t.split("\n")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn answer_part1(inputs: &Input) -> usize {
    let mut sums: Vec<u32> = inputs.iter().map(|x| x.iter().sum()).collect();
    sums.sort();
    *sums.last().unwrap() as usize
}

fn answer_part2(inputs: &Input) -> usize {
    let mut sums: Vec<u32> = inputs.iter().map(|x| x.iter().sum()).collect();
    sums.sort();
    sums[(sums.len() - 3)..].iter().sum::<u32>() as usize
}

fn main() {
    let inputs = parse(include_str!("inputs"));
    println!("Part 1 = {}", answer_part1(&inputs));
    println!("Part 2 = {}", answer_part2(&inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn _part1() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(24000, answer_part1(&inputs));
    }
    #[test]
    fn _part2() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(45000, answer_part2(&inputs));
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
