#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]

use std::cmp::Ordering::{Equal, Greater, Less};

fn part_1(input: &str) -> usize {
    let num_columns = input.lines().next().unwrap().chars().count();
    let numbers: Vec<_> = input.lines().collect();
    let gamma_rate: usize = usize::from_str_radix(
        &(0..num_columns)
            .map(|column| char::from(most_common_bit(&numbers, column).unwrap()))
            .collect::<String>(),
        2,
    )
    .unwrap();

    let epsilon_rate: usize = usize::from_str_radix(
        &(0..num_columns)
            .map(|column| char::from(least_common_bit(&numbers, column).unwrap()))
            .collect::<String>(),
        2,
    )
    .unwrap();

    gamma_rate * epsilon_rate
}

fn part_2(input: &str) -> usize {
    let numbers = input.lines().collect::<Vec<_>>();

    let oxygen_generator_rating = find_rating(&numbers, |candidate_numbers, column| {
        most_common_bit(candidate_numbers, column).unwrap_or(b'1')
    });

    let co2_scrubber_rating = find_rating(&numbers, |candidate_numbers, column| {
        least_common_bit(candidate_numbers, column).unwrap_or(b'0')
    });

    usize::from_str_radix(&oxygen_generator_rating, 2).unwrap()
        * usize::from_str_radix(&co2_scrubber_rating, 2).unwrap()
}

fn find_rating(numbers: &[&str], make_bit_criteria: impl Fn(&[&str], usize) -> u8) -> String {
    let num_columns = numbers[0].len();
    let mut candidate_numbers = Vec::from(numbers);
    for column in 0..num_columns {
        let bit_criteria = make_bit_criteria(&candidate_numbers, column);
        candidate_numbers.retain(|line| line.as_bytes()[column] == bit_criteria);
        if candidate_numbers.len() == 1 {
            break;
        }
    }

    candidate_numbers[0].into()
}

fn most_common_bit(numbers: &[&str], column: usize) -> Option<u8> {
    let mut num_zeros = 0;
    let mut num_ones = 0;

    for number in numbers {
        match number.as_bytes()[column] {
            b'0' => num_zeros += 1,
            b'1' => num_ones += 1,
            _ => panic!(),
        }
    }

    match num_zeros.cmp(&num_ones) {
        Greater => Some(b'0'),
        Equal => None,
        Less => Some(b'1'),
    }
}

fn least_common_bit(numbers: &[&str], column: usize) -> Option<u8> {
    match most_common_bit(numbers, column) {
        Some(b'0') => Some(b'1'),
        Some(b'1') => Some(b'0'),
        None => None,
        Some(_) => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample-input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_sample_1() {
        assert_eq!(part_1(SAMPLE_INPUT), 198);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 3_277_364);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(part_2(SAMPLE_INPUT), 230);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 5_736_383);
    }
}
