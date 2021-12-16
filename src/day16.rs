use crate::day16::Packet::LP;

const LITERAL_PACKET_TYPE_ID: u8 = 4;

#[derive(Debug)]
struct LiteralPacket {
    version: u8,
    type_id: u8,
    literal: i64,
}

#[derive(Debug)]
struct OperatorPacket {
    version: u8,
    type_id: u8,
    tag: u8,
    sub_packets: Vec<Packet>,
}

#[derive(Debug)]
enum Packet {
    LP(LiteralPacket),
    OP(OperatorPacket),
}

pub fn first_part(s: &str) -> i64 {
    solution(s, true)
}

pub fn second_part(s: &str) -> i64 {
    solution(s, false)
}

fn solution(s: &str, first_part: bool) -> i64 {
    let mut curr_idx = 0;
    let xs = convert_to_binary_from_hex(s);
    let padding = peek_if_literal(&xs);
    let packet = parse_packet(&xs, &mut curr_idx, padding);
    if first_part {
        sum_version(&packet) as i64
    } else {
        operator_value(&packet)
    }
}

fn operator_value(packet: &Packet) -> i64 {
    fn dfs(p: &Packet) -> i64 {
        match p {
            Packet::LP(lp) => lp.literal,
            Packet::OP(op) => {
                if op.type_id == 0 {
                    op.sub_packets.iter().fold(0, |acc, sub| acc + dfs(sub))
                } else if op.type_id == 1 {
                    op.sub_packets.iter().fold(1, |acc, sub| acc * dfs(sub))
                } else if op.type_id == 2 {
                    op.sub_packets
                        .iter()
                        .fold(i64::MAX, |acc, sub| acc.min(dfs(sub)))
                } else if op.type_id == 3 {
                    op.sub_packets
                        .iter()
                        .fold(i64::MIN, |acc, sub| acc.max(dfs(sub)))
                } else if op.type_id == 5 {
                    if op.sub_packets.len() != 2 {
                        panic!("len is not exactly 2  in type_id == 5");
                    }
                    let first = dfs(&op.sub_packets[0]);
                    let second = dfs(&op.sub_packets[1]);
                    if first > second {
                        1
                    } else {
                        0
                    }
                } else if op.type_id == 6 {
                    if op.sub_packets.len() != 2 {
                        panic!("len is not exactly 2 in type_id == 6");
                    }
                    let first = dfs(&op.sub_packets[0]);
                    let second = dfs(&op.sub_packets[1]);
                    if first < second {
                        1
                    } else {
                        0
                    }
                } else if op.type_id == 7 {
                    if op.sub_packets.len() != 2 {
                        panic!("len is not exactly 2 in type_id == 7");
                    }
                    let first = dfs(&op.sub_packets[0]);
                    let second = dfs(&op.sub_packets[1]);
                    if first == second {
                        1
                    } else {
                        0
                    }
                } else {
                    panic!("invalid Packet");
                }
            }
        }
    }
    dfs(packet)
}

fn sum_version(packet: &Packet) -> i32 {
    let mut sum = 0;
    fn dfs(p: &Packet, sum: &mut i32) {
        match p {
            Packet::LP(lp) => *sum += lp.version as i32,
            Packet::OP(op) => {
                *sum += op.version as i32;
                for sub in op.sub_packets.iter() {
                    dfs(sub, sum);
                }
            }
        }
    }
    dfs(packet, &mut sum);
    println!("{:?}", sum);
    sum
}

fn peek_if_literal(xs: &[u8]) -> bool {
    let (a, b, c) = (xs[3], xs[4], xs[5]);
    let type_id = a * 4 + b * 2 + c;
    if type_id == LITERAL_PACKET_TYPE_ID {
        true
    } else {
        false
    }
}

fn parse_packet(xs: &[u8], curr_idx: &mut usize, padding: bool) -> Packet {
    let (a, b, c) = (xs[*curr_idx + 3], xs[*curr_idx + 4], xs[*curr_idx + 5]);
    let type_id = a * 4 + b * 2 + c;
    if type_id == LITERAL_PACKET_TYPE_ID {
        Packet::LP(parse_literals(xs, curr_idx, padding))
    } else {
        Packet::OP(parse_operator(xs, curr_idx))
    }
}

