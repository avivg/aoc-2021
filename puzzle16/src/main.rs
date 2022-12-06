fn main() {
    for func in [sixteen::part1] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

// const INPUT: &str = include_str!("input.txt");
const INPUT: &str = "750F";

#[allow(unused)]
mod sixteen {

    pub fn part1(input: &str) -> u64 {
        let bits = dbg!(bits_from_hex(input));
        let packet = Packet::from(bits.into_iter());
        packet.sum_vers()
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

    fn next_num(it: &mut impl Iterator<Item = u8>, len: usize) -> u64 {
        let mut res = 0;
        for _ in 0..len {
            res <<= 1;
            res |= it.next().unwrap() as u64;
        }
        res
    }

    trait PacketType {}

    struct Packet {
        ver: u8,
        // ptype: Box<dyn PacketType>,
        sub_packets: Vec<Packet>,
    }

    impl Packet {
        fn from(mut it: impl Iterator<Item = u8>) -> Self {
            let ver = next_num(&mut it, 3);
            Self {
                ver: ver as u8,
                sub_packets: vec![],
            }
        }

        fn sum_vers(&self) -> u64 {
            self.ver as u64
                + self
                    .sub_packets
                    .iter()
                    .fold(0u64, |acc, sp| acc + sp.sum_vers())
        }
    }
}
