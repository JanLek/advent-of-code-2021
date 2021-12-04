#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::str::FromStr;

fn part_1(input: &str) -> Result<usize, BingoError> {
    let (numbers, mut boards) = parse_input(input)?;

    for number in numbers {
        for board in &mut boards {
            board.mark(number);
            if board.winner {
                return Ok(board.score(number));
            }
        }
    }

    Err(BingoError::NoWinningBoard)
}

fn part_2(input: &str) -> Result<usize, BingoError> {
    let (numbers, mut boards) = parse_input(input)?;

    for number in numbers {
        if boards.len() > 1 {
            for board in &mut boards {
                board.mark(number);
            }
            boards = boards.into_iter().filter(|b| !b.winner).collect();
        } else {
            boards[0].mark(number);
            if boards[0].winner {
                return Ok(boards[0].score(number));
            }
        }
    }

    Err(BingoError::NoWinningBoard)
}

fn parse_input(input: &str) -> Result<(Vec<usize>, Vec<BingoBoard>), BingoError> {
    let mut parts = input.split("\n\n");
    let numbers: Vec<usize> = parts
        .next()
        .ok_or(BingoError::InvalidInput)?
        .split(',')
        .map(|n| n.parse().map_err(|_| BingoError::InvalidInput))
        .collect::<Result<_, _>>()?;
    let boards: Vec<BingoBoard> = parts.map(str::parse).collect::<Result<_, _>>()?;
    Ok((numbers, boards))
}

struct BingoBoard {
    numbers: [BingoNumber; 25],
    winner: bool,
}

impl BingoBoard {
    fn mark(&mut self, number: usize) {
        if let Some(index) = self.numbers.iter().position(|&n| n == number) {
            self.numbers[index] = BingoNumber::Marked;
            self.check_win(index);
        }
    }

    fn check_win(&mut self, index: usize) {
        let row = index / 5;
        let column = index % 5;

        let mut row_numbers = self.numbers[row * 5..row * 5 + 5].iter();
        let mut column_numbers = (0..5).map(|row| &self.numbers[5 * row + column]);

        self.winner =
            row_numbers.all(BingoNumber::is_marked) || column_numbers.all(BingoNumber::is_marked);
    }

    fn score(&self, last_number: usize) -> usize {
        let sum: usize = self.numbers.iter().filter_map(<Option<usize>>::from).sum();
        sum * last_number
    }
}

impl FromStr for BingoBoard {
    type Err = BingoError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut numbers_iter = input
            .split(|c| c == '\n' || c == ' ')
            .filter(|s| !s.is_empty())
            .map(BingoNumber::from_str);
        let mut numbers = [BingoNumber::Unmarked(0); 25];
        for number in &mut numbers {
            *number = numbers_iter
                .next()
                .unwrap_or(Err(BingoError::InvalidInput))?;
        }
        Ok(Self {
            numbers,
            winner: false,
        })
    }
}

#[derive(Clone, Copy)]
enum BingoNumber {
    Unmarked(usize),
    Marked,
}

impl BingoNumber {
    fn is_marked(&self) -> bool {
        match self {
            Self::Unmarked(_) => false,
            Self::Marked => true,
        }
    }
}

impl FromStr for BingoNumber {
    type Err = BingoError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input
            .parse::<usize>()
            .map(BingoNumber::Unmarked)
            .map_err(|_| BingoError::InvalidInput)
    }
}

impl From<&BingoNumber> for Option<usize> {
    fn from(&bingo_number: &BingoNumber) -> Self {
        match bingo_number {
            BingoNumber::Unmarked(n) => Some(n),
            BingoNumber::Marked => None,
        }
    }
}

impl PartialEq<usize> for BingoNumber {
    fn eq(&self, rhs: &usize) -> bool {
        match self {
            Self::Unmarked(n) => n == rhs,
            Self::Marked => false,
        }
    }
}

#[derive(Debug)]
enum BingoError {
    InvalidInput,
    NoWinningBoard,
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
        assert_eq!(part_1(SAMPLE_INPUT).unwrap(), 4_512);
        assert_eq!(part_1(INPUT).unwrap(), 2_496);

        assert_eq!(part_2(SAMPLE_INPUT).unwrap(), 1_924);
        assert_eq!(part_2(INPUT).unwrap(), 25_925);
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
