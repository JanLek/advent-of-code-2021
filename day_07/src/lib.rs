#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(int_abs_diff, test)]

use std::convert::identity;

fn part_1(input: &str) -> usize {
    find_minimal_fuel_cost(&parse_input(input), identity)
}

fn part_2(input: &str) -> usize {
    find_minimal_fuel_cost(
        &parse_input(input),
        |n| n * (n + 1) / 2, // https://en.wikipedia.org/wiki/Triangular_number
    )
}

fn find_minimal_fuel_cost(
    crab_submarine_positions: &[u8; 2000],
    calculate_fuel_cost_for_distance: fn(usize) -> usize,
) -> usize {
    (0..=crab_submarine_positions.len())
        .map(|target_position| {
            calculate_total_fuel_cost(
                crab_submarine_positions,
                target_position,
                calculate_fuel_cost_for_distance,
            )
        })
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> [u8; 2000] {
    let mut positions = [0; 2000];
    for position in input.split(',') {
        positions[position.parse::<usize>().unwrap()] += 1;
    }
    positions
}

fn calculate_total_fuel_cost(
    positions: &[u8; 2000],
    target_position: usize,
    fuel_cost_for_distance: fn(usize) -> usize,
) -> usize {
    positions
        .iter()
        .enumerate()
        .fold(0, |cost, (position, &num_crab_submarines)| {
            cost + (num_crab_submarines as usize)
                * fuel_cost_for_distance(position.abs_diff(target_position))
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
    fn test() {
        assert_eq!(part_1(SAMPLE_INPUT), 37);
        assert_eq!(part_1(INPUT), 349_357);

        assert_eq!(part_2(SAMPLE_INPUT), 168);
        assert_eq!(part_2(INPUT), 96_708_205);
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
