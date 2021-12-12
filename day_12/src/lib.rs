#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::collections::HashMap;

fn part_1(input: &str) -> usize {
    let connections = build_connections_map(input);

    let start = Cave::parse("start");
    let end = Cave::parse("end");
    let mut candidate_paths = vec![vec![start]];
    let mut complete_paths: Vec<Vec<Cave>> = Vec::new();

    while let Some(candidate_path) = candidate_paths.pop() {
        let currently_at = candidate_path.last().unwrap();
        let can_go_to = connections.get(currently_at).unwrap();
        for &cave in can_go_to {
            if cave == end {
                complete_paths.push(add_to_path(&candidate_path, cave));
            } else if cave.is_large() || !candidate_path.contains(&cave) {
                candidate_paths.push(add_to_path(&candidate_path, cave));
            } else {
                // This path does not work, skip.
            }
        }
    }

    complete_paths.len()
}

fn part_2(_input: &str) -> usize {
    todo!()
}

fn build_connections_map(input: &str) -> HashMap<Cave, Vec<Cave>> {
    input
        .lines()
        .map(Connection::parse)
        .flat_map(|Connection(left, right)| [(left, right), (right, left)])
        .fold(HashMap::new(), |mut map, (from, to)| {
            map.entry(from).or_default().push(to);
            map
        })
}

fn add_to_path<'a>(path: &[Cave<'a>], cave: Cave<'a>) -> Vec<Cave<'a>> {
    let mut new_path = Vec::from(path);
    new_path.push(cave);
    new_path
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Cave<'a> {
    Large(&'a str),
    Small(&'a str),
}

impl<'a> Cave<'a> {
    fn parse(input: &'a str) -> Self {
        if input.bytes().all(|c: u8| c.is_ascii_uppercase()) {
            Self::Large(input)
        } else {
            Self::Small(input)
        }
    }

    fn is_large(&self) -> bool {
        match self {
            Self::Large(_) => true,
            Self::Small(_) => false,
        }
    }
}

struct Connection<'a>(Cave<'a>, Cave<'a>);

impl<'a> Connection<'a> {
    fn parse(input: &'a str) -> Self {
        let (left, right) = input.split_once('-').unwrap();
        Self(Cave::parse(left), Cave::parse(right))
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    const SMALL_SAMPLE_INPUT: &str = include_str!("small_sample_input.txt");
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");
    const LARGE_SAMPLE_INPUT: &str = include_str!("large_sample_input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SMALL_SAMPLE_INPUT), 10);
        assert_eq!(part_1(SAMPLE_INPUT), 19);
        assert_eq!(part_1(LARGE_SAMPLE_INPUT), 226);
        assert_eq!(part_1(INPUT), 4_304);
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_INPUT), 0);
        assert_eq!(part_2(INPUT), 0);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| part_1(INPUT));
    }

    #[bench]
    #[ignore]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| part_2(INPUT));
    }
}
