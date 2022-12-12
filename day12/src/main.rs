#![feature(test)]

extern crate test;

type N = u32;
type V3 = (N, N, u8);
type Input = (V3, V3, Grid);
struct Grid {
    width: N,
    height: N,
    grid: Vec<u8>,
}

fn parse(s: &str) -> Input {
    let mut start: N = 0;
    let mut end: N = 0;
    let mut width: N = 0;
    let mut grid: Vec<u8> = vec![];
    let mut idx = 0;
    for b in s.as_bytes().iter().copied() {
        match b {
            b'\n' => {
                if width == 0 {
                    width = idx;
                }
            }
            b'E' => {
                end = idx;
                grid.push(b'z');
                idx += 1;
            }
            b'S' => {
                start = idx;
                grid.push(b'a');
                idx += 1;
            }
            _ => {
                grid.push(b);
                idx += 1;
            }
        }
    }
    let start = ((start % width), (start / width), grid[start as usize]);
    let end = ((end % width), (end / width), grid[end as usize]);
    (
        start,
        end,
        Grid {
            width,
            height: grid.len() as N / width,
            grid,
        },
    )
}

fn h(pos: &V3, end: &V3) -> u32 {
    pos.0.abs_diff(end.0).pow(2) + pos.1.abs_diff(end.1).pow(2) + pos.2.abs_diff(end.2) as N
}

fn get_neighbors<'a, const UP: u8, const DOWN: u8>(
    grid: &'a Grid,
    current: V3,
) -> impl IntoIterator<Item = V3> + 'a {
    let left = current.0.checked_sub(1).map(|x| (x, current.1));
    let right = current
        .0
        .checked_add(1)
        .filter(|x| *x < grid.width as N)
        .map(|x| (x, current.1));
    let up = current.1.checked_sub(1).map(|y| (current.0, y));
    let down = current
        .1
        .checked_add(1)
        .filter(|y| *y < grid.height as N)
        .map(|y| (current.0, y));
    [left, right, up, down]
        .into_iter()
        .flatten()
        .map(move |(x, y)| (x, y, grid.grid[(y * grid.width + x) as usize]))
        .filter(move |p| {
            (p.2 >= current.2 && (p.2 - current.2 <= UP))
                || (p.2 < current.2 && (current.2 - p.2) <= DOWN)
        })
}

fn answer_part1((start, end, grid): &Input) -> Option<usize> {
    let move_cost = 1;
    let res = pathfinding::directed::astar::astar(
        start,
        |current| {
            get_neighbors::<1, 255>(&grid, *current)
                .into_iter()
                .map(|pos| (pos, move_cost))
        },
        |pos| h(pos, end),
        |n| n == end,
    );
    res.map(|x| x.0.len() - 1)
}

fn answer_part2((_, end, grid): &Input) -> Option<usize> {
    let move_cost = 1;
    let res = pathfinding::directed::astar::astar(
        end,
        |current| {
            get_neighbors::<255, 1>(&grid, *current)
                .into_iter()
                .map(|pos| (pos, move_cost))
        },
        |(_, _, y)| (b'z' - *y) as u32,
        |(_, _, y)| *y == b'a',
    );
    res.map(|x| x.0.len() - 1)
}

fn main() {
    let inputs = parse(include_str!("inputs"));
    println!(
        "Part 1 = {}",
        answer_part1(&inputs)
            .map(|n| format!("{}", n))
            .unwrap_or("None".to_string())
    );
    println!(
        "Part 2 = {}",
        answer_part2(&inputs)
            .map(|n| format!("{}", n))
            .unwrap_or("None".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn _part1() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(Some(31), answer_part1(&inputs));
    }
    #[test]
    fn _part2() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(Some(29), answer_part2(&inputs));
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
