#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::{num::ParseIntError, str::FromStr};

fn part_1<const N: usize>(input: &str) -> Result<usize, InvalidInputError> {
    let school = parse_input::<N>(input)?;
    Ok(num_fish_after_n_days(school, 80))
}

fn part_2<const N: usize>(input: &str) -> Result<usize, InvalidInputError> {
    let school = parse_input::<N>(input)?;
    Ok(num_fish_after_n_days(school, 256))
}

fn num_fish_after_n_days<const N: usize>(school: [LanternFish; N], n: usize) -> usize {
    let mut school = School::from(school);
    for _ in 0..n {
        school.advance_one_day();
    }
    school.len()
}

fn parse_input<const N: usize>(input: &str) -> Result<[LanternFish; N], InvalidInputError> {
    let mut nums = input.split(',');
    let mut school = [LanternFish::default(); N];
    for fish in &mut school {
        let num = nums.next().ok_or(InvalidInputError)?;
        *fish = num.parse()?;
    }
    Ok(school)
}

#[derive(Debug)]
struct School([usize; 9]);

impl School {
    fn from<const N: usize>(school: [LanternFish; N]) -> Self {
        let mut fish_counts = [0; 9];
        for fish in school {
            fish_counts[fish.days_to_reproduce() as usize] += 1;
        }
        Self(fish_counts)
    }

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

#[derive(Clone, Copy, Debug, Default)]
struct LanternFish(u8);

impl LanternFish {
    fn days_to_reproduce(self) -> u8 {
        self.0
    }
}

impl FromStr for LanternFish {
    type Err = InvalidInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

#[derive(Debug)]
struct InvalidInputError;

impl From<ParseIntError> for InvalidInputError {
    fn from(_: ParseIntError) -> Self {
        Self
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
        assert_eq!(part_1::<5>(SAMPLE_INPUT).unwrap(), 5_934);
        assert_eq!(part_1::<300>(INPUT).unwrap(), 380_612);

        assert_eq!(part_2::<5>(SAMPLE_INPUT).unwrap(), 26_984_457_539);
        assert_eq!(part_2::<300>(INPUT).unwrap(), 1_710_166_656_900);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| part_1::<300>(INPUT));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| part_2::<300>(INPUT));
    }
}
