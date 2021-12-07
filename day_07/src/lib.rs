#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(int_abs_diff, test)]

fn part_1(input: &str) -> usize {
    let counts = parse_input(input);
    (0..=2000)
        .map(|target_position| calculate_fuel_cost_incorrectly(&counts, target_position))
        .min()
        .unwrap()
}

fn part_2(input: &str) -> usize {
    let counts = parse_input(input);
    (0..=2000)
        .map(|target_position| calculate_fuel_cost(&counts, target_position))
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> [u8; 2000] {
    let mut positions = [0; 2000];

    for position in input.split(',') {
        let position: usize = position.parse().unwrap();
        positions[position] += 1;
    }

    positions
}

fn calculate_fuel_cost_incorrectly(positions: &[u8; 2000], target_position: usize) -> usize {
    positions
        .iter()
        .enumerate()
        .fold(0, |cost, (position, &num_crab_submarines)| {
            cost + (num_crab_submarines as usize) * position.abs_diff(target_position)
        })
}

fn calculate_fuel_cost(positions: &[u8; 2000], target_position: usize) -> usize {
    positions
        .iter()
        .enumerate()
        .fold(0, |cost, (position, &num_crab_submarines)| {
            let distance = position.abs_diff(target_position);
            if num_crab_submarines == 0 || distance == 0 {
                cost
            } else {
                cost + (num_crab_submarines as usize)
                    * fuel_cost_for_distance(position.abs_diff(target_position))
            }
        })
}

fn fuel_cost_for_distance(n: usize) -> usize {
    // https://en.wikipedia.org/wiki/1_%2B_2_%2B_3_%2B_4_%2B_%E2%8B%AF
    (n * (n + 1)) / 2
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
