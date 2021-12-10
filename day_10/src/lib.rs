#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| match parse_line(line) {
            ParseResult::InvalidCharacter(c) => Some(c),
            ParseResult::Incomplete(_) => None,
        })
        .map(syntax_error_points)
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut scores: Vec<_> = input
        .lines()
        .filter_map(|line| match parse_line(line) {
            ParseResult::InvalidCharacter(_) => None,
            ParseResult::Incomplete(completion_characters) => Some(completion_characters),
        })
        .map(|completion_characters| {
            completion_characters
                .iter()
                .rev()
                .fold(0, |score, &character| {
                    score * 5 + autocomplete_points(character)
                })
        })
        .collect();
    let middle = scores.len() / 2;
    *scores.select_nth_unstable(middle).1
}

fn parse_line(line: &str) -> ParseResult {
    let mut closing_characters = Vec::with_capacity(line.len());
    for character in line.bytes() {
        if let Some(closing_character) = closing_character(character) {
            closing_characters.push(closing_character);
        } else if let Some(expected) = closing_characters.pop() {
            if expected != character {
                return ParseResult::InvalidCharacter(character);
            }
        } else {
            unimplemented!("Correct lines are not supported");
        }
    }

    ParseResult::Incomplete(closing_characters)
}

enum ParseResult {
    InvalidCharacter(u8),
    Incomplete(Vec<u8>),
}

fn closing_character(byte: u8) -> Option<u8> {
    match byte {
        b'(' => Some(b')'),
        b'[' => Some(b']'),
        b'{' => Some(b'}'),
        b'<' => Some(b'>'),
        _ => None,
    }
}

fn syntax_error_points(byte: u8) -> usize {
    match byte {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => panic!(),
    }
}

fn autocomplete_points(byte: u8) -> usize {
    match byte {
        b')' => 1,
        b']' => 2,
        b'}' => 3,
        b'>' => 4,
        _ => panic!(),
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
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_INPUT), 26_397);
        assert_eq!(part_1(INPUT), 462_693);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_INPUT), 288_957);
        assert_eq!(part_2(INPUT), 3_094_671_161);
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
