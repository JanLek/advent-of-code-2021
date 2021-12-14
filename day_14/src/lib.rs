#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FormatResult},
    str::FromStr,
};

fn part_1(input: &str) -> Result<usize, ParseError> {
    let (mut polymer, pair_insertion_rules) = parse_input(input)?;
    for _ in 0..10 {
        polymer.apply(&pair_insertion_rules);
    }
    Ok(polymer.result())
}

fn part_2(input: &str) -> Result<usize, ParseError> {
    let (mut polymer, pair_insertion_rules) = parse_input(input)?;
    for _ in 0..40 {
        polymer.apply(&pair_insertion_rules);
    }
    Ok(polymer.result())
}

fn parse_input(input: &str) -> Result<(Polymer, PairInsertionRules), ParseError> {
    let (polymer_template, pair_insertion_rules) = input.split_once("\n\n").ok_or(ParseError)?;
    let pair_insertion_rules = pair_insertion_rules.lines().try_fold(
        HashMap::with_capacity(pair_insertion_rules.len()),
        |mut map, line| {
            let (pair, to_insert) = line.split_once(" -> ").ok_or(ParseError)?;
            map.insert(parse_byte_array(pair)?, parse_byte(to_insert)?);
            Ok(map)
        },
    )?;

    Ok((polymer_template.parse()?, pair_insertion_rules))
}

fn parse_byte_array(input: &str) -> Result<[u8; 2], ParseError> {
    let mut pair_bytes = input.bytes();
    let mut array = [0; 2];
    for byte in &mut array {
        *byte = pair_bytes.next().ok_or(ParseError)?;
    }
    Ok(array)
}

fn parse_byte(input: &str) -> Result<u8, ParseError> {
    input.bytes().next().ok_or(ParseError)
}

struct Polymer(Vec<u8>);

impl Polymer {
    fn apply(&mut self, pair_insertion_rules: &PairInsertionRules) {
        self.0 = self
            .0
            .windows(2)
            .flat_map(|pair| {
                let to_insert = pair_insertion_rules.get(pair).unwrap();
                [pair[0], *to_insert]
            })
            .chain(self.0.last().into_iter().copied())
            .collect();
    }

    fn result(&self) -> usize {
        let counts = self.0.iter().fold(HashMap::new(), |mut counts, element| {
            *counts.entry(element).or_insert(0) += 1;
            counts
        });

        counts.values().max().unwrap() - counts.values().min().unwrap()
    }
}

impl FromStr for Polymer {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(input.bytes().collect()))
    }
}

impl Display for Polymer {
    fn fmt(&self, formatter: &mut Formatter) -> FormatResult {
        write!(formatter, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

type PairInsertionRules = HashMap<[u8; 2], u8>;

#[derive(Debug)]
struct ParseError;

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_INPUT).unwrap(), 1_588);
        assert_eq!(part_1(INPUT).unwrap(), 2_602);
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_INPUT).unwrap(), 2_188_189_693_529);
        assert_eq!(part_2(INPUT).unwrap(), 0);
    }

    #[bench]
    #[ignore]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| part_1(INPUT).unwrap());
    }

    #[bench]
    #[ignore]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| part_2(INPUT).unwrap());
    }
}
