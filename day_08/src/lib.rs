#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

const NUM_SEGMENTS: [u8; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 8];

fn part_1(input: &str) -> usize {
    parse_input(input)
        .flat_map(|(_, digit_output_values)| digit_output_values)
        .filter(|digit_output_value| [2, 4, 3, 7].contains(&digit_output_value.len()))
        .count()
}

fn part_2(input: &str) -> usize {
    parse_input(input).map(deduce_output_value).sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = ([String; 10], [String; 4])> + '_ {
    input.lines().map(|line| {
        let (signal_patterns, digit_output_values) = line.split_once(" | ").unwrap();
        (
            to_array_of_sorted_strings(signal_patterns),
            to_array_of_sorted_strings(digit_output_values),
        )
    })
}

fn to_array_of_sorted_strings<const N: usize>(input: &str) -> [String; N]
where
    [String; N]: Default,
{
    let mut iterator = input.split(' ').map(|s| {
        let mut bytes = s.as_bytes().to_vec();
        bytes.sort_unstable();
        unsafe { String::from_utf8_unchecked(bytes) }
    });

    let mut array: [String; N] = Default::default();
    for element in &mut array {
        *element = iterator.next().unwrap();
    }
    array
}

const EMPTY_STRING: String = String::new();

#[allow(clippy::too_many_lines)]
fn deduce_output_value(
    (signal_patterns, digit_output_values): ([String; 10], [String; 4]),
) -> usize {
    let find_by_length = |length| {
        signal_patterns
            .iter()
            .find(|pattern| pattern.len() == length)
            .unwrap()
    };
    let filter_by_length = |length| {
        let mut iterator = signal_patterns
            .iter()
            .filter(|&pattern| pattern.len() == length);

        let mut array = [&signal_patterns[0]; 3];
        for i in 0..3 {
            array[i] = iterator.next().unwrap();
        }
        array
    };

    let empty_string = String::new(); // TODO make constant
    let mut patterns: [&String; 10] = [&empty_string; 10];

    // Unique length digits
    patterns[1] = find_by_length(2);
    patterns[4] = find_by_length(4);
    patterns[7] = find_by_length(3);
    patterns[8] = find_by_length(7);

    let two_three_or_five = filter_by_length(5);
    patterns[3] = pick_by_match(&two_three_or_five, patterns[1]);
    let four_sans_one: String = patterns[4]
        .chars()
        .filter(|&c| !patterns[1].contains(c))
        .collect();
    patterns[5] = two_three_or_five
        .iter()
        .find(|&&pattern| {
            pattern != patterns[3] && four_sans_one.chars().all(|c| pattern.contains(c))
        })
        .unwrap();
    patterns[2] = pick_by_exclusion(&two_three_or_five, &[patterns[3], patterns[5]]);

    let zero_six_or_nine = filter_by_length(6);
    patterns[6] = zero_six_or_nine
        .iter()
        .find(|pattern| !patterns[1].chars().all(|c| pattern.contains(c)))
        .unwrap();
    patterns[9] = zero_six_or_nine
        .iter()
        .find(|&&pattern| {
            pattern != patterns[6] && four_sans_one.chars().all(|c| pattern.contains(c))
        })
        .unwrap();
    patterns[0] = pick_by_exclusion(&zero_six_or_nine, &[patterns[6], patterns[9]]);

    digit_output_values
        .iter()
        .rev()
        .enumerate()
        .fold(0, |result, (index, digit_output_value)| {
            result
                + patterns
                    .iter()
                    .position(|&pattern| pattern == digit_output_value)
                    .unwrap()
                    * 10_usize.pow(index as u32)
        })
}

fn pick_by_match<'a>(patterns: &[&'a String], match_pattern: &str) -> &'a String {
    patterns
        .iter()
        .find(|pattern| match_pattern.chars().all(|c| pattern.contains(c)))
        .unwrap()
}

fn pick_by_exclusion<'a>(patterns: &[&'a String], excluded: &[&'a String]) -> &'a String {
    patterns
        .iter()
        .find(|pattern| !excluded.contains(pattern))
        .unwrap()
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
        assert_eq!(part_1(SAMPLE_INPUT), 26);
        assert_eq!(part_1(INPUT), 504);

        assert_eq!(part_2("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), 5353);
        assert_eq!(part_2(SAMPLE_INPUT), 61_229);
        assert_eq!(part_2(INPUT), 1_073_431);
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
