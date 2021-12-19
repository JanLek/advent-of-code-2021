#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::{
    cmp::{max, min},
    num::ParseIntError,
    str::FromStr,
};

fn part_1(input: &str) -> Result<i32, ParseError> {
    let target_area = TargetArea::from_str(input)?;
    let max_height = find_max_height(&target_area);
    Ok(max_height)
}

fn part_2(input: &str) -> Result<i32, ParseError> {
    let target_area = TargetArea::from_str(input)?;
    let num_trajectories = find_num_trajectories(&target_area);
    Ok(num_trajectories)
}

struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl FromStr for TargetArea {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let interesting_part = input.strip_prefix("target area: ").ok_or(ParseError)?;
        let (x_range, y_range) = interesting_part.split_once(", ").ok_or(ParseError)?;
        let (x_min, x_max) = x_range
            .strip_prefix("x=")
            .and_then(|s| s.split_once(".."))
            .ok_or(ParseError)?;
        let (y_a, y_b) = y_range
            .strip_prefix("y=")
            .and_then(|s| s.split_once(".."))
            .ok_or(ParseError)?;

        let y_a: i32 = y_a.parse()?;
        let y_b: i32 = y_b.parse()?;

        Ok(Self {
            x_min: x_min.parse()?,
            x_max: x_max.parse()?,
            y_min: min(y_a, y_b),
            y_max: max(y_a, y_b),
        })
    }
}

trait CheckTargetArea {
    fn is_in(&self, target_area: &TargetArea) -> bool;
}

impl CheckTargetArea for (i32, i32) {
    fn is_in(
        &self,
        TargetArea {
            x_min,
            x_max,
            y_min,
            y_max,
        }: &TargetArea,
    ) -> bool {
        let (x, y) = self;
        (x_min..=x_max).contains(&x) && (y_min..=y_max).contains(&y)
    }
}

fn find_max_height(target_area: &TargetArea) -> i32 {
    let mut max_height = 0;

    for delta_x in 0..100 {
        for delta_y in 0..100 {
            if let Some(height) = check_trajectory(delta_x, delta_y, target_area) {
                if height > max_height {
                    max_height = height;
                }
            }
        }
    }

    max_height
}

fn find_num_trajectories(target_area: &TargetArea) -> i32 {
    let mut num_trajectories = 0;

    for delta_x in delta_x_range(target_area) {
        for delta_y in -600..600 {
            if check_trajectory(delta_x, delta_y, target_area).is_some() {
                num_trajectories += 1;
            }
        }
    }

    num_trajectories
}

fn delta_x_range(target_area: &TargetArea) -> std::ops::RangeInclusive<i32> {
    // let mut from = 1;
    // let mut delta_from = 1;
    // while from < target_area.x_min {
    //     from += delta_from;
    //     delta_from += 1;
    // }
    let from = 1;
    let to = target_area.x_max;
    from..=to
}

fn check_trajectory(delta_x: i32, delta_y: i32, target_area: &TargetArea) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut max_y = 0;
    let mut delta_x = delta_x;
    let mut delta_y = delta_y;

    while can_reach_target(x, y, delta_x, delta_y, target_area) {
        if (x, y).is_in(target_area) {
            return Some(max_y);
        }

        x += delta_x;
        y += delta_y;
        if y > max_y {
            max_y = y;
        }

        if delta_x > 0 {
            delta_x -= 1;
        }
        delta_y -= 1;
    }

    None
}

fn can_reach_target(x: i32, y: i32, delta_x: i32, delta_y: i32, target_area: &TargetArea) -> bool {
    // y > target_area.y_min && (delta_x > 0 || x > target_area.x_min)

    if delta_x <= 0 && x < target_area.x_min {
        false
    } else if y < target_area.y_min {
        false
    } else {
        true
    }
}

#[derive(Debug)]
struct ParseError;

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("target area: x=20..30, y=-10..-5").unwrap(), 45);
        assert_eq!(part_1(INPUT).unwrap(), 3_003);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("target area: x=20..30, y=-10..-5").unwrap(), 112);
        assert_eq!(part_2(INPUT).unwrap(), 940);
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
