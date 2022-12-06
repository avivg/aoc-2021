fn main() {
    for func in [eight::part1, eight::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} ({dur} ns)");
    }
}

mod eight {
    use std::{collections::HashMap, fmt::Debug};

    pub fn part1(input: &str) -> u64 {
        input
            .lines()
            .map(|l| l.split("|").nth(1).unwrap().trim())
            .map(|o| {
                o.split(" ")
                    .map(|comb| match comb.len() {
                        2 | 3 | 4 | 7 => 1,
                        _ => 0,
                    })
                    .sum::<u64>()
            })
            .sum()
    }

    pub fn part2(input: &str) -> u64 {
        input
            .lines()
            .map(SevenDisp::from)
            .fold(0, |acc, d| acc + d.output())
    }

    #[derive(Debug)]
    struct SevenDisp {
        input_digits: Vec<Digit>,
        output_digits: Vec<Digit>,
        decoder: HashMap<u8, u8>,
    }

    impl SevenDisp {
        fn from(input_output: &str) -> Self {
            let mut parts = input_output.split(" | ");
            let mut sd = Self {
                input_digits: parts.next().unwrap().split(" ").map(Digit::from).collect(),
                output_digits: parts.next().unwrap().split(" ").map(Digit::from).collect(),
                decoder: HashMap::new(),
            };
            sd.decode();
            sd
        }

        fn decode(&mut self) {
            let mut displays = [0; 10];
            // Find 1, 4, 7, 8
            for d in self.input_digits.iter() {
                match d.num_segments() {
                    2 => displays[1] = d.signals,
                    3 => displays[7] = d.signals,
                    4 => displays[4] = d.signals,
                    7 => displays[8] = d.signals,
                    _ => {}
                }
            }
            // Find 3 - the only five segments number with complete overlap with 1 (2 and 5 are the others)
            for d in self.input_digits.iter() {
                if d.num_segments() == 5 && d.signals & displays[1] == displays[1] {
                    displays[3] = d.signals;
                    break;
                }
            }
            for d in self.input_digits.iter() {
                if d.num_segments() == 6 {
                    if d.signals & displays[1] != displays[1] {
                        // found 6 - the only six segments number with incomplete overlap with 1 (0 and 9 are the others)
                        displays[6] = d.signals;
                    } else if d.signals & displays[3] == displays[3] {
                        // found 9 - the only six segments number with complete overlap with 3 (0 and 6 are the others)
                        displays[9] = d.signals;
                    } else {
                        // found 0 - the only six segment number that doesn't fit any of the previous predicates
                        displays[0] = d.signals;
                    }
                }
            }
            for d in self.input_digits.iter() {
                if d.num_segments() == 5 {
                    if d.signals == displays[3] {
                        continue;
                    }
                    if d.signals & displays[9] == d.signals {
                        // found 5 - the only 5 segments number with complete overlap with 9
                        displays[5] = d.signals;
                    } else {
                        // found 2 - the only one left
                        displays[2] = d.signals;
                    }
                }
            }
            for (display, signal) in displays.iter().enumerate() {
                self.decoder.insert(*signal, display as u8);
            }
        }

        fn output(&self) -> u64 {
            self.output_digits
                .iter()
                .map(|d| self.decoder[&d.signals])
                .fold(0, |acc, d| (acc * 10) + d as u64)
        }
    }

    #[derive(Clone, Copy)]
    struct Digit {
        signals: u8,
    }

    impl Digit {
        fn from(s: &str) -> Self {
            Self {
                signals: s
                    .chars()
                    .map(|c| (c as u8) - ('a' as u8))
                    .map(|b| 1 << b)
                    .fold(0, |acc, b| acc | b),
            }
        }

        fn num_segments(&self) -> usize {
            self.signals.count_ones() as usize
        }
    }

    impl Debug for Digit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Digit")
                .field("signals", &format!("{:07b}", self.signals))
                .finish()
        }
    }
}

const INPUT: &str = include_str!("input.txt");
