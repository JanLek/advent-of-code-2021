#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(array_chunks)]

use std::str::FromStr;

fn part_1(input: &str) -> Result<usize, BingoError> {
    let (numbers, boards) = parse_input(input)?;

    for i in 0..numbers.len() {
        let marked_numbers = &numbers[0..=i];
        if let Some(winning_board) = boards.iter().find(|board| board.wins(marked_numbers)) {
            return winning_board.score(marked_numbers);
        }
    }

    Err(BingoError::NoWinningBoard)
}

fn part_2(input: &str) -> Result<usize, BingoError> {
    let (numbers, mut boards) = parse_input(input)?;

    for i in 0..numbers.len() {
        let marked_numbers = &numbers[0..=i];
        if boards.len() > 1 {
            boards.retain(|board| !board.wins(marked_numbers));
        } else if boards[0].wins(marked_numbers) {
            return boards[0].score(marked_numbers);
        }
    }

    Err(BingoError::NoWinningBoard)
}

fn parse_input(input: &str) -> Result<(Vec<usize>, Vec<BingoBoard>), BingoError> {
    let mut parts = input.split("\n\n");
    let numbers: Vec<usize> = parts
        .next()
        .ok_or(BingoError::Parse)?
        .split(',')
        .map(|n| n.parse().map_err(|_| BingoError::Parse))
        .collect::<Result<_, _>>()?;
    let boards: Vec<BingoBoard> = parts.map(str::parse).collect::<Result<_, _>>()?;
    Ok((numbers, boards))
}

struct BingoBoard([usize; 25]);

impl BingoBoard {
    fn wins(&self, marked_numbers: &[usize]) -> bool {
        let is_fully_marked =
            |numbers: [usize; 5]| numbers.iter().all(|n| marked_numbers.contains(n));
        self.rows().any(is_fully_marked) || self.columns().any(is_fully_marked)
    }

    fn rows(&self) -> impl Iterator<Item = [usize; 5]> + '_ {
        self.0.array_chunks::<5>().copied()
    }

    #[allow(clippy::needless_range_loop)]
    fn columns(&self) -> impl Iterator<Item = [usize; 5]> + '_ {
        (0..5).map(|column_index| {
            let mut column = [0; 5];
            for row_index in 0..5 {
                column[row_index] = self.0[5 * row_index + column_index];
            }
            column
        })
    }

    fn score(&self, marked_numbers: &[usize]) -> Result<usize, BingoError> {
        Ok(self
            .0
            .iter()
            .filter(|n| !marked_numbers.contains(n))
            .sum::<usize>()
            * marked_numbers.last().ok_or(BingoError::NoMarkedNumbers)?)
    }
}

impl FromStr for BingoBoard {
    type Err = BingoError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<usize> = input
            .lines()
            .flat_map(|line| line.split(' ').collect::<Vec<&str>>())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().map_err(|_| BingoError::Parse))
            .collect::<Result<_, _>>()?;

        let mut array: [usize; 25] = Default::default();
        array[..25].clone_from_slice(&numbers[..25]);

        Ok(Self(array))
    }
}

#[derive(Debug)]
enum BingoError {
    Parse,
    NoWinningBoard,
    NoMarkedNumbers,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test() {
        assert_eq!(part_1(SAMPLE_INPUT).unwrap(), 4_512);
        assert_eq!(part_1(INPUT).unwrap(), 2_496);

        assert_eq!(part_2(SAMPLE_INPUT).unwrap(), 1_924);
        assert_eq!(part_2(INPUT).unwrap(), 25_925);
    }
}
