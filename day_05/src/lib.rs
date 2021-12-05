#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(int_abs_diff)]

use std::{
    cmp::max,
    iter::Rev,
    num::ParseIntError,
    ops::{Index, IndexMut, RangeInclusive},
    str::FromStr,
};

fn part_1<const N: usize>(input: &str) -> Result<usize, InvalidInputError> {
    let lines = parse_input::<N>(input)?;
    let mut grid = Grid::new();

    for coordinate in lines
        .iter()
        .filter(|line| !line.is_diagonal())
        .flat_map(Line::coordinates)
    {
        grid[coordinate] += 1;
    }

    Ok(grid.num_overlaps())
}

fn part_2<const N: usize>(input: &str) -> Result<usize, InvalidInputError> {
    let lines = parse_input::<N>(input)?;
    let mut grid = Grid::new();

    for coordinate in lines.iter().flat_map(Line::coordinates) {
        grid[coordinate] += 1;
    }

    Ok(grid.num_overlaps())
}

fn parse_input<const N: usize>(input: &str) -> Result<[Line; N], InvalidInputError> {
    let mut input_lines = input.lines();
    let mut lines = [Line::default(); N];

    for line in &mut lines {
        *line = input_lines.next().ok_or(InvalidInputError)?.parse()?;
    }

    Ok(lines)
}

struct Grid([u8; 1_000_000]);

impl Grid {
    const NUM_ROWS: usize = 1_000;
    const NUM_COLUMS: usize = 1_000;

    fn new() -> Self {
        Self([0; Self::NUM_ROWS * Self::NUM_COLUMS])
    }

    fn num_overlaps(&self) -> usize {
        self.0.iter().filter(|&&x| x > 1).count()
    }
}

impl Index<Coordinate> for Grid {
    type Output = u8;

    fn index(&self, Coordinate(x, y): Coordinate) -> &Self::Output {
        &self.0[(y * Self::NUM_COLUMS + x) as usize]
    }
}

impl IndexMut<Coordinate> for Grid {
    fn index_mut(&mut self, Coordinate(x, y): Coordinate) -> &mut Self::Output {
        &mut self.0[(y * Self::NUM_COLUMS + x)]
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Line(Coordinate, Coordinate);

impl Line {
    fn is_diagonal(&self) -> bool {
        let Self(Coordinate(x1, y1), Coordinate(x2, y2)) = self;
        // Assumes lines can only be horizontal, vertical, or diagonal
        x1 != x2 && y1 != y2
    }

    fn coordinates(&self) -> impl Iterator<Item = Coordinate> {
        let Self(Coordinate(x1, y1), Coordinate(x2, y2)) = *self;
        let mut x = Range::new(x1, x2);
        let mut y = Range::new(y1, y2);
        let num_coordinates = max(x1.abs_diff(x2) + 1, y1.abs_diff(y2) + 1);

        (0..num_coordinates)
            .map(move |_| Coordinate(x.next().unwrap_or(x2), y.next().unwrap_or(y2)))
    }
}

impl FromStr for Line {
    type Err = InvalidInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, finish) = s.split_once(" -> ").ok_or(InvalidInputError)?;
        Ok(Self(start.parse()?, finish.parse()?))
    }
}

// Need custom range type, in case start < finish
enum Range {
    Increasing(RangeInclusive<usize>),
    Decreasing(Rev<RangeInclusive<usize>>),
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        if start <= end {
            Self::Increasing(start..=end)
        } else {
            Self::Decreasing((end..=start).rev())
        }
    }
}

impl Iterator for Range {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        match self {
            Self::Increasing(range) => range.next(),
            Self::Decreasing(range) => range.next(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Coordinate(usize, usize);

impl FromStr for Coordinate {
    type Err = InvalidInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(InvalidInputError)?;
        Ok(Self(x.parse()?, y.parse()?))
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
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test() {
        assert_eq!(part_1::<10>(SAMPLE_INPUT).unwrap(), 5);
        assert_eq!(part_1::<500>(INPUT).unwrap(), 5_084);

        assert_eq!(part_2::<10>(SAMPLE_INPUT).unwrap(), 12);
        assert_eq!(part_2::<500>(INPUT).unwrap(), 17_882);
    }
}
