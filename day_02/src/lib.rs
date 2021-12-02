#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(maybe_uninit_uninit_array, maybe_uninit_array_assume_init, test)]

extern crate test;

use std::{mem::MaybeUninit, ops::Add};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Command {
    AdjustAim(i32),
    MoveForward(i32),
}

impl From<&str> for Command {
    fn from(input: &str) -> Self {
        let mut parts = input.split(' ');
        let direction = parts.next().unwrap();
        let x = parts.next().unwrap().parse().unwrap();
        assert_eq!(parts.next(), None);

        match direction {
            "forward" => Self::MoveForward(x),
            "down" => Self::AdjustAim(x),
            "up" => Self::AdjustAim(-x),
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Copy, Default)]
struct SubmarineState {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

impl SubmarineState {
    fn add_incorrectly(self, rhs: Command) -> Self {
        match rhs {
            Command::AdjustAim(x) => Self {
                depth: self.depth + x,
                ..self
            },
            Command::MoveForward(x) => Self {
                horizontal: self.horizontal + x,
                ..self
            },
        }
    }

    fn result(&self) -> i32 {
        self.depth * self.horizontal
    }
}

impl Add<Command> for SubmarineState {
    type Output = Self;

    fn add(self, rhs: Command) -> Self::Output {
        match rhs {
            Command::AdjustAim(x) => Self {
                aim: self.aim + x,
                ..self
            },
            Command::MoveForward(x) => Self {
                depth: self.depth + self.aim * x,
                horizontal: self.horizontal + x,
                ..self
            },
        }
    }
}

fn aggregate_commands<const N: usize>(
    commands: &[Command; N],
    apply: fn(SubmarineState, Command) -> SubmarineState,
) -> SubmarineState {
    commands
        .iter()
        .copied()
        .fold(SubmarineState::default(), apply)
}

fn parse_input_unsafe<'a, T, const N: usize>(
    input: &'a str,
    parse: impl Fn(&'a str) -> T + 'a,
) -> [T; N] {
    let mut depth_measurements: [MaybeUninit<T>; N] = MaybeUninit::uninit_array();
    let mut initialised_elements = 0;
    for (index, line) in input.lines().enumerate() {
        depth_measurements[index].write(parse(line));
        initialised_elements += 1;
    }
    assert_eq!(initialised_elements, N);
    unsafe { MaybeUninit::array_assume_init(depth_measurements) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input_unsafe(
                "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2",
                Command::from
            ),
            [
                Command::MoveForward(5),
                Command::AdjustAim(5),
                Command::MoveForward(8),
                Command::AdjustAim(-3),
                Command::AdjustAim(8),
                Command::MoveForward(2)
            ]
        );
    }

    #[test]
    fn sample_1() {
        // Arrange
        let commands: [_; 6] = parse_input_unsafe(
            "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2",
            Command::from,
        );

        // Act
        let state = aggregate_commands(&commands, SubmarineState::add_incorrectly);

        // Assert
        assert_eq!(state.depth, 10);
        assert_eq!(state.horizontal, 15);
        assert_eq!(state.result(), 150);
    }

    #[test]
    fn part_1() {
        // Arrange
        let commands: [_; 1000] = parse_input_unsafe(include_str!("commands.txt"), Command::from);

        // Act
        let state = aggregate_commands(&commands, SubmarineState::add_incorrectly);

        // Assert
        assert_eq!(state.result(), 2_147_104);
    }

    #[test]
    fn sample_2() {
        // Arrange
        let commands: [_; 6] = parse_input_unsafe(
            "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2",
            Command::from,
        );

        // Act
        let state = aggregate_commands(&commands, std::ops::Add::add);

        // Assert
        assert_eq!(state.depth, 60);
        assert_eq!(state.horizontal, 15);
        assert_eq!(state.result(), 900);
    }

    #[test]
    fn part_2() {
        // Arrange
        let commands: [_; 1000] = parse_input_unsafe(include_str!("commands.txt"), Command::from);

        // Act
        let state = aggregate_commands(&commands, Add::add);

        // Assert
        assert_eq!(state.result(), 2_044_620_088);
    }
}
