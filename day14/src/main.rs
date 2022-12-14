#![feature(let_chains)]
#![feature(test)]
extern crate nom;
extern crate test;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator as C,
    multi::{self as M},
    sequence::{self as S},
    IResult,
};

type V2 = (i32, i32);

type Input = Vec<Vec<V2>>;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Cell {
    Empty,
    Rock,
    Sand,
}

struct Grid {
    buf: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn add_line(&mut self, vertices: &[V2]) {
        let width = self.width;
        for (from, to) in vertices[1..].iter().copied().scan(vertices[0], |from, to| {
            let res = (*from, to);
            *from = to;
            Some(res)
        }) {
            // assume it's either straight left or straight right
            let x_from = from.0.min(to.0);
            let x_to = from.0.max(to.0);
            let y_from = from.1.min(to.1);
            let y_to = from.1.max(to.1);
            let range = (x_from..=x_to)
                .map(|x| (x, y_from))
                .chain((y_from..=y_to).map(|y| (x_from, y)));

            for (x, y) in range {
                let x = x as usize;
                let y = y as usize;
                self.buf[y * width + x] = Cell::Rock;
            }
        }
    }

    fn add_sand(&mut self, from_x: usize, from_y: usize) -> bool {
        let y = from_y + 1;
        if y == self.height {
            self.buf[self.width * from_y + from_x] = Cell::Sand;
            return false;
        }
        for x in [from_x, from_x - 1, from_x + 1].into_iter() {
            if self.buf[self.width * y + x] == Cell::Empty {
                return self.add_sand(x, y);
            }
        }
        self.buf[self.width * from_y + from_x] = Cell::Sand;
        return true;
    }
}

fn parse_i32(s: &str) -> IResult<&str, i32> {
    C::map_res(digit1, |n: &str| n.parse::<i32>())(s)
}

fn parse(s: &str) -> Input {
    let parse_v2 = S::separated_pair(parse_i32, tag(","), parse_i32);
    let parse_line = M::separated_list1(tag(" -> "), parse_v2);
    M::separated_list1(newline, parse_line)(s).unwrap().1
}

fn answer_part1(inputs: &Input) -> usize {
    let highest = inputs
        .iter()
        .flat_map(|x| x.iter())
        .copied()
        .map(|(_, y)| y as usize)
        .fold(0, |y1, y| (y1.max(y)));

    let width = 1000;
    let height = highest + 1;
    let mut grid = Grid {
        buf: vec![Cell::Empty; width * height],
        height,
        width,
    };
    for line in inputs.iter() {
        grid.add_line(line);
    }

    (0..)
        .skip_while(|_| grid.add_sand(500, 0))
        .nth(0)
        .unwrap_or(usize::MAX)
}

fn answer_part2(inputs: &Input) -> usize {
    let highest = inputs
        .iter()
        .flat_map(|x| x.iter())
        .copied()
        .map(|(_, y)| y as usize)
        .fold(0, |y1, y| (y1.max(y)));

    let height = highest + 2;
    let width = 1000;
    let mut grid = Grid {
        buf: vec![Cell::Empty; width * height],
        height,
        width,
    };
    for line in inputs.iter() {
        grid.add_line(line);
    }

    let answer = (0..)
        .skip_while(|_| {
            if grid.buf[500] == Cell::Sand {
                return false;
            }
            grid.add_sand(500, 0);
            true
        })
        .nth(0)
        .unwrap_or(usize::MAX);

    answer
}

fn main() {
    let inputs = parse(include_str!("inputs"));
    println!("Part 1 = {}", answer_part1(&inputs));
    print!("Part 2 = ");
    println!("{}", answer_part2(&inputs))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn _part1() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(24, answer_part1(&inputs));
    }
    #[test]
    fn _part2() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(93, answer_part2(&inputs));
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
