#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::{collections::HashMap, iter::IntoIterator, str::FromStr};

fn part_1(input: &str) -> Result<usize, ParseError> {
    let (polymer, pair_insertion_rules) = parse_input(input)?;
    Ok(calculate_result(&polymer, &pair_insertion_rules, 10))
}

fn part_2(input: &str) -> Result<usize, ParseError> {
    let (polymer, pair_insertion_rules) = parse_input(input)?;
    Ok(calculate_result(&polymer, &pair_insertion_rules, 40))
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

type ElementCounts = HashMap<u8, usize>;
type ElementCountCache = HashMap<([u8; 2], usize), ElementCounts>;

fn calculate_result(
    polymer: &Polymer,
    pair_insertion_rules: &PairInsertionRules,
    steps: usize,
) -> usize {
    let mut cache: ElementCountCache = HashMap::new();
    let mut counts = polymer
        .0
        .windows(2)
        .map(|pair| count_elements([pair[0], pair[1]], steps, pair_insertion_rules, &mut cache))
        .flat_map(IntoIterator::into_iter)
        .fold(HashMap::new(), |mut counts, (element, count)| {
            *counts.entry(element).or_insert(0) += count;
            counts
        });

    // Subtract double counted elements.
    for element in polymer.0.iter().skip(1).take(polymer.0.len() - 2) {
        *counts.get_mut(element).unwrap() -= 1;
    }

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn count_elements(
    pair: [u8; 2],
    steps: usize,
    pair_insertion_rules: &PairInsertionRules,
    cache: &mut ElementCountCache,
) -> ElementCounts {
    if let Some(result) = cache.get(&(pair, steps)) {
        result.clone()
    } else if steps == 0 {
        let result: ElementCounts = pair
            .into_iter()
            .fold(HashMap::new(), |mut result, element| {
                *result.entry(element).or_insert(0) += 1;
                result
            });
        cache.insert((pair, steps), result.clone());
        result
    } else {
        let insert = *pair_insertion_rules.get(&pair).unwrap();
        let mut result = [[pair[0], insert], [insert, pair[1]]]
            .into_iter()
            .map(|pair| count_elements(pair, steps - 1, pair_insertion_rules, cache))
            .flat_map(IntoIterator::into_iter)
            .fold(HashMap::new(), |mut counts, (element, count)| {
                *counts.entry(element).or_insert(0) += count;
                counts
            });
        *result.get_mut(&insert).unwrap() -= 1;
        cache.insert((pair, steps), result.clone());
        result
    }
}

struct Polymer(Vec<u8>);

impl FromStr for Polymer {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(input.bytes().collect()))
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
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_INPUT).unwrap(), 2_188_189_693_529);
        assert_eq!(part_2(INPUT).unwrap(), 2_942_885_922_173);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| part_1(INPUT).unwrap());
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| part_2(INPUT).unwrap());
    }
}
