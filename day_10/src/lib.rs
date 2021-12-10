#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter_map(first_illegal_character)
        .map(points)
        .sum()
}

fn part_2(_input: &str) -> usize {
    todo!()
}

const CHUNK_CHARACTERS: [(u8, u8); 4] = [(b'(', b')'), (b'[', b']'), (b'{', b'}'), (b'<', b'>')];

fn first_illegal_character(line: &str) -> Option<u8> {
    let mut closing_characters = Vec::with_capacity(line.len());
    let mut seen = String::new();
    for character in line.bytes() {
        seen.push(char::from(character));
        if let Some((_, closing_character)) =
            CHUNK_CHARACTERS.iter().find(|&&(o, _)| o == character)
        {
            closing_characters.push(closing_character);
        } else if let Some(&expected) = closing_characters.pop() {
            if expected != character {
                return Some(character);
            }
        }
    }
    None
}

fn points(byte: u8) -> usize {
    match byte {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
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
    #[ignore]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_INPUT), 0);
        // assert_eq!(part_2(INPUT), 0);
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
