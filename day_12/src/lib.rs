#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::collections::HashMap;

fn part_1(input: &str) -> usize {
    let connections = build_connections_map(input);
    count_possible_paths(&connections, |path, cave| {
        cave.is_large() || !path.contains(&cave)
    })
}

fn part_2(input: &str) -> usize {
    let connections = build_connections_map(input);
    let start = Cave::parse("start");
    count_possible_paths(&connections, move |path, cave| {
        cave.is_large() || (cave != start && (!path.used_double_visit || !path.contains(&cave)))
    })
}

fn build_connections_map(input: &str) -> HashMap<Cave, Vec<Cave>> {
    input
        .lines()
        .flat_map(|line| {
            let (left, right) = line.split_once('-').unwrap();
            let left = Cave::parse(left);
            let right = Cave::parse(right);
            [(left, right), (right, left)]
        })
        .fold(HashMap::with_capacity(15), |mut map, (from, to)| {
            map.entry(from).or_default().push(to);
            map
        })
}

fn count_possible_paths(
    connections: &HashMap<Cave, Vec<Cave>>,
    can_be_added: impl Fn(&Path, Cave) -> bool,
) -> usize {
    let start = Cave::parse("start");
    let end = Cave::parse("end");
    let mut candidate_paths = vec![Path::from(vec![start])];
    let mut num_possible_paths = 0;

    while let Some(candidate_path) = candidate_paths.pop() {
        let currently_at = candidate_path.last_cave();
        let can_go_to = connections.get(currently_at).unwrap();
        for &cave in can_go_to {
            if cave == end {
                num_possible_paths += 1;
            } else if can_be_added(&candidate_path, cave) {
                candidate_paths.push(candidate_path.new_with(cave));
            }
        }
    }

    num_possible_paths
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

    fn is_small(&self) -> bool {
        !self.is_large()
    }
}

struct Path<'a> {
    caves: Vec<Cave<'a>>,
    used_double_visit: bool,
}

impl<'a> Path<'a> {
    fn from(caves: Vec<Cave<'a>>) -> Self {
        Self {
            caves,
            used_double_visit: false,
        }
    }

    fn last_cave(&self) -> &Cave {
        self.caves.last().unwrap()
    }

    fn caves(&self) -> impl Iterator<Item = &Cave> {
        self.caves.iter()
    }

    fn len(&self) -> usize {
        self.caves.len()
    }

    fn contains(&self, cave: &Cave) -> bool {
        self.caves.contains(cave)
    }

    fn new_with(&self, cave: Cave<'a>) -> Self {
        let mut new_path = self.caves.clone();
        new_path.push(cave);

        let used_double_visit = self.used_double_visit || (cave.is_small() && self.contains(&cave));

        Self {
            caves: new_path,
            used_double_visit,
        }
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
    fn test_part_2() {
        assert_eq!(part_2(SMALL_SAMPLE_INPUT), 36);
        assert_eq!(part_2(SAMPLE_INPUT), 103);
        assert_eq!(part_2(LARGE_SAMPLE_INPUT), 3509);
        assert_eq!(part_2(INPUT), 118_242);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| part_1(INPUT));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| part_2(INPUT));
    }
}
