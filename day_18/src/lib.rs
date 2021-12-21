#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(int_roundings, test)]

use std::{
    fmt::{Debug, Formatter, Result as FormatResult},
    iter::Sum,
    num::ParseIntError,
    ops::Add,
    str::FromStr,
};

fn part_1(input: &str) -> Result<u32, ParseError> {
    let numbers: Vec<SnailfishNumber> = input.lines().map(str::parse).collect::<Result<_, _>>()?;
    let sum: SnailfishNumber = numbers.into_iter().sum();
    dbg!(&sum);
    Ok(sum.magnitude())
}

fn part_2(_input: &str) -> Result<u32, ParseError> {
    todo!()
}

#[derive(Clone, PartialEq)]
enum SnailfishNumber {
    Single(u32),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

impl SnailfishNumber {
    fn new(a: Self, b: Self) -> Self {
        Self::Pair(Box::new(a), Box::new(b))
    }

    fn reduce(self) -> Self {
        let mut result = self;
        loop {
            let (did_explode, exploded) = result.clone().explode();
            if did_explode {
                result = exploded;
                continue;
            }

            let (did_split, split) = result.clone().split();
            if did_split {
                result = split;
                continue;
            }

            break; // No explosion & no split => done
        }
        result
    }

    fn magnitude(self) -> u32 {
        match self {
            Self::Single(n) => n,
            Self::Pair(a, b) => a.magnitude() * 3 + b.magnitude() * 2,
        }
    }

    fn explode(self) -> (bool, Self) {
        let (did_explode, _, number, _) = self.do_explode(0);
        (did_explode, number)
    }

    fn do_explode(self, depth: u8) -> (bool, Option<u32>, Self, Option<u32>) {
        use SnailfishNumber::{Pair, Single};

        match (self, depth) {
            (Single(n), _) => (false, None, Single(n), None),
            (Pair(a, b), 4) => (
                true,
                Some(a.unwrap_single()),
                Single(0),
                Some(b.unwrap_single()),
            ), // EXPLODE!
            (Pair(a, b), depth) if depth < 4 => {
                match a.clone().do_explode(depth + 1) {
                    (false, None, _, None) => {
                        // left side did not explode, try right side below
                    }
                    (true, None, number, None) => {
                        return (true, None, Self::new(number, *b), None);
                    }
                    (did_explode, None, number, Some(n)) => {
                        return (
                            did_explode,
                            None,
                            Self::new(number, b.add_to_left_most_number(n)),
                            None,
                        );
                    }
                    (did_explode, Some(n), number, None) => {
                        return (did_explode, Some(n), Self::new(number, *b), None);
                    }
                    (did_explode, Some(n1), number, Some(n2)) => {
                        return (
                            did_explode,
                            Some(n1),
                            Self::new(number, b.add_to_left_most_number(n2)),
                            None,
                        );
                    }
                };

                match b.clone().do_explode(depth + 1) {
                    (false, None, _, None) => (false, None, Pair(a, b), None), // Right side did not explode either.
                    (true, None, number, None) => (true, None, Self::new(*a, number), None),
                    (did_explode, None, number, Some(n)) => {
                        (did_explode, None, Self::new(*a, number), Some(n))
                    }
                    (did_explode, Some(n), number, None) => (
                        did_explode,
                        None,
                        Self::new(a.add_to_right_most_number(n), number),
                        None,
                    ),
                    (did_explode, Some(n1), number, Some(n2)) => (
                        did_explode,
                        None,
                        Self::new(a.add_to_right_most_number(n1), number),
                        Some(n2),
                    ),
                }
            }
            _ => panic!("Depth too high: {}", depth),
        }
    }

    fn split(self) -> (bool, Self) {
        use SnailfishNumber::{Pair, Single};

        match self {
            Single(n) if n >= 10 => (
                true,
                Self::new(
                    Single(n.unstable_div_floor(2)),
                    Single(n.unstable_div_ceil(2)),
                ),
            ),
            Single(n) => (false, Single(n)),
            Pair(a, b) => {
                let (did_split, split_a) = a.clone().split();
                if did_split {
                    (did_split, Self::new(split_a, *b))
                } else {
                    let (did_split, split_b) = b.split();
                    (did_split, Self::new(*a, split_b))
                }
            }
        }
    }

    fn unwrap_single(self) -> u32 {
        match self {
            Self::Single(n) => n,
            Self::Pair(_, _) => panic!(),
        }
    }

