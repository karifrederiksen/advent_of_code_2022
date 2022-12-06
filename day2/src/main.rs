#![feature(test)]

extern crate test;

#[derive(Clone, Copy, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}
#[derive(Clone, Copy, Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

type Input = Vec<(Move, Outcome)>;

fn parse(s: &str) -> Input {
    use Move::*;
    use Outcome::*;
    s.replace("\r", "")
        .split("\n")
        .map(|x| {
            let (l, r) = x.split_once(" ").unwrap();
            let l = match l {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissors,
                _ => panic!("Attempt to parse '{}' as ABC", l),
            };
            let r = match r {
                "X" => Lose,
                "Y" => Draw,
                "Z" => Win,
                _ => panic!("Attempt to parse '{}' as XYZ", r),
            };
            (l, r)
        })
        .collect::<Vec<_>>()
}

fn score(opponent: Move, me: Outcome) -> u32 {
    use Move::*;
    use Outcome::*;
    let score = match (me, opponent) {
        (Lose, Rock) => 3,
        (Lose, Paper) => 0,
        (Lose, Scissors) => 6,

        (Draw, Rock) => 6,
        (Draw, Paper) => 3,
        (Draw, Scissors) => 0,

        (Win, Rock) => 0,
        (Win, Paper) => 6,
        (Win, Scissors) => 3,
    };
    let pick = match me {
        Lose => 1,
        Draw => 2,
        Win => 3,
    };
    score + pick
}

fn score2(opponent: Move, outcome: Outcome) -> u32 {
    use Move::*;
    use Outcome::*;
    match (outcome, opponent) {
        (Lose, Rock) => 0 + 3,
        (Lose, Paper) => 0 + 1,
        (Lose, Scissors) => 0 + 2,

        (Draw, Rock) => 3 + 1,
        (Draw, Paper) => 3 + 2,
        (Draw, Scissors) => 3 + 3,

        (Win, Rock) => 6 + 2,
        (Win, Paper) => 6 + 3,
        (Win, Scissors) => 6 + 1,
    }
}

fn answer_part1(inputs: &Input) -> usize {
    inputs.iter().map(|&(l, r)| score(l, r)).sum::<u32>() as usize
}

fn answer_part2(inputs: &Input) -> usize {
    inputs.iter().map(|&(l, r)| score2(l, r)).sum::<u32>() as usize
}

fn main() {
    let inputs = parse(include_str!("inputs"));
    println!("Part 1 = {}", answer_part1(&inputs));
    println!("Part 2 = {}", answer_part2(&inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "A Y
B X
C Z";

    #[test]
    fn _part1() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(15, answer_part1(&inputs));
    }
    #[test]
    fn _part2() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(12, answer_part2(&inputs));
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
