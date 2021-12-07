#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(int_abs_diff, test)]

fn part_1(input: &str) -> usize {
    let counts = parse_input(input);
    (0..=2000)
        .map(|target_position| calculate_fuel_cost(&counts, target_position))
        .min()
        .unwrap()
}

fn part_2(_input: &str) -> usize {
    todo!()
}

fn parse_input(input: &str) -> [u8; 2000] {
    let mut positions = [0; 2000];

    for position in input.split(',') {
        let position: usize = position.parse().unwrap();
        positions[position] += 1;
    }

    positions
}

fn calculate_fuel_cost(positions: &[u8; 2000], target_position: usize) -> usize {
    positions
        .iter()
        .enumerate()
        .fold(0, |cost, (position, &num_crab_submarines)| {
            cost + (num_crab_submarines as usize) * position.abs_diff(target_position)
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
        assert_eq!(part_1(INPUT), 0);

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
