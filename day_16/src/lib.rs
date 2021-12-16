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

fn part_2(input: &str) -> Result<usize, ParseError> {
    let bits = convert_hex_to_binary(input)?;
    let (packet, remaining) = parse_packet(&bits)?;
    if remaining.bytes().all(|c| c == b'0') {
        Ok(packet.value())
    } else {
        Err(ParseError)
    }
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
    let (operation_type, remaining) = parse_operation_type(remaining)?;
    if let OperationType::SingleNumber = operation_type {
        let (value, remaining) = parse_value(remaining)?;
        Ok((Packet::Literal { version, value }, remaining))
    } else {
        let (length_type_id, remaining) = parse_n_bit_number(1, remaining)?;
        match length_type_id {
            0 => {
                let (length, remaining) = parse_n_bit_number(15, remaining)?;
                let (packets, remaining) = parse_length_of_packets(length, remaining)?;
                Ok((
                    Packet::Operation {
                        version,
                        operation_type,
                        packets,
                    },
                    remaining,
                ))
            }
            1 => {
                let (n, remaining) = parse_n_bit_number(11, remaining)?;
                let (packets, remaining) = parse_n_packets(n, remaining)?;
                Ok((
                    Packet::Operation {
                        version,
                        operation_type,
                        packets,
                    },
                    remaining,
                ))
            }
            _ => Err(ParseError),
        }
    }
}

fn parse_operation_type(input: &str) -> Result<(OperationType, &str), ParseError> {
    let (type_id, remaining) = parse_n_bit_number(3, input)?;
    let operation_type = OperationType::try_from(type_id)?;
    Ok((operation_type, remaining))
}

fn parse_value(input: &str) -> Result<(usize, &str), ParseError> {
    let mut number = String::new();
    let mut is_last = false;
    let mut remaining = input;
    while !is_last {
        let (first_bit, r) = parse_n_bit_number(1, remaining)?;
        is_last = first_bit == 0;
        let (group, r) = parse_n_bits(4, r)?;
        number.push_str(group);
        remaining = r;
    }
    Ok((
        usize::from_str_radix(&number, 2).map_err(|_| ParseError)?,
        remaining,
    ))
}

fn parse_length_of_packets(length: usize, input: &str) -> Result<(Vec<Packet>, &str), ParseError> {
    let mut length_of_parsed_sub_packets = 0;
    let mut sub_packets: Vec<Packet> = Vec::new();
    let mut remaining = input;
    while length_of_parsed_sub_packets < length && !remaining.is_empty() {
        let (sub_packet, r) = parse_packet(remaining)?;
        sub_packets.push(sub_packet);
        length_of_parsed_sub_packets += remaining.len() - r.len();
        remaining = r;
    }
    Ok((sub_packets, remaining))
}

fn parse_n_packets(n: usize, input: &str) -> Result<(Vec<Packet>, &str), ParseError> {
    let mut sub_packets = Vec::with_capacity(n);
    let mut r = input;
    for _ in 0..n {
        let (sub_packet, remaining) = parse_packet(r)?;
        sub_packets.push(sub_packet);
        r = remaining;
    }
    Ok((sub_packets, r))
}

enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Operation {
        version: usize,
        operation_type: OperationType,
        packets: Vec<Packet>,
    },
}

impl Packet {
    fn version(&self) -> usize {
        match *self {
            Self::Literal { version, .. } | Self::Operation { version, .. } => version,
        }
    }

    fn sum_versions(&self) -> usize {
        match self {
            Self::Literal { version, .. } => *version,
            Self::Operation {
                version, packets, ..
            } => version + packets.iter().map(Self::sum_versions).sum::<usize>(),
        }
    }

    fn value(&self) -> usize {
        match self {
            Self::Literal { value, .. } => *value,
            Self::Operation {
                packets,
                operation_type,
                ..
            } => match operation_type {
                OperationType::Sum => packets.iter().map(Packet::value).sum(),
                OperationType::Product => packets.iter().map(Packet::value).product(),
                OperationType::Minimum => packets.iter().map(Packet::value).min().unwrap(),
                OperationType::Maximum => packets.iter().map(Packet::value).max().unwrap(),
                OperationType::SingleNumber => panic!(),
                OperationType::GreaterThan => {
                    if packets[0].value() > packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                OperationType::LessThan => {
                    if packets[0].value() < packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                OperationType::EqualTo => {
                    if packets[0].value() == packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

enum OperationType {
    Sum,
    Product,
    Minimum,
    Maximum,
    SingleNumber,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl TryFrom<usize> for OperationType {
    type Error = ParseError;

    fn try_from(operation_type: usize) -> Result<Self, Self::Error> {
        match operation_type {
            0 => Ok(Self::Sum),
            1 => Ok(Self::Product),
            2 => Ok(Self::Minimum),
            3 => Ok(Self::Maximum),
            4 => Ok(Self::SingleNumber),
            5 => Ok(Self::GreaterThan),
            6 => Ok(Self::LessThan),
            7 => Ok(Self::EqualTo),
            _ => Err(ParseError),
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
    fn test_part_2() {
        assert_eq!(part_2("C200B40A82").unwrap(), 3);
        assert_eq!(part_2("04005AC33890").unwrap(), 54);
        assert_eq!(part_2("880086C3E88112").unwrap(), 7);
        assert_eq!(part_2("CE00C43D881120").unwrap(), 9);
        assert_eq!(part_2("D8005AC2A8F0").unwrap(), 1);
        assert_eq!(part_2("F600BC2D8F").unwrap(), 0);
        assert_eq!(part_2("9C005AC2F8F0").unwrap(), 0);
        assert_eq!(part_2("9C0141080250320F1802104A08").unwrap(), 1);

        assert_eq!(part_2(INPUT).unwrap(), 739_303_923_668);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| part_1(INPUT).unwrap());
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| part_2(INPUT).unwrap());
    }
}
