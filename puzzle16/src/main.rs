fn main() {
    for func in [sixteen::part1, sixteen::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

const INPUT: &str = include_str!("input.txt");

mod sixteen {

    pub fn part1(input: &str) -> u64 {
        let mut bits = bits_from_hex(input).into_iter();
        let packet = Packet::next(&mut bits);
        packet.sum_vers()
    }

    pub fn part2(input: &str) -> u64 {
        let mut bits = bits_from_hex(input).into_iter();
        let packet = Packet::next(&mut bits);
        packet.value()
    }

    fn bits_from_hex(s: &str) -> Vec<u8> {
        s.chars()
            .map(|c| match c {
                'A'..='F' => c as u8 - 'A' as u8 + 10,
                '0'..='9' => c as u8 - '0' as u8,
                _ => unreachable!(),
            })
            .map(|nibble| [8, 4, 2, 1].map(|b| (b & nibble != 0) as u8))
            .flatten()
            .collect()
    }

    fn next_num(bitstream: &mut impl Iterator<Item = u8>, len: usize) -> u64 {
        let mut res = 0;
        for _ in 0..len {
            res <<= 1;
            res |= bitstream.next().unwrap() as u64;
        }
        res
    }

    #[derive(Debug)]
    struct Packet {
        ver: u8,
        opcode: u8,
        payload: PacketPayload,
    }

    impl Packet {
        fn next(bitstream: &mut impl Iterator<Item = u8>) -> Self {
            let ver = next_num(bitstream, 3);
            let opcode = next_num(bitstream, 3);
            Self {
                ver: ver as u8,
                opcode: opcode as u8,
                payload: PacketPayload::next(opcode as u8, bitstream),
            }
        }

        fn sum_vers(&self) -> u64 {
            self.ver as u64
                + match &self.payload {
                    PacketPayload::Literal(_) => 0,
                    PacketPayload::SubPackets(spkts) => {
                        spkts.packets.iter().map(|pkt| pkt.sum_vers()).sum()
                    }
                }
        }

        fn size(&self) -> usize {
            3 + // ver size
            3 + // op size
            self.payload.size()
        }

        fn value(&self) -> u64 {
            match self.opcode {
                0 => self.payload.sum(),
                1 => self.payload.prod(),
                2 => self.payload.min(),
                3 => self.payload.max(),
                4 => self.payload.as_literal().value,
                5 => self.payload.gt(),
                6 => self.payload.lt(),
                7 => self.payload.eq(),
                _ => unreachable!(),
            }
        }
    }

    #[derive(Debug)]
    enum PacketPayload {
        Literal(LiteralValue),
        SubPackets(PacketString),
    }

    impl PacketPayload {
        fn next(opcode: u8, bitstream: &mut impl Iterator<Item = u8>) -> Self {
            match opcode {
                4 => PacketPayload::Literal(LiteralValue::next(bitstream)),
                _ => PacketPayload::SubPackets(PacketString::next(bitstream)),
            }
        }

        fn size(&self) -> usize {
            match &self {
                PacketPayload::Literal(l) => l.size,
                PacketPayload::SubPackets(pkt_str) => pkt_str.size(),
            }
        }

        fn as_literal(&self) -> &LiteralValue {
            match &self {
                PacketPayload::Literal(l) => &l,
                PacketPayload::SubPackets(_) => panic!("Payload is not a literal!"),
            }
        }

        fn sum(&self) -> u64 {
            match &self {
                PacketPayload::Literal(_) => unreachable!(),
                PacketPayload::SubPackets(pkt_str) => {
                    pkt_str.packets.iter().fold(0, |acc, pkt| acc + pkt.value())
                }
            }
        }
        fn prod(&self) -> u64 {
            match &self {
                PacketPayload::Literal(_) => unreachable!(),
                PacketPayload::SubPackets(pkt_str) => {
                    pkt_str.packets.iter().fold(1, |acc, pkt| acc * pkt.value())
                }
            }
        }
        fn min(&self) -> u64 {
            match &self {
                PacketPayload::Literal(_) => unreachable!(),
                PacketPayload::SubPackets(pkt_str) => {
                    pkt_str.packets.iter().map(|p| p.value()).min().unwrap()
                }
            }
        }
        fn max(&self) -> u64 {
            match &self {
                PacketPayload::Literal(_) => unreachable!(),
                PacketPayload::SubPackets(pkt_str) => {
                    pkt_str.packets.iter().map(|p| p.value()).max().unwrap()
                }
            }
        }
        fn gt(&self) -> u64 {
            match &self {
                PacketPayload::Literal(_) => unreachable!(),
                PacketPayload::SubPackets(pkt_str) => {
                    if pkt_str.packets[0].value() > pkt_str.packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
            }
        }
        fn lt(&self) -> u64 {
            match &self {
                PacketPayload::Literal(_) => unreachable!(),
                PacketPayload::SubPackets(pkt_str) => {
                    if pkt_str.packets[0].value() < pkt_str.packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
            }
        }
        fn eq(&self) -> u64 {
            match &self {
                PacketPayload::Literal(_) => unreachable!(),
                PacketPayload::SubPackets(pkt_str) => {
                    if pkt_str.packets[0].value() == pkt_str.packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
            }
        }
    }

    #[derive(Debug)]
    struct LiteralValue {
        value: u64,
        size: usize,
    }

    impl LiteralValue {
        fn next(bitstream: &mut impl Iterator<Item = u8>) -> Self {
            let mut value = 0;
            let mut size = 0;
            while let Some(cont_bit) = bitstream.next() {
                let nibble = next_num(bitstream, 4);
                value = (value << 4) | nibble;
                size += 5;
                if cont_bit == 0 {
                    break;
                }
            }
            Self { value, size }
        }
    }

    #[derive(Debug)]
    enum LengthType {
        Bits,
        PktCount,
    }

    #[derive(Debug)]
    struct PacketString {
        length_type: LengthType,
        // length: u64,
        packets: Vec<Packet>,
    }

    impl PacketString {
        fn next(bitstream: &mut impl Iterator<Item = u8>) -> Self {
            let length_type_id = bitstream.next().unwrap();
            let (length_type, length) = match length_type_id {
                0 => (LengthType::Bits, next_num(bitstream, 15)),
                1 => (LengthType::PktCount, next_num(bitstream, 11)),
                _ => unreachable!(),
            };
            let mut packets: Vec<Packet> = vec![];
            match length_type {
                LengthType::Bits => {
                    while packets.iter().map(|p| p.size()).sum::<usize>() < length as usize {
                        packets.push(Packet::next(bitstream));
                    }
                }
                LengthType::PktCount => {
                    for _ in 0..length {
                        packets.push(Packet::next(bitstream))
                    }
                }
            }
            Self {
                length_type,
                // length,
                packets,
            }
        }

        fn size(&self) -> usize {
            self.packets
                .iter()
                .fold(0usize, |acc, pkt| acc + pkt.size())
                + match &self.length_type {
                    LengthType::Bits => 16,     // 1 type, 15 length
                    LengthType::PktCount => 12, // 1 type, 11 length
                }
        }
    }

    #[cfg(test)]
    mod tests {
        use std::{iter::once, iter::repeat};

        use super::*;

        #[test]
        fn parse_literal_packet() {
            // ver=2 op=4 nib=A  nib=B  nib=A  nib=B  padding
            // 010   100  1_1010 1_1011 1_1010 0_1011 00
            // ^^^^^^^+++++++^^^^^ ++++ ^^^^^++++++^^^^^
            //    5      3     5     B    D     2    C
            let input = "535BD2C";
            let bits = bits_from_hex(input);
            let packet = dbg! {Packet::next(&mut bits.into_iter())};

            assert_eq!(2, packet.ver);
            assert_eq!(4, packet.opcode);
            assert_eq!(26, packet.size());
            if let PacketPayload::Literal(l) = packet.payload {
                assert_eq!(l.value, 0xABAB);
            } else {
                panic!("Expected a literal")
            }
        }

        #[test]
        fn parse_literal_value() {
            let mut bits = [1u8, 0, 0, 0, 1, 0, 0, 0, 1, 0].into_iter();
            let l = dbg! {LiteralValue::next(&mut bits)};
            assert_eq!(0x12, l.value);
            assert_eq!(10, l.size);
        }

        #[test]
        fn parse_empty_pkt_string() {
            let mut bits = [0u8; 16].into_iter();
            let pkt_str = dbg!(PacketString::next(&mut bits));
            assert_eq!(16, pkt_str.size());

            let mut bits = once(1u8).chain(repeat(0u8)).take(12);
            let pkt_str = dbg!(PacketString::next(&mut bits));
            assert_eq!(12, pkt_str.size());
        }

        #[test]
        fn parse_pkt_string_by_count() {
            let lit_pkt1 = literal();
            let lit_pkt2 = literal();
            let mut bits = once(1u8) // len type = 1 : count
                .chain(repeat(0).take(9))
                .chain(once(1))
                .chain(once(0)) // len(11) = 00000000010
                .chain(lit_pkt1)
                .chain(lit_pkt2);
            let pkt_str = dbg!(PacketString::next(&mut bits));

            assert_eq!(26 + 26 + 11 + 1, pkt_str.size());
            assert_eq!(2, pkt_str.packets.len());
            if let PacketPayload::Literal(l) = &pkt_str.packets.first().unwrap().payload {
                assert_eq!(l.value, 0xABAB);
            } else {
                panic!("Expected literal");
            }
            if let PacketPayload::Literal(l) = &pkt_str.packets.last().unwrap().payload {
                assert_eq!(l.value, 0xABAB);
            } else {
                panic!("Expected literal");
            }
        }

        #[test]
        fn parse_pkt_string_by_length() {
            let lit_pkt1 = literal();
            let lit_pkt2 = literal();
            let mut bits = once(0u8) // len type = 0 : length
                .chain([0u8, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0].into_iter()) // len(15) = 000000000110100
                .chain(lit_pkt1)
                .chain(lit_pkt2);
            let pkt_str = dbg!(PacketString::next(&mut bits));

            assert_eq!(26 + 26 + 15 + 1, pkt_str.size());
            assert_eq!(2, pkt_str.packets.len());
            if let PacketPayload::Literal(l) = &pkt_str.packets.first().unwrap().payload {
                assert_eq!(l.value, 0xABAB);
            } else {
                panic!("Expected literal");
            }
            if let PacketPayload::Literal(l) = &pkt_str.packets.last().unwrap().payload {
                assert_eq!(l.value, 0xABAB);
            } else {
                panic!("Expected literal");
            }
        }

        #[test]
        fn parse_compound_pkt() {
            let mut bits = repeat(1)
                .take(6) // ver = 7, op = 7
                .chain(two_literals_chain())
                .chain(repeat(1).take(2)); // don't care padding
            let pkt = dbg!(Packet::next(&mut bits));

            assert_eq!(pkt.size(), two_literals_chain().count() + 6);
        }

        fn literal() -> impl Iterator<Item = u8> {
            let input = "535BD2C";
            let mut bits = bits_from_hex(input);
            bits.pop();
            bits.pop();
            bits.into_iter()
        }

        fn two_literals_chain() -> impl Iterator<Item = u8> {
            let lit_pkt1 = literal();
            let lit_pkt2 = literal();
            once(0u8) // len type = 0 : length
                .chain([0u8, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0].into_iter()) // len(15) = 000000000110100
                .chain(lit_pkt1)
                .chain(lit_pkt2)
        }

        #[test]
        fn example1() {
            let input = "38006F45291200";
            let bits = bits_from_hex(input);
            let pkt = Packet::next(&mut bits.into_iter());
            dbg!(&pkt);
            assert_eq!(pkt.ver, 1);
            assert_eq!(pkt.opcode, 6);
        }

        #[test]
        fn real_part1() {
            assert_eq!(part1(crate::INPUT), 969);
        }

        #[test]
        fn real_part2() {
            assert_eq!(part2(crate::INPUT), 124921618408u64);
        }
    }
}
