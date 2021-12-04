#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]

use std::str::FromStr;

fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let numbers: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let boards: Vec<BingoBoard> = input
        .split("\n\n")
        .skip(1)
        .map(|board| board.parse().unwrap())
        .collect();

    for i in 1..numbers.len() {
        let marked_numbers = &numbers[0..=i];
        if let Some(winning_board) = boards.iter().find(|board| board.wins(marked_numbers)) {
            return winning_board.score(marked_numbers);
        }
    }

    panic!("No winning board")
}

fn part_2(input: &str) -> usize {
    let mut lines = input.lines();
    let numbers: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards: Vec<BingoBoard> = input
        .split("\n\n")
        .skip(1)
        .map(|board| board.parse().unwrap())
        .collect();

    for i in 1..numbers.len() {
        let marked_numbers = &numbers[0..=i];
        if boards.len() > 1 {
            boards.retain(|board| !board.wins(marked_numbers));
        } else if boards[0].wins(marked_numbers) {
            return boards[0].score(marked_numbers);
        }
    }

    panic!("No winning board")
}

#[derive(Debug)]
struct BingoBoard([usize; 25]);

impl BingoBoard {
    fn wins(&self, marked_numbers: &[usize]) -> bool {
        // Horizontal row win
        if self
            .0
            .chunks(5)
            .any(|row| row.iter().all(|n| marked_numbers.contains(n)))
        {
            return true;
        }

        // TODO vertical row win
        for column_index in 0..5 {
            if (0..5)
                .map(|row_index| self.0[5 * row_index + column_index])
                .all(|n| marked_numbers.contains(&n))
            {
                return true;
            }
        }

        false
    }

    fn score(&self, marked_numbers: &[usize]) -> usize {
        self.0
            .iter()
            .filter(|n| !marked_numbers.contains(n))
            .sum::<usize>()
            * marked_numbers.last().unwrap()
    }
}

impl FromStr for BingoBoard {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<usize> = input
            .lines()
            .flat_map(|line| line.split(' ').collect::<Vec<&str>>())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let mut array: [usize; 25] = Default::default();
        array[..25].clone_from_slice(&numbers[..25]);

        Ok(Self(array))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test() {
        assert_eq!(part_1(SAMPLE_INPUT), 4_512);
        assert_eq!(part_1(INPUT), 2_496);

        assert_eq!(part_2(SAMPLE_INPUT), 1_924);
        assert_eq!(part_2(INPUT), 25_925);
    }
}
