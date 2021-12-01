#![deny(clippy::all, clippy::pedantic)]

const DEPTH_MEASUREMENTS: &str = include_str!("depth-measurements.txt");

fn main() {
    let depth_measurements: Vec<_> = {
        DEPTH_MEASUREMENTS
            .split('\n')
            .map(|number| number.parse().unwrap())
            .collect()
    };

    println!(
        "Part 1 - Number of increases in sea depth: {}",
        count_increases(depth_measurements.iter().copied())
    );

    println!(
        "Part 2 - Number of sliding window increases in sea depth: {}",
        count_sliding_sum_increases(&depth_measurements)
    );
}

fn count_increases(numbers: impl Iterator<Item = u32>) -> u32 {
    numbers
        .fold((u32::MAX, 0), |(previous, count), number| {
            (number, if number > previous { count + 1 } else { count })
        })
        .1
}

fn count_sliding_sum_increases(numbers: &[u32]) -> u32 {
    let window_sums = numbers.windows(3).map(|window| window.iter().sum());
    count_increases(window_sums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increases() {
        // Arrange
        let depth_measurements = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        // Act
        let num_increases = count_increases(depth_measurements.into_iter());

        // Assert
        assert_eq!(num_increases, 7);
    }

    #[test]
    fn test_count_sliding_sum_increases() {
        // Arrange
        let depth_measurements = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        // Act
        let num_increases = count_sliding_sum_increases(&depth_measurements);

        // Assert
        assert_eq!(num_increases, 5);
    }
}
