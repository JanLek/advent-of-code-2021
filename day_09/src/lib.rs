#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::{ops::Index, str::FromStr};

fn part_1<const R: usize, const C: usize>(input: &str) -> usize {
    let height_map: HeightMap<R, C> = input.parse().unwrap();
    height_map
        .low_points()
        .map(|coordinate| height_map[coordinate] as usize + 1)
        .sum()
}

fn part_2<const R: usize, const C: usize>(input: &str) -> usize {
    let mut height_map: HeightMap<R, C> = input.parse().unwrap();

    let mut largest_basins = LargestBasins::new();
    for low_point in height_map.clone().low_points() {
        largest_basins.add(height_map.basin_size(low_point));
    }
    largest_basins.product()
}

#[derive(Clone, Debug)]
struct HeightMap<const R: usize, const C: usize>([[u8; C]; R]);

impl<const R: usize, const C: usize> HeightMap<R, C> {
    fn basin_size(&mut self, coordinate: (usize, usize)) -> usize {
        let height = self[coordinate];
        self.0[coordinate.0][coordinate.1] = 9; // Mark as seen
        Self::adjacent_points(coordinate)
            .filter_map(|c| {
                let adjacent_height = self[c];
                if adjacent_height > height && adjacent_height != 9 {
                    Some(self.basin_size(c)) // Recursion FTW
                } else {
                    None
                }
            })
            .sum::<usize>()
            + 1
    }

    fn low_points(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..R)
            .flat_map(|row| (0..C).map(move |column| (row, column)))
            .filter(|&coordinate| {
                Self::adjacent_points(coordinate)
                    .all(|adjecent_point| self[adjecent_point] > self[coordinate])
            })
    }

    fn adjacent_points((row, column): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        [
            row.checked_sub(1).map(|r| (r, column)),
            if row + 1 < R {
                Some((row + 1, column))
            } else {
                None
            },
            column.checked_sub(1).map(|c| (row, c)),
            if column + 1 < C {
                Some((row, column + 1))
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

        for row in &mut grid {
            for height in row {
                *height = heights.next().unwrap();
            }
        }

        Ok(Self(grid))
    }
}

struct LargestBasins([usize; 3]);

impl LargestBasins {
    fn new() -> Self {
        Self([0; 3])
    }

    fn add(&mut self, basin_size: usize) {
        if basin_size > self.0[0] {
            self.0[2] = self.0[1];
            self.0[1] = self.0[0];
            self.0[0] = basin_size;
        } else if basin_size > self.0[1] {
            self.0[2] = self.0[1];
            self.0[1] = basin_size;
        } else if basin_size > self.0[2] {
            self.0[2] = basin_size;
        }
    }

    fn product(self) -> usize {
        self.0.into_iter().product()
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

        assert_eq!(part_2::<5, 10>(SAMPLE_INPUT), 1_134);
        assert_eq!(part_2::<100, 100>(INPUT), 1_142_757);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| part_1::<100, 100>(INPUT));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| part_2::<100, 100>(INPUT));
    }
}
