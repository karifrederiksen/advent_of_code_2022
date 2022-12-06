#![feature(test)]

extern crate test;

type Input = String;

fn parse(s: &str) -> Input {
    s.to_string()
}

fn all_distinct<const N: usize>(buf: &[u8; N]) -> bool {
    for i in 0..N {
        for j in (i + 1)..N {
            if buf[i] == buf[j] {
                return false;
            }
        }
    }
    true
}

fn index_of_consecutive_distinct<const N: usize>(s: &str) -> Option<usize> {
    let s = s.as_bytes();
    let mut buf: [u8; N] = [0u8; N];
    for i in 0..N {
        buf[i] = s[i];
    }
    if all_distinct(&buf) {
        return Some(N + 1);
    }

    for i in 0..(s.len() - N) {
        for j in 0..(N - 1) {
            buf[j] = buf[j + 1];
        }
        buf[N - 1] = s[i + N];
        if all_distinct(&buf) {
            return Some(i + N + 1);
        }
    }
    None
}
fn answer_part1(inputs: &Input) -> Option<usize> {
    index_of_consecutive_distinct::<4>(inputs)
}

fn answer_part2(inputs: &Input) -> Option<usize> {
    index_of_consecutive_distinct::<14>(inputs)
}

fn main() {
    let inputs = parse(include_str!("inputs"));
    if let Some(answer) = answer_part1(&inputs) {
        println!("Part 1 = {}", answer);
    } else {
        println!("Part 1 = no answer");
    }
    if let Some(answer) = answer_part2(&inputs) {
        println!("Part 2 = {}", answer);
    } else {
        println!("Part 2 = no answer");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _part1() {
        assert_eq!(
            Some(7),
            answer_part1(&parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb"))
        );
        assert_eq!(
            Some(5),
            answer_part1(&parse("bvwbjplbgvbhsrlpgdmjqwftvncz"))
        );
        assert_eq!(
            Some(6),
            answer_part1(&parse("nppdvjthqldpwncqszvftbrmjlhg"))
        );
        assert_eq!(
            Some(10),
            answer_part1(&parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"))
        );
        assert_eq!(
            Some(11),
            answer_part1(&parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"))
        );
    }
    // #[test]
    // fn _part2() {
    //     let inputs = parse(EXAMPLE_INPUT);
    //     assert_eq!(0, answer_part2(&inputs));
    // }

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

    // #[bench]
    // fn bench_answer_part2(b: &mut test::Bencher) {
    //     let inputs = parse(include_str!("inputs"));
    //     b.iter(|| {
    //         test::black_box(answer_part2(&inputs));
    //     });
    // }
}
