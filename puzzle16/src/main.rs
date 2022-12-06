fn main() {
    for func in [sixteen::part1] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

const INPUT: &str = include_str!("input.txt");

mod sixteen {
    pub fn part1(input: &str) -> u64 {
        0
    }
}