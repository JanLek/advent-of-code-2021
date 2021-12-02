#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(maybe_uninit_uninit_array, maybe_uninit_array_assume_init, test)]

extern crate test;

use std::mem::MaybeUninit;

const INPUT: &str = include_str!("commands.txt");

fn part_1(commands: &[Command; 1_000]) -> i32 {
    aggregate_commands(commands, SubmarineState::apply_command_incorrectly).result()
}

fn part_2(commands: &[Command; 1_000]) -> i32 {
    aggregate_commands(commands, SubmarineState::apply_command).result()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Command {
    AdjustAim(i32),
    MoveForward(i32),
}

#[derive(Clone, Copy, Default)]
struct SubmarineState {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

impl SubmarineState {
    fn apply_command_incorrectly(self, command: Command) -> Self {
        match command {
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

    fn apply_command(self, command: Command) -> Self {
        match command {
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

    fn result(&self) -> i32 {
        self.depth * self.horizontal
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

#[allow(clippy::needless_range_loop)]
fn fast_parse<const N: usize>(input: &str) -> [Command; N] {
    let bytes = input.as_bytes();
    let mut commands: [MaybeUninit<Command>; N] = MaybeUninit::uninit_array();

    let mut index = 0;
    for j in 0..commands.len() {
        match bytes[index] {
            b'f' => {
                index += 8;
                commands[j].write(Command::MoveForward(parse_byte_to_int(bytes[index])));
                index += 2;
            }
            b'd' => {
                index += 5;
                commands[j].write(Command::AdjustAim(parse_byte_to_int(bytes[index])));
                index += 2;
            }
            b'u' => {
                index += 3;
                commands[j].write(Command::AdjustAim(-(parse_byte_to_int(bytes[index]))));
                index += 2;
            }
            _ => panic!(),
        }
    }

    unsafe { MaybeUninit::array_assume_init(commands) }
}

fn parse_byte_to_int(byte: u8) -> i32 {
    i32::from(byte - b'0')
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn sample_1() {
        // Arrange
        let commands: [_; 6] = fast_parse("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");

        // Act
        let state = aggregate_commands(&commands, SubmarineState::apply_command_incorrectly);

        // Assert
        assert_eq!(state.depth, 10);
        assert_eq!(state.horizontal, 15);
        assert_eq!(state.result(), 150);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&fast_parse(INPUT)), 2_147_104);
    }

    #[test]
    fn sample_2() {
        // Arrange
        let commands: [_; 6] = fast_parse("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");

        // Act
        let state = aggregate_commands(&commands, SubmarineState::apply_command);

        // Assert
        assert_eq!(state.depth, 60);
        assert_eq!(state.horizontal, 15);
        assert_eq!(state.result(), 900);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&fast_parse(INPUT)), 2_044_620_088);
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let commands: [_; 1000] = fast_parse(INPUT);
            commands
        });
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let c = fast_parse(INPUT);
        b.iter(|| part_1(&c));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let c = fast_parse(INPUT);
        b.iter(|| part_2(&c));
    }
}
