#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::{ops::Index, str::FromStr};

fn part_1<const R: usize, const C: usize>(input: &str) -> usize {
    let height_map: HeightMap<R, C> = input.parse().unwrap();
    height_map
        .low_points()
        .map(|point| (point + 1) as usize)
        .sum()
}

fn part_2<const R: usize, const C: usize>(input: &str) -> usize {
    todo!();
}

#[derive(Debug)]
struct HeightMap<const R: usize, const C: usize>([[u8; C]; R]);

impl<const R: usize, const C: usize> HeightMap<R, C> {
    fn low_points(&self) -> impl Iterator<Item = u8> + '_ {
        (0..R)
            .flat_map(|row| (0..C).map(move |column| (row, column)))
            .filter(|&coordinate| {
                self.adjecent_points(coordinate)
                    .all(|adjecent_point| adjecent_point > self[coordinate])
            })
            .map(|coordinate| self[coordinate])
    }

    fn adjecent_points(&self, (row, column): (usize, usize)) -> impl Iterator<Item = u8> {
        [
            row.checked_sub(1).map(|r| self[(r, column)]),
            if row + 1 < R {
                Some(self[(row + 1, column)])
            } else {
                None
            },
            column.checked_sub(1).map(|c| self[(row, c)]),
            if column + 1 < C {
                Some(self[(row, column + 1)])
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
    }
}

impl<const R: usize, const C: usize> Index<(usize, usize)> for HeightMap<R, C> {
    type Output = u8;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        &self.0[row][column]
    }
}

impl<const R: usize, const C: usize> FromStr for HeightMap<R, C> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = [[0; C]; R];

        let mut heights = s.lines().flat_map(str::bytes).map(|byte| byte - b'0');
        for row in 0..R {
            for column in 0..C {
                grid[row][column] = heights.next().unwrap();
            }
        }

        Ok(Self(grid))
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
        assert_eq!(part_1::<5, 10>(SAMPLE_INPUT), 15);
        assert_eq!(part_1::<100, 100>(INPUT), 537);

        // assert_eq!(part_2::<5, 10>(SAMPLE_INPUT), 0);
        // assert_eq!(part_2::<100, 100>(INPUT), 0);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| part_1::<100, 100>(INPUT));
    }

    #[bench]
    #[ignore]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| part_2::<100, 100>(INPUT));
    }
}
