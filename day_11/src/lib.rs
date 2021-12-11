#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(bool_to_option, test)]

use std::{
    fmt::{Display, Formatter, Result as FormatResult},
    ops::{Index, IndexMut},
    str::FromStr,
};

fn part_1(input: &str) -> usize {
    let mut grid: Grid = input.parse().unwrap();
    let mut num_flashes = 0;
    for _ in 0..100 {
        num_flashes += grid.simulate_step();
    }
    num_flashes
}

fn part_2(input: &str) -> usize {
    let mut grid: Grid = input.parse().unwrap();
    for step in 0.. {
        if grid.simulate_step() == 100 {
            return step + 1;
        }
    }
    panic!("The bioluminescent dumbo octopuses did not synchronise their flash!")
}

struct Grid([[u8; 10]; 10]);

impl Grid {
    fn simulate_step(&mut self) -> usize {
        for octupus in self.all_octopuses() {
            *octupus += 1;
        }

        let mut num_flashed = 0;
        let mut will_flash = self.will_flash_next();
        while !will_flash.is_empty() {
            for coordinate in will_flash {
                self[coordinate] = 0;
                num_flashed += 1;
                for adjacent_coordinate in adjacent_coordinates(coordinate) {
                    if self[adjacent_coordinate] != 0 {
                        self[adjacent_coordinate] += 1;
                    }
                }
            }
            will_flash = self.will_flash_next();
        }
        num_flashed
    }

    fn all_octopuses(&mut self) -> impl Iterator<Item = &mut u8> {
        self.0.iter_mut().flat_map(|row| row.iter_mut())
    }

    fn will_flash_next(&self) -> Vec<Coordinate> {
        all_coordinates()
            .filter(|&coordinate| self[coordinate] != 0 && self[coordinate] > 9)
            .collect()
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid: [[u8; 10]; 10] = Default::default();
        let mut octopuses = input.lines().flat_map(str::bytes).map(|byte| byte - b'0');
        for row in &mut grid {
            for number in row {
                *number = octopuses.next().unwrap();
            }
        }
        Ok(Self(grid))
    }
}

impl Index<Coordinate> for Grid {
    type Output = u8;

    fn index(&self, (row, column): Coordinate) -> &Self::Output {
        &self.0[row][column]
    }
}

impl IndexMut<Coordinate> for Grid {
    fn index_mut(&mut self, (row, column): Coordinate) -> &mut Self::Output {
        &mut self.0[row][column]
    }
}

impl Display for Grid {
    fn fmt(&self, formatter: &mut Formatter) -> FormatResult {
        for line in self.0 {
            let as_str = unsafe {
                String::from_utf8_unchecked(
                    line.iter()
                        .map(|octopus| octopus + b'0')
                        .collect::<Vec<_>>(),
                )
            };
            writeln!(formatter, "{}", as_str)?;
        }
        Ok(())
    }
}

type Coordinate = (usize, usize);

fn all_coordinates() -> impl Iterator<Item = Coordinate> {
    (0..10).flat_map(|row| (0..10).map(move |column| (row, column)))
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
fn adjacent_coordinates((row, column): Coordinate) -> impl Iterator<Item = Coordinate> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .into_iter()
    .filter_map(move |(delta_row, delta_column)| {
        let row = row as isize + delta_row;
        let column = column as isize + delta_column;
        if (0..10).contains(&row) && (0..10).contains(&column) {
            Some((row as usize, column as usize))
        } else {
            None
        }
    })
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
        assert_eq!(part_1(SAMPLE_INPUT), 1_656);
        assert_eq!(part_1(INPUT), 1_694);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_INPUT), 195);
        assert_eq!(part_2(INPUT), 346);
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
