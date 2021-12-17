use anyhow::Result;
use nom::{
    bytes::complete::{tag, take},
    combinator::{map, map_res},
    multi::{length_count, many_till},
    sequence::preceded,
    IResult,
};

pub fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let soln_a = solve_a()?;
    eprintln!("Part A elapsed {:?}", start.elapsed());
    println!("solution part A: {}", soln_a);

    let start = std::time::Instant::now();
    let soln_b = solve_b()?;
    eprintln!("Part B elapsed {:?}", start.elapsed());
    println!("solution part B: {}", soln_b);

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Packet {
    LiteralPacket {
        version: u64,
        type_id: u64,
        num: u64,
    },
    OperatorPacket {
        version: u64,
        type_id: u64,
        subpackets: Vec<Packet>,
    },
}

fn hex2binary(s: &str) -> Result<String> {
    let bytes: Vec<u8> = hex::decode(s)?;
    let mut bin_string: String = String::new();
    for byte in bytes {
        bin_string.push_str(&format!("{:08b}", byte));
    }

    Ok(bin_string)
}

fn from_binary(s: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(s, 2)
}

fn parse_literal_packet(s: &str) -> IResult<&str, u64> {
    let (s, parts) = map(
        many_till(
            preceded(tag("1"), take(4usize)),
            preceded(tag("0"), take(4usize)),
        ),
        |(mut parts, final_part)| {
            parts.push(final_part);
            parts
        },
    )(s)?;

    let literal = parts.iter().enumerate().fold(0u64, |acc, (i, p)| {
        let mut x = u64::from_str_radix(p, 2).unwrap();
        x <<= 4 * (parts.len() - i - 1);
        acc + x
    });

    Ok((s, literal))
}

fn u16_bin11(s: &str) -> IResult<&str, u16> {
    map_res(take(11usize), |s| u16::from_str_radix(s, 2))(s)
}

fn parse_operator_packet(s: &str) -> IResult<&str, Vec<Packet>> {
    let (s, length_id) = map_res(take(1usize), from_binary)(s)?;

    let sub_packets_from_len = |s| {
        let (ns, subpacket_len) = map_res(take(15usize), from_binary)(s)?;
        let mut packets = vec![];
        let mut consumed_len = 0;
        let mut remaining_s = ns;

        while consumed_len < subpacket_len {
            let (ns, p) = parse_packet(remaining_s)?;
            packets.push(p);
            consumed_len += (remaining_s.len() - ns.len()) as u64;
            remaining_s = ns;
        }

        Ok((remaining_s, packets))
    };

    match length_id {
        0 => sub_packets_from_len(s),
        1 => length_count(u16_bin11, parse_packet)(s),
        _ => unreachable!(),
    }
}

fn parse_packet(s: &str) -> IResult<&str, Packet> {
    let (s, version) = map_res(take(3usize), from_binary)(s)?;
    let (s, type_id) = map_res(take(3usize), from_binary)(s)?;

    let (s, p) = match type_id {
        4 => {
            let (s, lit) = parse_literal_packet(s)?;
            (
                s,
                Packet::LiteralPacket {
                    version,
                    type_id,
                    num: lit,
                },
            )
        }
        _ => {
            let (s, packets) = parse_operator_packet(s)?;
            (
                s,
                Packet::OperatorPacket {
                    version,
                    type_id,
                    subpackets: packets,
                },
            )
        }
    };

    Ok((s, p))
}

fn sum_versions(p: &Packet) -> u64 {
    match p {
        Packet::LiteralPacket { version, .. } => *version,
        Packet::OperatorPacket {
            version,
            subpackets,
            ..
        } => version + subpackets.iter().map(sum_versions).sum::<u64>(),
    }
}

