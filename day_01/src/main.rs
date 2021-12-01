#![deny(clippy::all, clippy::pedantic)]

fn main() {
    let depth_measurements = parse_input::<2000>(include_str!("depth-measurements.txt"));

    println!(
        "Part 1 - Number of increases in sea depth: {}",
        count_sliding_sum_increases(&depth_measurements, 1)
    );

    println!(
        "Part 2 - Number of sliding window increases in sea depth: {}",
        count_sliding_sum_increases(&depth_measurements, 3)
    );
}

// Parse to array rather than Vec to avoid heap allocation
fn parse_input<const N: usize>(input: &str) -> [u32; N] {
    let mut depth_measurements = [0; N];
    for (index, line) in input.lines().enumerate() {
        assert!(index < N);
        depth_measurements[index] = line.parse().unwrap();
    }
    assert_ne!(depth_measurements[N - 1], 0);
    depth_measurements
}

fn count_sliding_sum_increases<const N: usize>(numbers: &[u32; N], size: usize) -> usize {
    // Little trick: when comparing two sliding sums, the last numbers of the
    // first sum are the same as the first numbers of the second sum, so we
    // only need to compare the first number of the first sum with the last
    // number of the second sum.
    numbers
        .windows(size + 1)
        .filter(|window| window[size] > window[0])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        let depth_measurements = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        // Act
        let num_increases = count_sliding_sum_increases(&depth_measurements, 1);

        // Assert
        assert_eq!(num_increases, 7);
    }

    #[test]
    fn test_part_2() {
        // Arrange
        let depth_measurements = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        // Act
        let num_increases = count_sliding_sum_increases(&depth_measurements, 3);

        // Assert
        assert_eq!(num_increases, 5);
    }
}
