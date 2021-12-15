#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::{collections::HashMap, ops::Index, str::FromStr};

fn part_1(input: &str) -> Result<usize, ParseError> {
    let cavern = Cavern::from_str(input)?;
    let mut cache = HashMap::new();
    Ok(lowest_total_risk(
        &cavern,
        cavern.num_rows - 1,
        cavern.num_columns - 1,
        &mut cache,
    ))
    // let path = lowest_risk_path(
    //     &cavern,
    //     cavern.num_rows - 1,
    //     cavern.num_columns - 1,
    //     &mut cache,
    // );
    // Ok(path.score(&cavern))
}

fn part_2(input: &str) -> Result<usize, ParseError> {
    let mut cavern = Cavern::from_str(input)?;
    cavern.expand();
    let mut cache = HashMap::new();
    Ok(lowest_total_risk(
        &cavern,
        cavern.num_rows - 1,
        cavern.num_columns - 1,
        &mut cache,
    ))
}

fn lowest_total_risk(
    cavern: &Cavern,
    row: usize,
    column: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if row == 0 && column == 0 {
        return 0;
    }

    if let Some(&cached_result) = cache.get(&(row, column)) {
        return cached_result;
    }

    let result = cavern
        .candidate_previous_locations(row, column)
        .map(|(row, column)| lowest_total_risk(cavern, row, column, cache))
        .min()
        .unwrap()
        + cavern[(row, column)] as usize;
    cache.insert((row, column), result);
    result
}

fn lowest_risk_path(
    cavern: &Cavern,
    row: usize,
    column: usize,
    cache: &mut HashMap<(usize, usize), Path>,
) -> Path {
    if row == 0 && column == 0 {
        return Path::start();
    }

    if let Some(cached_result) = cache.get(&(row, column)) {
        return cached_result.clone();
    }

    let result = cavern
        .candidate_previous_locations(row, column)
        .map(|(row, column)| lowest_risk_path(cavern, row, column, cache))
        .min_by(|a, b| a.score(cavern).cmp(&b.score(cavern)))
        .unwrap()
        .with_location((row, column));
    cache.insert((row, column), result.clone());
    result
}

#[derive(Debug, Eq, PartialEq)]
struct Cavern {
    num_rows: usize,
    num_columns: usize,
    risk_levels: Vec<u8>,
}

impl Cavern {
    fn expand(&mut self) {
        let num_rows = 5 * self.num_rows;
        let num_columns = 5 * self.num_columns;
        let mut risk_levels = vec![0; 25 * self.risk_levels.len()];

        for row in 0..num_rows {
            for column in 0..num_columns {
                let original_location = (row % self.num_rows, column % self.num_columns);
                let original_value = self[original_location];
                let increment = row / self.num_rows + column / self.num_columns;
                let mut new_value = original_value + u8::try_from(increment).unwrap();
                if new_value > 9 {
                    new_value -= 9;
                }
                risk_levels[row * num_columns + column] = new_value;
            }
        }

        self.num_rows = num_rows;
        self.num_columns = num_columns;
        self.risk_levels = risk_levels;
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn candidate_previous_locations(
        &self,
        row: usize,
        column: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let deltas = [(-1, 0), (0, -1)];
        deltas
            .into_iter()
            .filter_map(move |(delta_row, delta_column)| {
                let row = row as isize + delta_row;
                let column = column as isize + delta_column;
                if (0..self.num_rows as isize).contains(&row)
                    && (0..self.num_columns as isize).contains(&column)
                {
                    Some((row as usize, column as usize))
                } else {
                    None
                }
            })
    }
}

impl Index<(usize, usize)> for Cavern {
    type Output = u8;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        &self.risk_levels[row * self.num_columns + column]
    }
}

impl FromStr for Cavern {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let num_columns = input
            .bytes()
            .position(|byte| byte == b'\n')
            .ok_or(ParseError)?;
        let num_rows = input.len() / num_columns;
        let risk_levels: Vec<_> = input
            .bytes()
            .filter_map(|byte| match byte {
                b'\n' => None,
                b'1'..=b'9' => Some(Ok(byte - b'0')),
                _ => Some(Err(ParseError)),
            })
            .collect::<Result<_, _>>()?;
        if risk_levels.len() == num_rows * num_columns {
            Ok(Self {
                num_rows,
                num_columns,
                risk_levels,
            })
        } else {
            Err(ParseError)
        }
    }
}

impl std::fmt::Display for Cavern {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.num_rows {
            for column in 0..self.num_columns {
                write!(formatter, "{}", self[(row, column)])?;
            }
            writeln!(formatter);
        }
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Path {
    locations: Vec<(usize, usize)>,
}

impl Path {
    fn start() -> Self {
        Self {
            locations: vec![(0, 0)],
        }
    }

    fn with_location(&self, location: (usize, usize)) -> Self {
        let mut locations = self.locations.clone();
        locations.push(location);
        Self { locations }
    }

    fn contains(&self, location: &(usize, usize)) -> bool {
        self.locations.contains(location)
    }

    fn score(&self, cavern: &Cavern) -> usize {
        self.locations
            .iter()
            .skip(1) // Skip start location
            .map(|&location| cavern[location] as usize)
            .sum()
    }
}

#[derive(Debug)]
struct ParseError;

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    #[ignore]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_INPUT).unwrap(), 40);
        assert_eq!(part_1(INPUT).unwrap(), 621);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_INPUT).unwrap(), 315);
        assert!(
            part_2(INPUT).unwrap() < 2_918,
            "Should be less than 2918, but it is {}",
            part_2(INPUT).unwrap()
        );
        assert_eq!(part_2(INPUT).unwrap(), 0);
    }

    #[bench]
    #[ignore]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| part_1(INPUT).unwrap());
    }

    #[bench]
    #[ignore]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| part_2(INPUT).unwrap());
    }
}
