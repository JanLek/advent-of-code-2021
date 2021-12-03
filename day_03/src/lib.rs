#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]

fn part_1(input: &str) -> usize {
    let num_columns = input.lines().next().unwrap().chars().count();
    let numbers: Vec<_> = input.lines().collect();
    let gamma_rate = find_rate(num_columns, &numbers, most_common_bit);
    let epsilon_rate = find_rate(num_columns, &numbers, least_common_bit);
    gamma_rate * epsilon_rate
}

fn part_2(input: &str) -> usize {
    let num_columns = input.lines().next().unwrap().chars().count();
    let numbers = input.lines().collect::<Vec<_>>();
    let oxygen_generator_rating = find_rating(num_columns, &numbers, most_common_bit);
    let co2_scrubber_rating = find_rating(num_columns, &numbers, least_common_bit);

    oxygen_generator_rating * co2_scrubber_rating
}

fn find_rate(num_columns: usize, numbers: &[&str], choose_bit: fn(&[&str], usize) -> u8) -> usize {
    usize::from_str_radix(
        &(0..num_columns)
            .map(|column| char::from(choose_bit(numbers, column)))
            .collect::<String>(),
        2,
    )
    .unwrap()
}

fn find_rating(
    num_columns: usize,
    numbers: &[&str],
    bit_criteria: impl Fn(&[&str], usize) -> u8,
) -> usize {
    let mut candidate_numbers = Vec::from(numbers);
    for column in 0..num_columns {
        let bit_criteria = bit_criteria(&candidate_numbers, column);
        candidate_numbers.retain(|line| line.as_bytes()[column] == bit_criteria);
        if candidate_numbers.len() == 1 {
            break;
        }
    }

    usize::from_str_radix(candidate_numbers[0], 2).unwrap()
}

fn most_common_bit(numbers: &[&str], column: usize) -> u8 {
    let mut num_zeros = 0;
    let mut num_ones = 0;

    for number in numbers {
        match number.as_bytes()[column] {
            b'0' => num_zeros += 1,
            b'1' => num_ones += 1,
            _ => panic!(),
        }
    }

    if num_zeros > num_ones {
        b'0'
    } else {
        b'1'
    }
}

fn least_common_bit(numbers: &[&str], column: usize) -> u8 {
    match most_common_bit(numbers, column) {
        b'0' => b'1',
        b'1' => b'0',
        _ => panic!(),
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
