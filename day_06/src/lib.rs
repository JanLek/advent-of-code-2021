#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::str::FromStr;

fn part_1(input: &str) -> usize {
    num_fish_after_n_days(input.parse().unwrap(), 80)
}

fn part_2(input: &str) -> usize {
    num_fish_after_n_days(input.parse().unwrap(), 256)
}

fn num_fish_after_n_days(mut school: School, n: usize) -> usize {
    for _ in 0..n {
        school.advance_one_day();
    }
    school.len()
}

#[derive(Debug)]
struct School([usize; 9]);

impl School {
    fn advance_one_day(&mut self) {
        let num_reproductions = self.0[0];
        for days in 0..8 {
            self.0[days] = self.0[days + 1];
        }
        self.0[6] += num_reproductions;
        self.0[8] = num_reproductions;
    }

    fn len(&self) -> usize {
        self.0.iter().sum()
    }
}

impl FromStr for School {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let mut counts = [0; 9];
        for i in (0..bytes.len()).step_by(2) {
            counts[(bytes[i] - b'0') as usize] += 1;
        }
        Ok(Self(counts))
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
    fn test() {
        assert_eq!(part_1(SAMPLE_INPUT), 5_934);
        assert_eq!(part_1(INPUT), 380_612);

        assert_eq!(part_2(SAMPLE_INPUT), 26_984_457_539);
        assert_eq!(part_2(INPUT), 1_710_166_656_900);
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
