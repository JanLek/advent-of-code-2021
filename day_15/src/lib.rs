#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::{collections::HashMap, ops::Index, str::FromStr};

fn part_1(input: &str) -> Result<u32, ParseError> {
    let cavern = Cavern::from_str(input)?;
    let mut cache = HashMap::new();
    Ok(lowest_total_risk(
        &cavern,
        cavern.num_rows - 1,
        cavern.num_columns - 1,
        &mut cache,
    ))
}

fn part_2(_input: &str) -> Result<usize, ParseError> {
    todo!()
}

fn lowest_total_risk(
    cavern: &Cavern,
    row: usize,
    column: usize,
    cache: &mut HashMap<(usize, usize), u32>,
) -> u32 {
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
        + u32::from(cavern[(row, column)]);
    cache.insert((row, column), result);
    result
}

#[derive(Debug)]
struct Cavern {
    num_rows: usize,
    num_columns: usize,
    risk_levels: Vec<u8>,
}

impl Cavern {
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
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_INPUT).unwrap(), 40);
        assert!(part_1(INPUT).unwrap() > 620);
        // assert_eq!(part_1(INPUT).unwrap(), 0);
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_INPUT).unwrap(), 0);
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