fn evaluate_packet(p: &Packet) -> u64 {
    match p {
        Packet::LiteralPacket { num, .. } => *num,
        Packet::OperatorPacket {
            type_id,
            subpackets,
            ..
        } => {
            let x = subpackets.iter().map(evaluate_packet).collect::<Vec<_>>();

            match type_id {
                0 => x.iter().sum(),
                1 => x.iter().product(),
                2 => *x.iter().min().unwrap(),
                3 => *x.iter().max().unwrap(),
                5 => {
                    if x[0] > x[1] {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if x[0] < x[1] {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if x[0] == x[1] {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

pub fn solve_a() -> Result<u64> {
    let x = hex2binary(include_str!("../input"))?;
    let (_, p) = parse_packet(&x).map_err(|e| e.map(|e| (e.input.to_string(), e.code)))?;
    let sver = sum_versions(&p);

    Ok(sver)
}

pub fn solve_b() -> Result<u64> {
    let x = hex2binary(include_str!("../input"))?;
    let (_, p) = parse_packet(&x).map_err(|e| e.map(|e| (e.input.to_string(), e.code)))?;
    let soln = evaluate_packet(&p);

    Ok(soln)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_hex2bin() {
        assert_eq!(
            hex2binary("D2FE28").unwrap(),
            "110100101111111000101000".to_string()
        )
    }

    #[test]
    fn test_literal() {
        let x = hex2binary("D2FE28").unwrap();
        let (_, p) = parse_packet(&x).unwrap();
        assert_eq!(
            p,
            Packet::LiteralPacket {
                version: 6,
                type_id: 4,
                num: 2021,
            }
        )
    }

    #[test]
    fn test_operator() {
        let x = hex2binary("38006F45291200").unwrap();
        let (_, p) = parse_packet(&x).unwrap();
        assert_eq!(
            p,
            Packet::OperatorPacket {
                version: 1,
                type_id: 6,
                subpackets: vec![
                    Packet::LiteralPacket {
                        version: 6,
                        type_id: 4,
                        num: 10,
                    },
                    Packet::LiteralPacket {
                        version: 2,
                        type_id: 4,
                        num: 20
                    }
                ]
            }
        )
    }

    #[test]
    fn test_sum_versions() {
        let x = hex2binary("8A004A801A8002F478").unwrap();
        let (_, p) = parse_packet(&x).unwrap();
        assert_eq!(sum_versions(&p), 16);

        let x = hex2binary("620080001611562C8802118E34").unwrap();
        let (_, p) = parse_packet(&x).unwrap();
        assert_eq!(sum_versions(&p), 12);

        let x = hex2binary("C0015000016115A2E0802F182340").unwrap();
        let (_, p) = parse_packet(&x).unwrap();
        assert_eq!(sum_versions(&p), 23);

        let x = hex2binary("A0016C880162017C3686B18A3D4780").unwrap();
        let (_, p) = parse_packet(&x).unwrap();
        assert_eq!(sum_versions(&p), 31);
    }

    #[test]
    fn test_evaluate() {
        let x = hex2binary("C200B40A82").unwrap();
        let (_, p) = parse_packet(&x).unwrap();
        assert_eq!(evaluate_packet(&p), 3);

        let x = hex2binary("04005AC33890").unwrap();
        let (_, p) = parse_packet(&x).unwrap();
        assert_eq!(evaluate_packet(&p), 54);

        let x = hex2binary("880086C3E88112").unwrap();
        let (_, p) = parse_packet(&x).unwrap();
        assert_eq!(evaluate_packet(&p), 7);

        let x = hex2binary("CE00C43D881120").unwrap();
        let (_, p) = parse_packet(&x).unwrap();
        assert_eq!(evaluate_packet(&p), 9);

        let x = hex2binary("D8005AC2A8F0").unwrap();
        let (_, p) = parse_packet(&x).unwrap();
        assert_eq!(evaluate_packet(&p), 1);

        let x = hex2binary("F600BC2D8F").unwrap();
        let (_, p) = parse_packet(&x).unwrap();
        assert_eq!(evaluate_packet(&p), 0);

        let x = hex2binary("9C005AC2F8F0").unwrap();
        let (_, p) = parse_packet(&x).unwrap();
        assert_eq!(evaluate_packet(&p), 0);

        let x = hex2binary("9C0141080250320F1802104A08").unwrap();
        let (_, p) = parse_packet(&x).unwrap();
        assert_eq!(evaluate_packet(&p), 1);
    }
}
