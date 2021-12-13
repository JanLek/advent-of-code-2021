#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::{
    collections::HashSet,
    fmt::{Display, Formatter, Result as FormatResult},
    num::ParseIntError,
    ops::Index,
    str::FromStr,
};

fn part_1(input: &str) -> Result<usize, ParseError> {
    let (coordinates, fold_alongs) = parse_input(input)?;
    let mut grid: Grid = coordinates.into();
    grid.fold(fold_alongs[0]);
    Ok(grid.num_points())
}

fn part_2(input: &str) -> Result<usize, ParseError> {
    let (coordinates, fold_alongs) = parse_input(input)?;
    let mut grid: Grid = coordinates.into();
    for fold_along in fold_alongs {
        grid.fold(fold_along);
    }
    println!("{}", grid);
    Ok(grid.num_points())
}

fn parse_input(input: &str) -> Result<(Vec<Coordinate>, Vec<FoldAlong>), ParseError> {
    let (coordinates, fold_alongs) = input.split_once("\n\n").ok_or(ParseError)?;
    Ok((
        coordinates
            .lines()
            .map(str::parse)
            .collect::<Result<_, _>>()?,
        fold_alongs
            .lines()
            .map(str::parse)
            .collect::<Result<_, _>>()?,
    ))
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl FromStr for Coordinate {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParseError)?;
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum FoldAlong {
    X(usize),
    Y(usize),
}

impl FromStr for FoldAlong {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("fold along ").ok_or(ParseError)?;
        match s.split_once('=').ok_or(ParseError)? {
            ("x", x) => Ok(Self::X(x.parse()?)),
            ("y", y) => Ok(Self::Y(y.parse()?)),
            _ => Err(ParseError),
        }
    }
}

#[derive(Debug)]
struct ParseError;

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        Self
    }
}

struct Grid {
    num_columns: usize,
    num_rows: usize,
    points: Vec<Coordinate>,
}

impl Grid {
    fn fold(&mut self, fold_along: FoldAlong) {
        match fold_along {
            FoldAlong::X(x_fold) => {
                for point in &mut self.points {
                    if point.x > x_fold {
                        point.x = x_fold - (point.x - x_fold);
                    }
                }
                self.num_columns = x_fold;
            }
            FoldAlong::Y(y_fold) => {
                for point in &mut self.points {
                    if point.y > y_fold {
                        point.y = y_fold - (point.y - y_fold);
                    }
                    self.num_rows = y_fold;
                }
            }
        }
    }

    fn num_points(&self) -> usize {
        self.points.iter().copied().collect::<HashSet<_>>().len()
    }
}

impl From<Vec<Coordinate>> for Grid {
    fn from(points: Vec<Coordinate>) -> Self {
        Self {
            num_columns: points
                .iter()
                .map(|coordinate| coordinate.x)
                .max()
                .unwrap_or_default()
                + 1,
            num_rows: points
                .iter()
                .map(|coordinate| coordinate.y)
                .max()
                .unwrap_or_default()
                + 1,
            points,
        }
    }
}

impl Index<Coordinate> for Grid {
    type Output = bool;

    fn index(&self, coordinate: Coordinate) -> &Self::Output {
        if self.points.contains(&coordinate) {
            &true
        } else {
            &false
        }
    }
}

impl Display for Grid {
    fn fmt(&self, formatter: &mut Formatter) -> FormatResult {
        for y in 0..self.num_rows {
            for x in 0..self.num_columns {
                if self[Coordinate { x, y }] {
                    write!(formatter, "#")?;
                } else {
                    write!(formatter, ".")?;
                }
            }
            writeln!(formatter);
        }

        Ok(())
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
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_INPUT).unwrap(), 17);
        assert_eq!(part_1(INPUT).unwrap(), 687);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_INPUT).unwrap(), 16);
        assert_eq!(part_2(INPUT).unwrap(), 98); // Code: FGKCKBZG
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| part_1(INPUT).unwrap());
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| part_2(INPUT).unwrap());
    }
}
