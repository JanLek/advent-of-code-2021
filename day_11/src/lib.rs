#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

fn part_1(_input: &str) -> usize {
    todo!()
}

fn part_2(_input: &str) -> usize {
    todo!()
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
        assert_eq!(part_1(SAMPLE_INPUT), 0);
        // assert_eq!(part_1(INPUT), 0);
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_INPUT), 0);
        assert_eq!(part_2(INPUT), 0);
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
