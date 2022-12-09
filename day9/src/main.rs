#![feature(test)]

use std::collections::HashSet;

extern crate test;

#[derive(Debug, Clone, Copy)]
enum D {
    Left,
    Right,
    Up,
    Down,
}

type Input = Vec<(D, u8)>;

fn parse(s: &str) -> Input {
    s.lines()
        .map(|s| {
            let (dir, n) = s.split_once(" ").unwrap();
            let dir = match dir {
                "L" => D::Left,
                "R" => D::Right,
                "U" => D::Up,
                "D" => D::Down,
                _ => panic!("unexpected direction"),
            };
            let n = n.parse::<u8>().unwrap();
            (dir, n)
        })
        .collect()
}

fn display(visited: &HashSet<(i32, i32)>, knots: &[(i32, i32); 10]) {
    let x_min = visited
        .iter()
        .chain(knots.iter())
        .map(|v| v.0)
        .min()
        .unwrap_or(0);
    let y_min = visited
        .iter()
        .chain(knots.iter())
        .map(|v| v.1)
        .min()
        .unwrap_or(0);
    let x_max = visited
        .iter()
        .chain(knots.iter())
        .map(|v| v.0)
        .max()
        .unwrap_or(0);
    let y_max = visited
        .iter()
        .chain(knots.iter())
        .map(|v| v.1)
        .max()
        .unwrap_or(0);

    for y in 0..=(y_max - y_min) {
        print!("| ");
        for x in 0..=(x_max - x_min) {
            let v = (x + x_min, y + y_min);
            if knots[0] == v {
                print!("H");
            } else if let Some((idx, _)) = knots.iter().enumerate().filter(|(_, k)| **k == v).nth(0)
            {
                print!("{}", idx);
            } else if visited.contains(&v) {
                print!("#");
            } else {
                print!(".");
            }
            print!(" ");
        }
        println!();
    }
}

fn in_range(head: &(i32, i32), tail: &(i32, i32)) -> bool {
    head.0.abs_diff(tail.0) < 2 && head.1.abs_diff(tail.1) < 2
}

fn step(dir: D, v: (i32, i32)) -> (i32, i32) {
    match dir {
        D::Left => (v.0 - 1, v.1),
        D::Right => (v.0 + 1, v.1),
        D::Up => (v.0, v.1 - 1),
        D::Down => (v.0, v.1 + 1),
    }
}

fn answer_part1(inputs: &Input) -> usize {
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = head;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut prev_tail = tail;

    for (dir, n) in inputs {
        for _ in 0..*n {
            let prev_head = head;
            head = step(*dir, head);
            if !in_range(&head, &tail) {
                tail = prev_head;
            }
            // HashSets are slow
            if prev_tail != tail {
                visited.insert(tail);
                prev_tail = tail;
            }
        }
    }
    visited.len()
}

fn answer_part2(inputs: &Input) -> usize {
    let mut knots: [(i32, i32); 10] = Default::default();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut prev_tail = knots[9];

    for (dir, n) in inputs {
        for _ in 0..*n {
            knots[0] = step(*dir, knots[0]);
            for i in 1..10 {
                if !in_range(&knots[i - 1], &knots[i]) {
                    let head = knots[i - 1];
                    let tail = knots[i];
                    knots[i] = (
                        tail.0 + (head.0 - tail.0).min(1).max(-1),
                        tail.1 + (head.1 - tail.1).min(1).max(-1),
                    );
                }
            }
            if prev_tail != knots[9] {
                visited.insert(knots[9]);
                prev_tail = knots[9];
            }
        }
    }
    visited.len()
}

fn main() {
    let inputs = parse(include_str!("inputs"));
    println!("Part 1 = {}", answer_part1(&inputs));
    println!("Part 2 = {}", answer_part2(&inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _part1() {
        let inputs = parse(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        );
        assert_eq!(13, answer_part1(&inputs));
    }
    #[test]
    fn _part2() {
        let inputs = parse(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
        );
        assert_eq!(36, answer_part2(&inputs));
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
