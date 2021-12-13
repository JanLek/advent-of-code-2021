#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::{
    fmt::{Display, Formatter, Result as FormatResult},
    num::ParseIntError,
    ops::{Index, IndexMut},
    str::FromStr,
};

fn part_1(input: &str) -> Result<usize, ParseError> {
    let (coordinates, fold_alongs) = parse_input(input)?;
    let mut grid: Grid = coordinates.into();
    grid.fold(fold_alongs[0]);
    Ok(grid.num_points())
}

fn part_2(_input: &str) -> usize {
    todo!();
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

#[derive(Debug)]
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
    points: Vec<bool>,
}

impl Grid {
    fn fold(&mut self, fold_along: FoldAlong) {
        match fold_along {
            FoldAlong::X(x_fold) => {
                let new_num_columns = x_fold;
                let mut new_points = vec![false; new_num_columns * self.num_rows];

                for y in 0..self.num_rows {
                    for x in 0..x_fold {
                        if self[Coordinate { x, y }] {
                            new_points[y * new_num_columns + x] = true;
                        }
                    }

                    for x in x_fold + 1..self.num_columns {
                        if self[Coordinate { x, y }] {
                            let new_x = x_fold - (x - x_fold);
                            new_points[y * new_num_columns + new_x] = true;
                        }
                    }
                }
                self.num_columns = new_num_columns;
                self.points = new_points;
            }
            FoldAlong::Y(y_fold) => {
                for y in y_fold + 1..self.num_rows {
                    for x in 0..self.num_columns {
                        if self[Coordinate { x, y }] {
                            self[Coordinate {
                                x,
                                y: y_fold - (y - y_fold),
                            }] = true;
                        }
                    }
                }
                self.num_rows = y_fold;
                self.points.resize(self.num_columns * self.num_rows, false);
            }
        }
    }

    fn num_points(&self) -> usize {
        self.points.iter().filter(|&&point| point).count()
    }
}

impl From<Vec<Coordinate>> for Grid {
    fn from(coordinates: Vec<Coordinate>) -> Self {
        let num_columns = coordinates
            .iter()
            .map(|coordinate| coordinate.x)
            .max()
            .unwrap_or_default()
            + 1;
        let num_rows = coordinates
            .iter()
            .map(|coordinate| coordinate.y)
            .max()
            .unwrap_or_default()
            + 1;

        let mut grid = Self {
            num_columns,
            num_rows,
            points: vec![false; num_columns * num_rows],
        };
        for coordinate in coordinates {
            grid[coordinate] = true;
        }
        grid
    }
}

impl Index<Coordinate> for Grid {
    type Output = bool;

    fn index(&self, coordinate: Coordinate) -> &Self::Output {
        &self.points[coordinate.y * self.num_columns + coordinate.x]
    }
}

impl IndexMut<Coordinate> for Grid {
    fn index_mut(&mut self, coordinate: Coordinate) -> &mut Self::Output {
        &mut self.points[coordinate.y * self.num_columns + coordinate.x]
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
