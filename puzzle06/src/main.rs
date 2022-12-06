fn main() {
    for gens in [80, 256] {
        let start = std::time::Instant::now();
        let res = six::population(gens);
        let dur = start.elapsed().as_nanos();

        println!("{res} ({dur} ns)");
    }
}

mod six {
    pub fn population(gen: usize) -> u128 {
        let mut lfs = initial_population();
        for _ in 0..gen {
            next_gen(&mut lfs);
        }
        lfs.iter().sum()
    }

    fn initial_population() -> [u128; 9] {
        let mut res = [0;9];
        for lf in crate::INPUT.split(",") {
            res[lf.parse::<usize>().unwrap()] += 1;
        }
        res
    }

    fn next_gen(lfs: &mut [u128; 9]) {
        let pregnant = lfs[0];
        for i in 0..8 {
            lfs[i] = lfs[i+1];
        }
        lfs[8] = pregnant;
        lfs[6] += pregnant;
    }
}

const INPUT: &str = include_str!("input.txt");