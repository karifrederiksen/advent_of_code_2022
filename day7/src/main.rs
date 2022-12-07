#![feature(test)]

use std::collections::HashMap;

extern crate test;

#[derive(Debug)]
enum Com {
    Ls { output: Vec<LsNode> },
    Cd { arg: String },
}

#[derive(Debug)]
enum LsNode {
    File { name: String, size: usize },
    Dir { name: String },
}

#[derive(Debug)]
enum FsNode {
    File(usize),
    Dir((usize, HashMap<String, FsNode>)),
}

impl FsNode {
    fn size(&self) -> usize {
        match self {
            Self::Dir((size, _)) => *size,
            Self::File(size) => *size,
        }
    }
}

type Input = Vec<Com>;

fn infer_filesystem(commands: &[Com]) -> FsNode {
    let mut ancestors: Vec<(String, FsNode)> = vec![];
    let mut node: HashMap<String, FsNode> = Default::default();
    for com in commands {
        match com {
            Com::Ls { output } => {
                for out in output {
                    match out {
                        LsNode::File { name, size } => {
                            node.insert(name.to_string(), FsNode::File(*size));
                        }
                        LsNode::Dir { name } => {
                            node.insert(name.to_string(), FsNode::Dir(Default::default()));
                        }
                    }
                }
            }
            Com::Cd { arg } => {
                if arg == "/" {
                    continue;
                } else if arg == ".." {
                    if let Some((name, parent)) = ancestors.pop() {
                        let mut parent = match parent {
                            FsNode::Dir((_, n)) => n,
                            FsNode::File(_) => unreachable!(),
                        };
                        parent.insert(name, FsNode::Dir((0, node)));
                        node = parent;
                    } else {
                        unreachable!();
                    }
                } else if let Some(next_node) = node.remove(arg) {
                    let node_ = match next_node {
                        FsNode::Dir((_, n)) => n,
                        FsNode::File(_) => unreachable!(),
                    };
                    ancestors.push((arg.to_string(), FsNode::Dir((0, node))));
                    node = node_;
                } else {
                    panic!("impossible!")
                }
            }
        }
    }

    for (name, parent) in ancestors.into_iter() {
        let mut parent = match parent {
            FsNode::Dir((_, n)) => n,
            FsNode::File(_) => unreachable!(),
        };
        parent.insert(name, FsNode::Dir((0, node)));
        node = parent;
    }

    let mut root = FsNode::Dir((0, node));
    cache_size_of(&mut root);
    root
}

fn cache_size_of(n: &mut FsNode) -> usize {
    match n {
        FsNode::File(size) => *size,
        FsNode::Dir((ref mut size, ns)) => {
            let n = ns.iter_mut().map(|(_, n)| cache_size_of(n)).sum();
            *size = n;
            n
        }
    }
}

fn parse_cd(s: &str) -> Option<(&str, Com)> {
    if !s.starts_with("$ cd ") {
        return None;
    }
    let s = &s["$ cd ".len()..];
    let (arg, rest) = s.split_once("\n").unwrap_or_else(|| (s, ""));

    Some((
        rest,
        Com::Cd {
            arg: arg.to_string(),
        },
    ))
}
fn parse_ls(s: &str) -> Option<(&str, Com)> {
    if !s.starts_with("$ ls\n") {
        return None;
    }
    let s = &s["$ ls\n".len()..];
    let mut nodes: Vec<LsNode> = Vec::new();
    let mut s = s;

    while s.len() > 0 && !s.starts_with("$") {
        let (a, rest_s) = s.split_once(" ").unwrap();
        let (b, rest_s) = rest_s.split_once("\n").unwrap_or_else(|| (s, ""));
        s = rest_s;
        if a == "dir" {
            nodes.push(LsNode::Dir {
                name: b.to_string(),
            });
        } else {
            nodes.push(LsNode::File {
                name: b.to_string(),
                size: a.parse::<usize>().unwrap(),
            });
        }
    }

    Some((s, Com::Ls { output: nodes }))
}

fn parse(s: &str) -> Input {
    let mut coms: Vec<Com> = Vec::new();
    let mut s = s;
    while s.len() > 0 {
        if let Some((next_s, com)) = parse_cd(s).or_else(|| parse_ls(s)) {
            coms.push(com);
            s = next_s;
        } else {
            panic!("Unexpected input: \'{}\'", s)
        }
    }
    coms
}

fn answer_part1_(input: FsNode) -> usize {
    match input {
        FsNode::Dir((dir_size, sub_nodes)) => {
            let dir_size = if dir_size <= 100_000 { dir_size } else { 0 };
            let sub_sum = sub_nodes
                .into_iter()
                .map(|(_, n)| answer_part1_(n))
                .sum::<usize>();
            dir_size + sub_sum
        }
        _ => 0,
    }
}
fn answer_part1(inputs: &Input) -> usize {
    let root = infer_filesystem(inputs);
    answer_part1_(root)
}

const FS_TOTAL_SPACE: usize = 70_000_000;
const FS_REQUIRED_SPACE: usize = 30_000_000;

fn answer_part2_(input: FsNode, space_to_free: usize) -> usize {
    let sum = match input {
        FsNode::Dir((dir_size, sub_nodes)) => {
            let dir_size = if dir_size > space_to_free {
                dir_size
            } else {
                usize::MAX
            };
            let sub_min = sub_nodes
                .into_iter()
                .map(|(_, n)| answer_part2_(n, space_to_free))
                .min()
                .unwrap_or(usize::MAX);
            usize::min(dir_size, sub_min)
        }
        _ => usize::MAX,
    };
    sum
}
fn answer_part2(inputs: &Input) -> usize {
    let root = infer_filesystem(inputs);
    println!("Root = {}", root.size());
    let available_space = FS_TOTAL_SPACE - root.size();
    println!("Available = {}", available_space);
    let space_to_free = FS_REQUIRED_SPACE - available_space;
    println!("To free = {}", space_to_free);
    answer_part2_(root, space_to_free)
}

fn main() {
    let inputs = parse(include_str!("inputs"));
    println!("Part 1 = {}", answer_part1(&inputs));
    println!("Part 2 = {}", answer_part2(&inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn _part1() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(95437, answer_part1(&inputs));
    }
    #[test]
    fn _part2() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(24933642, answer_part2(&inputs));
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
