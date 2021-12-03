#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]

use std::cmp::Ordering::{Equal, Greater, Less};

fn part_1(input: &str) -> usize {
    let num_columns = input.lines().next().unwrap().chars().count();
    let mut counts: Vec<[usize; 2]> = Vec::with_capacity(num_columns);
    counts.resize(num_columns, [0, 0]);

    for line in input.lines() {
        for (i, bit) in line.bytes().map(|b| usize::from(b - b'0')).enumerate() {
            counts[i][bit] += 1;
        }
    }

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    for count in counts {
        if count[0] > count[1] {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        } else {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        }
    }

    usize::from_str_radix(&gamma_rate, 2).unwrap()
        * usize::from_str_radix(&epsilon_rate, 2).unwrap()
}

fn part_2(input: &str) -> usize {
    let num_columns = input.lines().next().unwrap().chars().count();

    let mut most_common_bits: Vec<Option<u8>> = Vec::with_capacity(num_columns);
    let mut least_common_bits: Vec<Option<u8>> = Vec::with_capacity(num_columns);
    for column in 0..num_columns {
        let mut num_zeros = 0;
        let mut num_ones = 0;
        for line in input.lines() {
            match line.as_bytes()[column] {
                b'0' => {
                    num_zeros += 1;
                }
                b'1' => {
                    num_ones += 1;
                }
                _ => panic!(),
            }
        }

        most_common_bits.push(match num_zeros.cmp(&num_ones) {
            Less => Some(b'1'),
            Equal => None,
            Greater => Some(b'0'),
        });

        least_common_bits.push(match num_zeros.cmp(&num_ones) {
            Less => Some(b'0'),
            Equal => None,
            Greater => Some(b'1'),
        });
    }

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
        let least_common_bit = match most_common_bit(&co2_scrubber_rating_candidates, column) {
            Some(b'0') => b'1',
            Some(b'1') => b'0',
            None => b'0',
            Some(_) => panic!(),
        };

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
