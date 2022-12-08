#![feature(test)]

extern crate test;

struct Input {
    width: usize,
    height: usize,
    buf: Vec<u8>,
}

impl Input {
    fn size(&self, x: usize, y: usize) -> u8 {
        self.buf[(y * self.width) + x]
    }
}

fn parse(s: &str) -> Input {
    let width = s.find("\r\n").unwrap_or_else(|| s.find("\n").unwrap());

    let buf: Vec<u8> = s
        .lines()
        .flat_map(|line| line.as_bytes())
        .copied()
        .map(|c| c - 48)
        .collect();

    Input {
        width: width,
        height: buf.len() / width,
        buf,
    }
}

fn is_visible(input: &Input, x: usize, y: usize) -> bool {
    let n = input.size(x, y);
    let row = &input.buf[(input.width * y)..(input.width * (y + 1))];
    let left = row[..x].iter().all(|m| *m < n);
    let right = row[(x + 1)..].iter().all(|m| *m < n);
    let top = (0..y).all(|y| input.size(x, y) < n);
    let bottom = ((y + 1)..input.height).all(|y| input.size(x, y) < n);

    left || right || top || bottom
}

fn answer_part1(inputs: &Input) -> usize {
    let edge_visible = inputs.width * 2 + (inputs.height * 2) - 4;
    let interior_visible = (1..(inputs.height - 1))
        .flat_map(|y| (1..(inputs.width - 1)).map(move |x| (x, y)))
        .filter(|&(x, y)| is_visible(inputs, x, y))
        .count();
    edge_visible + interior_visible
}

fn scenic_score(input: &Input, x: usize, y: usize) -> usize {
    let n = input.size(x, y);
    let row = &input.buf[(input.width * y)..(input.width * (y + 1))];
    let left = row[..x]
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, n2)| **n2 >= n)
        .map(|(x2, _)| x.abs_diff(x2))
        .nth(0)
        .unwrap_or(x);
    let right = row[(x + 1)..]
        .iter()
        .enumerate()
        .map(|(i, n)| (i + x + 1, n))
        .filter(|(_, n2)| **n2 >= n)
        .map(|(x2, _)| x.abs_diff(x2))
        .nth(0)
        .unwrap_or(input.width - x - 1);
    let top = (0..y)
        .rev()
        .filter(|y| input.size(x, *y) >= n)
        .map(|y2| y.abs_diff(y2))
        .nth(0)
        .unwrap_or(y);
    let bottom = ((y + 1)..input.height)
        .filter(|y| input.size(x, *y) >= n)
        .map(|y2| y.abs_diff(y2))
        .nth(0)
        .unwrap_or(input.height - y - 1);

    top * left * bottom * right
}

fn answer_part2(inputs: &Input) -> usize {
    (1..(inputs.height - 1))
        .flat_map(|y| (1..(inputs.width - 1)).map(move |x| (x, y)))
        .map(|(x, y)| scenic_score(inputs, x, y))
        .max()
        .unwrap_or(0)
}

fn main() {
    let inputs = parse(include_str!("inputs"));
    println!("Part 1 = {}", answer_part1(&inputs));
    println!("Part 2 = {}", answer_part2(&inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "30373
25512
65332
33549
35390";

    #[test]
    fn _part1() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(21, answer_part1(&inputs));
    }
    #[test]
    fn _part2() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(8, answer_part2(&inputs));
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
