#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

use std::{
    cmp::{max, min},
    num::ParseIntError,
    str::FromStr,
};

fn part_1(input: &str) -> Result<i16, ParseError> {
    let target_area = TargetArea::from_str(input)?;
    let max_height = find_max_height(&target_area);
    Ok(max_height)
    // if let Some(max_height) = check_trajectory(6, 9, &target_area) {
    //     Ok(max_height)
    // } else {
    //     panic!()
    // }
}

fn part_2(_input: &str) -> Result<usize, ParseError> {
    todo!();
}

struct TargetArea {
    x_min: i16,
    x_max: i16,
    y_min: i16,
    y_max: i16,
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

        let y_a: i16 = y_a.parse()?;
        let y_b: i16 = y_b.parse()?;

        Ok(Self {
            x_min: x_min.parse()?,
            x_max: x_max.parse()?,
            y_min: min(y_a, y_b),
            y_max: max(y_a, y_b),
        })
    }
}

trait CheckTargetArea {
    fn has_passed(&self, target_area: &TargetArea) -> bool;
    fn is_in(&self, target_area: &TargetArea) -> bool;
}

impl CheckTargetArea for (i16, i16) {
    fn has_passed(&self, TargetArea { x_max, y_min, .. }: &TargetArea) -> bool {
        let (x, y) = self;
        x > x_max && y < y_min
    }
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

fn find_max_height(target_area: &TargetArea) -> i16 {
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

fn check_trajectory(delta_x: i16, delta_y: i16, target_area: &TargetArea) -> Option<i16> {
    let mut x = 0;
    let mut y = 0;
    let mut max_y = 0;
    let mut delta_x = delta_x;
    let mut delta_y = delta_y;

    while !((x, y).has_passed(target_area)) && can_reach_target(x, y, delta_x, delta_y, target_area)
    {
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

fn can_reach_target(x: i16, y: i16, delta_x: i16, delta_y: i16, target_area: &TargetArea) -> bool {
    if delta_x <= 0 && x < target_area.x_min {
        false
    } else if delta_y < 0 && y < target_area.y_min {
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
    #[ignore]
    fn test_part_2() {
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