    fn add_to_left_most_number(self, number: u32) -> Self {
        match self {
            Self::Single(n) => Self::Single(n + number),
            Self::Pair(a, b) => Self::new(a.add_to_left_most_number(number), *b),
        }
    }

    fn add_to_right_most_number(self, number: u32) -> Self {
        match self {
            Self::Single(n) => Self::Single(n + number),
            Self::Pair(a, b) => Self::new(*a, b.add_to_right_most_number(number)),
        }
    }
}

impl Add for SnailfishNumber {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self, other).reduce()
    }
}

impl Debug for SnailfishNumber {
    fn fmt(&self, formatter: &mut Formatter) -> FormatResult {
        match self {
            Self::Single(n) => n.fmt(formatter),
            Self::Pair(a, b) => {
                write!(formatter, "[")?;
                a.fmt(formatter)?;
                write!(formatter, ",")?;
                b.fmt(formatter)?;
                write!(formatter, "]")?;
                Ok(())
            }
        }
    }
}

impl FromStr for SnailfishNumber {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (number, remaining) = parse_pair_snailfish_number(input)?;
        if remaining.is_empty() {
            Ok(number)
        } else {
            Err(ParseError)
        }
    }
}

impl Sum for SnailfishNumber {
    fn sum<I>(numbers: I) -> SnailfishNumber
    where
        I: Iterator<Item = SnailfishNumber>,
    {
        numbers
            .reduce(Add::add)
            .expect("empty iterators are not supported")
    }
}

fn parse_snailfish_number(input: &str) -> Result<(SnailfishNumber, &str), ParseError> {
    parse_single_snailfish_number(input).or_else(|_| parse_pair_snailfish_number(input))
}

fn parse_single_snailfish_number(input: &str) -> Result<(SnailfishNumber, &str), ParseError> {
    let (digits, remaining) = parse_at_least_one_digit(input)?;
    let number = digits.parse()?;
    Ok((SnailfishNumber::Single(number), remaining))
}

fn parse_pair_snailfish_number(input: &str) -> Result<(SnailfishNumber, &str), ParseError> {
    let (_, input) = parse_character('[', input)?;
    let (a, input) = parse_snailfish_number(input)?;
    let (_, input) = parse_character(',', input)?;
    let (b, input) = parse_snailfish_number(input)?;
    let (_, input) = parse_character(']', input)?;

    Ok((SnailfishNumber::new(a, b), input))
}

fn parse_at_least_one_digit(input: &str) -> Result<(&str, &str), ParseError> {
    let num_digits = input.chars().take_while(|c| c.is_digit(10)).count();
    if num_digits > 0 {
        Ok((&input[0..num_digits], &input[num_digits..]))
    } else {
        Err(ParseError)
    }
}

fn parse_character(character: char, input: &str) -> Result<(char, &str), ParseError> {
    if input.starts_with(character) {
        Ok((character, &input[character.len_utf8()..]))
    } else {
        Err(ParseError)
    }
}

#[derive(Debug)]
struct ParseError;

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_addition() {
        let a: SnailfishNumber = "[1,2]".parse().unwrap();
        let b: SnailfishNumber = "[[3,4],5]".parse().unwrap();
        let expected: SnailfishNumber = "[[1,2],[[3,4],5]]".parse().unwrap();

        assert_eq!(a + b, expected);
    }

    #[test]
    fn test_explode() {
        for (input, expected) in [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
            (
                "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]",
                "[[[[0,7],4],[15,[0,13]]],[1,1]]",
            ),
        ] {
            let number: SnailfishNumber = input.parse().unwrap();
            let expected: SnailfishNumber = expected.parse().unwrap();
            assert_eq!(number.explode().1, expected);
        }
    }

    #[test]
    fn test_split() {
        for (input, expected) in [
            (
                "[[[[0,7],4],[15,[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            ),
            (
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            ),
        ] {
            let number: SnailfishNumber = input.parse().unwrap();
            let expected: SnailfishNumber = expected.parse().unwrap();
            assert_eq!(number.split().1, expected);
        }
    }

    #[test]
    fn test_add_and_reduce() {
        let a: SnailfishNumber = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
        let b: SnailfishNumber = "[1,1]".parse().unwrap();

        assert_eq!(a + b, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap());
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_INPUT).unwrap(), 4_140);
        assert_eq!(part_1(INPUT).unwrap(), 3_551);
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_INPUT).unwrap(), 0);
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
