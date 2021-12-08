#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::collections::HashMap;

const NUM_SEGMENTS: [u8; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 8];

fn part_1(input: &str) -> usize {
    parse_input(input)
        .flat_map(|(_, digit_output_values)| digit_output_values)
        .filter(|digit_output_value| [2, 4, 3, 7].contains(&digit_output_value.len()))
        .count()
}

fn part_2(input: &str) -> usize {
    parse_input(input).map(deduce_output_value).sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = ([String; 10], [String; 4])> + '_ {
    input.lines().map(|line| {
        let (signal_patterns, digit_output_values) = line.split_once(" | ").unwrap();
        (
            to_array_of_sorted_strings(signal_patterns),
            to_array_of_sorted_strings(digit_output_values),
        )
    })
}

fn to_array_of_sorted_strings<const N: usize>(input: &str) -> [String; N]
where
    [String; N]: Default,
{
    let mut iterator = input.split(' ').map(|s| {
        let mut bytes = s.as_bytes().to_vec();
        bytes.sort_unstable();
        unsafe { String::from_utf8_unchecked(bytes) }
    });

    let mut array: [String; N] = Default::default();
    for element in &mut array {
        *element = iterator.next().unwrap();
    }
    array
}

fn deduce_output_value(
    (signal_patterns, digit_output_values): ([String; 10], [String; 4]),
) -> usize {
    let mut signal_patterns_by_len: HashMap<_, _> = HashMap::with_capacity(6);
    for pattern in signal_patterns {
        signal_patterns_by_len
            .entry(pattern.len())
            .or_insert_with(|| Vec::with_capacity(3))
            .push(pattern);
    }

    let one = &signal_patterns_by_len.get(&2).unwrap()[0];
    let four = &signal_patterns_by_len.get(&4).unwrap()[0];
    let seven = &signal_patterns_by_len.get(&3).unwrap()[0];
    let eight = &signal_patterns_by_len.get(&7).unwrap()[0];

    let two_three_or_five = signal_patterns_by_len.get(&5).unwrap();
    let three = two_three_or_five
        .iter()
        .find(|pattern| one.chars().all(|c| pattern.contains(c)))
        .unwrap();
    let four_sans_one: Vec<_> = four.chars().filter(|&c| !one.contains(c)).collect();
    let five = two_three_or_five
        .iter()
        .find(|&pattern| pattern != three && four_sans_one.iter().all(|&c| pattern.contains(c)))
        .unwrap();
    let two = two_three_or_five
        .iter()
        .find(|&pattern| pattern != three && pattern != five)
        .unwrap();

    let zero_six_or_nine = signal_patterns_by_len.get(&6).unwrap();
    let six = zero_six_or_nine
        .iter()
        .find(|pattern| !one.chars().all(|c| pattern.contains(c)))
        .unwrap();
    let nine = zero_six_or_nine
        .iter()
        .find(|&pattern| pattern != six && four.chars().all(|c| pattern.contains(c)))
        .unwrap();
    let zero = zero_six_or_nine
        .iter()
        .find(|&pattern| pattern != six && pattern != nine)
        .unwrap();

    let find_digit = |s: &String| {
        if s == zero {
            0
        } else if s == one {
            1
        } else if s == two {
            2
        } else if s == three {
            3
        } else if s == four {
            4
        } else if s == five {
            5
        } else if s == six {
            6
        } else if s == seven {
            7
        } else if s == eight {
            8
        } else if s == nine {
            9
        } else {
            dbg!(s);
            panic!()
        }
    };

    digit_output_values
        .iter()
        .rev()
        .enumerate()
        .fold(0, |result, (index, digit_output_value)| {
            result + find_digit(digit_output_value) * 10_usize.pow(index as u32)
        })
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test() {
        assert_eq!(part_1(SAMPLE_INPUT), 26);
        assert_eq!(part_1(INPUT), 504);

        assert_eq!(part_2("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), 5353);
        assert_eq!(part_2(SAMPLE_INPUT), 61_229);
        assert_eq!(part_2(INPUT), 1_073_431);
    }

    #[bench]
    #[ignore]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| part_1(INPUT));
    }

    #[bench]
    #[ignore]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| part_2(INPUT));
    }
}
