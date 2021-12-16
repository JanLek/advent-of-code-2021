#![allow(dead_code)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(int_roundings, test)]

fn part_1(input: &str) -> Result<usize, ParseError> {
    let bits = convert_hex_to_binary(input)?;
    let (packet, remaining) = parse_packet(&bits)?;
    if remaining.bytes().all(|c| c == b'0') {
        Ok(packet.sum_versions())
    } else {
        Err(ParseError)
    }
}

fn part_2(_input: &str) -> Result<usize, ParseError> {
    todo!()
}

fn convert_hex_to_binary(hex: &str) -> Result<String, ParseError> {
    hex.bytes()
        .map(|hex_byte| match hex_byte {
            b'0'..=b'9' => Ok(hex_byte - b'0'),
            b'A'..=b'F' => Ok(10 + hex_byte - b'A'),
            _ => Err(ParseError),
        })
        .map(|binary_byte| binary_byte.map(|byte| format!("{:04b}", byte)))
        .collect()
}

fn parse_n_bits(n: usize, input: &str) -> Result<(&str, &str), ParseError> {
    if input.len() >= n {
        Ok((&input[0..n], &input[n..]))
    } else {
        Err(ParseError)
    }
}

fn parse_n_bit_number(n: usize, input: &str) -> Result<(usize, &str), ParseError> {
    parse_n_bits(n, input).and_then(|(bits, remaining)| {
        let number = usize::from_str_radix(bits, 2).map_err(|_| ParseError)?;
        Ok((number, remaining))
    })
}

fn parse_packet(input: &str) -> Result<(Packet, &str), ParseError> {
    let (version, remaining) = parse_n_bit_number(3, input)?;
    let (type_id, remaining) = parse_n_bit_number(3, remaining)?;
    if type_id == 4 {
        let mut number = String::new();
        let mut is_last = false;
        let mut r = remaining;
        while !is_last {
            let (first_bit, remaining) = parse_n_bit_number(1, r)?;
            is_last = first_bit == 0;
            let (group, remaining) = parse_n_bits(4, remaining)?;
            number.push_str(group);
            r = remaining;
        }
        Ok((Packet::Literal { version }, r))
    } else {
        let (length_type_id, remaining) = parse_n_bit_number(1, remaining)?;
        match length_type_id {
            0 => {
                let (length_of_sub_packets, remaining) = parse_n_bit_number(15, remaining)?;
                let mut length_of_parsed_sub_packets = 0;
                let mut sub_packets: Vec<Packet> = Vec::new();
                let mut r = remaining;
                while length_of_parsed_sub_packets < length_of_sub_packets && !r.is_empty() {
                    let (sub_packet, remaining) = parse_packet(r)?;
                    sub_packets.push(sub_packet);
                    length_of_parsed_sub_packets += r.len() - remaining.len();
                    r = remaining;
                }
                Ok((
                    Packet::Operation {
                        version,
                        packets: sub_packets,
                    },
                    r,
                ))
            }
            1 => {
                let (number_of_sub_packets, remaining) = parse_n_bit_number(11, remaining)?;
                let mut sub_packets = Vec::with_capacity(number_of_sub_packets);
                let mut r = remaining;
                for _ in 0..number_of_sub_packets {
                    let (sub_packet, remaining) = parse_packet(r)?;
                    sub_packets.push(sub_packet);
                    r = remaining;
                }
                Ok((
                    Packet::Operation {
                        version,
                        packets: sub_packets,
                    },
                    r,
                ))
            }
            _ => Err(ParseError),
        }
    }
}

#[derive(Debug)]
enum Packet {
    Literal {
        version: usize,
    },
    Operation {
        version: usize,
        packets: Vec<Packet>,
    },
}

impl Packet {
    fn version(&self) -> usize {
        match *self {
            Self::Literal { version } | Self::Operation { version, .. } => version,
        }
    }

    fn sum_versions(&self) -> usize {
        match self {
            Self::Literal { version } => *version,
            Self::Operation { version, packets } => {
                version + packets.iter().map(Self::sum_versions).sum::<usize>()
            }
        }
    }
}

#[derive(Debug)]
struct ParseError;

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("D2FE28").unwrap(), 6);
        assert_eq!(part_1("38006F45291200").unwrap(), 9);
        assert_eq!(part_1("EE00D40C823060").unwrap(), 14);
        assert_eq!(part_1("8A004A801A8002F478").unwrap(), 16);
        assert_eq!(part_1("620080001611562C8802118E34").unwrap(), 12);
        assert_eq!(part_1("C0015000016115A2E0802F182340").unwrap(), 23);
        assert_eq!(part_1("A0016C880162017C3686B18A3D4780").unwrap(), 31);

        assert_eq!(part_1(INPUT).unwrap(), 889);
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
