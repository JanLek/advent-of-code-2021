#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]

const INPUT: &str = include_str!("input.txt");

fn part_1(input: &str) -> i32 {
    todo!();
}

fn part_2(input: &str) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample-input.txt");

    #[test]
    #[ignore]
    fn test_sample_1() {
        assert_eq!(part_1(SAMPLE_INPUT), Default::default());
    }

    #[test]
    #[ignore]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), Default::default());
    }

    #[test]
    #[ignore]
    fn test_sample_2() {
        assert_eq!(part_2(SAMPLE_INPUT), Default::default());
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), Default::default());
    }
}
