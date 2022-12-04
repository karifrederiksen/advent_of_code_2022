#![feature(test)]

extern crate test;

type Input = Vec<((i32, i32), (i32, i32))>;

fn parse(s: &str) -> Input {
    s.split("\n")
        .map(|line| {
            let (r1, r2) = line.split_once(",").unwrap();
            let (s1, e1) = r1.split_once("-").unwrap();
            let (s2, e2) = r2.split_once("-").unwrap();
            (
                (s1.parse::<i32>().unwrap(), e1.parse::<i32>().unwrap()),
                (s2.parse::<i32>().unwrap(), e2.parse::<i32>().unwrap()),
            )
        })
        .collect()
}

fn has_full_intersect((s1, e1): (i32, i32), (s2, e2): (i32, i32)) -> bool {
    (s1 >= s2 && e1 <= e2) || (s2 >= s1 && e2 <= e1)
}

fn has_no_intersect((s1, e1): (i32, i32), (s2, e2): (i32, i32)) -> bool {
    e1 < s2 || e2 < s1
}

fn answer_part1(inputs: &Input) -> usize {
    inputs
        .iter()
        .filter(|(r1, r2)| has_full_intersect(*r1, *r2))
        .count()
}

fn answer_part2(inputs: &Input) -> usize {
    inputs
        .iter()
        .filter(|(r1, r2)| !has_no_intersect(*r1, *r2))
        .count()
}

fn main() {
    let inputs = parse(include_str!("inputs"));
    println!("Part 1 = {}", answer_part1(&inputs));
    println!("Part 2 = {}", answer_part2(&inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn _part1() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(2, answer_part1(&inputs));
    }
    #[test]
    fn _part2() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(4, answer_part2(&inputs));
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
