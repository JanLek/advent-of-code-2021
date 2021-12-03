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
    let num_columns = input.lines().next().unwrap().chars().count();

    let mut oxygen_generator_rating_candidates: Vec<_> = input.lines().collect();
    for column in 0..num_columns {
        let most_common_bit =
            most_common_bit(&oxygen_generator_rating_candidates, column).unwrap_or(b'1');

        oxygen_generator_rating_candidates
            .retain(|line| line.as_bytes()[column] == most_common_bit);
        if oxygen_generator_rating_candidates.len() == 1 {
            break;
        }
    }

    let mut co2_scrubber_rating_candidates: Vec<_> = input.lines().collect();
    for column in 0..num_columns {
        let least_common_bit =
            least_common_bit(&co2_scrubber_rating_candidates, column).unwrap_or(b'0');

        co2_scrubber_rating_candidates.retain(|line| line.as_bytes()[column] == least_common_bit);
        if co2_scrubber_rating_candidates.len() == 1 {
            break;
        }
    }

    println!(
        "O2: {:?}, CO2: {:?}",
        oxygen_generator_rating_candidates, co2_scrubber_rating_candidates
    );

    usize::from_str_radix(oxygen_generator_rating_candidates[0], 2).unwrap()
        * usize::from_str_radix(co2_scrubber_rating_candidates[0], 2).unwrap()
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
