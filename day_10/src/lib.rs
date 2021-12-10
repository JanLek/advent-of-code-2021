#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::iter::{IntoIterator, Take};

const CHUNK_CHARACTERS: [(u8, u8); 4] = [(b'(', b')'), (b'[', b']'), (b'{', b'}'), (b'<', b'>')];

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
                .into_iter()
                .rev()
                .fold(0, |score, character| {
                    score * 5 + autocomplete_points(character)
                })
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn parse_line(line: &str) -> ParseResult {
    let mut closing_characters = PoorMansStack::new();
    for character in line.bytes() {
        if let Some(&(_, closing_character)) =
            CHUNK_CHARACTERS.iter().find(|&&(o, _)| o == character)
        {
            closing_characters.push(closing_character);
        } else if let Some(expected) = closing_characters.pop() {
            if expected != character {
                return ParseResult::InvalidCharacter(character);
            }
        } else {
            panic!("This line should never be reached");
        }
    }

    ParseResult::Incomplete(closing_characters)
}

enum ParseResult {
    InvalidCharacter(u8),
    Incomplete(PoorMansStack),
}

struct PoorMansStack {
    bytes: [u8; 100],
    size: usize,
}

impl PoorMansStack {
    fn new() -> Self {
        Self {
            bytes: [0; 100],
            size: 0,
        }
    }

    fn push(&mut self, byte: u8) {
        if self.size >= self.bytes.len() {
            panic!("Poor man's stack overflow")
        }

        self.bytes[self.size] = byte;
        self.size += 1;
    }

    fn pop(&mut self) -> Option<u8> {
        if self.size == 0 {
            return None;
        }

        self.size -= 1;
        Some(self.bytes[self.size])
    }
}

impl IntoIterator for PoorMansStack {
    type Item = u8;
    type IntoIter = Take<std::array::IntoIter<Self::Item, 100>>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes.into_iter().take(self.size)
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