fn parse_literals(xs: &[u8], curr_idx: &mut usize, padding: bool) -> LiteralPacket {
    let mut literal: i64 = 0;
    let version = xs[*curr_idx] * 4 + xs[*curr_idx + 1] * 2 + xs[*curr_idx + 2];
    let type_id = LITERAL_PACKET_TYPE_ID;
    let mut count = 0;
    *curr_idx += 6;
    while *curr_idx < xs.len() && *curr_idx + 4 < xs.len() {
        literal = literal * 16
            + (xs[*curr_idx + 1] * 8
                + xs[*curr_idx + 2] * 4
                + xs[*curr_idx + 3] * 2
                + xs[*curr_idx + 4]) as i64;
        count += 1;
        *curr_idx += 5;
        if xs[*curr_idx - 5] == 0 {
            break;
        }
    }
    if padding && count * 5 % 4 != 0 {
        *curr_idx += 4 - count * 5 % 4;
    }
    LiteralPacket {
        version,
        type_id,
        literal,
    }
}

fn parse_operator(xs: &[u8], curr_idx: &mut usize) -> OperatorPacket {
    let version = xs[*curr_idx] * 4 + xs[*curr_idx + 1] * 2 + xs[*curr_idx + 2];
    *curr_idx += 3;
    let type_id = xs[*curr_idx] * 4 + xs[*curr_idx + 1] * 2 + xs[*curr_idx + 2];
    *curr_idx += 3;
    let tag = xs[*curr_idx];
    *curr_idx += 1;
    let mut sub_packets: Vec<Packet> = vec![];
    if tag == 1 {
        let length = calculate(xs, curr_idx, 11) as usize;
        *curr_idx += 11;
        for _ in 0..length {
            sub_packets.push(parse_packet(xs, curr_idx, false));
        }
    } else {
        let length = calculate(xs, curr_idx, 15) as usize;
        *curr_idx += 15;
        let next_idx = *curr_idx + length;
        while *curr_idx < next_idx {
            sub_packets.push(parse_packet(xs, curr_idx, false));
        }
    }

    OperatorPacket {
        version,
        type_id,
        tag,
        sub_packets,
    }
}

fn calculate(xs: &[u8], curr_idx: &usize, length: usize) -> usize {
    let mut amount = 0;
    for i in 0..length {
        amount += ((xs[curr_idx + i] as i32) * 2_i32.pow((length - 1 - i) as u32)) as usize;
    }
    amount
}

fn convert_to_binary_from_hex(hex: &str) -> Vec<u8> {
    hex.chars()
        .map(|c| to_binary(c))
        .flat_map(|seg| seg.as_bytes().to_vec())
        .map(|u| u - b'0')
        .collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::convert_to_binary_from_hex;
    use super::first_part;
    use super::second_part;

    #[test]
    fn convert_works() {
        let s = "D2FE28";
        let xs = convert_to_binary_from_hex(s);
        assert_eq!(
            xs,
            [1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]
        );
    }

    #[test]
    fn first_part_works_1() {
        let s = "D2FE28";
        assert_eq!(first_part(s), 6);
    }

    #[test]
    fn first_part_works_2() {
        let s = "38006F45291200";
        assert_eq!(first_part(s), 9);
    }

    #[test]
    fn first_part_works_3() {
        let s = "8A004A801A8002F478";
        assert_eq!(first_part(s), 16);
    }

    #[test]
    fn first_part_works_4() {
        let s = "620080001611562C8802118E34";
        assert_eq!(first_part(s), 12);
    }

    #[test]
    fn first_part_works_5() {
        let s = "C0015000016115A2E0802F182340";
        assert_eq!(first_part(s), 23);
    }

    #[test]
    fn first_part_works_6() {
        let s = "A0016C880162017C3686B18A3D4780";
        assert_eq!(first_part(s), 31);
    }

    #[test]
    fn second_part_works_1() {
        let s = "C200B40A82";
        assert_eq!(second_part(s), 3);
    }

    #[test]
    fn second_part_works_2() {
        let s = "04005AC33890";
        assert_eq!(second_part(s), 54);
    }

    #[test]
    fn second_part_works_3() {
        let s = "880086C3E88112";
        assert_eq!(second_part(s), 7);
    }

    #[test]
    fn second_part_works_4() {
        let s = "CE00C43D881120";
        assert_eq!(second_part(s), 9);
    }

    #[test]
    fn second_part_works_5() {
        let s = "D8005AC2A8F0";
        assert_eq!(second_part(s), 1);
    }

    #[test]
    fn second_part_works_6() {
        let s = "F600BC2D8F";
        assert_eq!(second_part(s), 0);
    }

    #[test]
    fn second_part_works_7() {
        let s = "9C005AC2F8F0";
        assert_eq!(second_part(s), 0);
    }

    #[test]
    fn second_part_works_8() {
        let s = "9C0141080250320F1802104A08";
        assert_eq!(second_part(s), 1);
    }
}
