fn main() {
    for func in [thirteen::part1, thirteen::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} ({dur} ns)");
    }
}

const INPUT: &str = include_str!("input.txt");

mod thirteen {
    use std::collections::HashSet;
    use itertools::Itertools;

    pub fn part1(input: &str) -> usize {
        let mut l = input.lines();
        let mut paper = Paper::from(&mut l);
        paper.fold(l.next().unwrap());
        paper.dots().len()
    }

    pub fn part2(input: &str) -> usize {
        let mut l = input.lines();
        let mut paper = Paper::from(&mut l);
        l.for_each(|i| paper.fold(i));
        let mut map = vec![vec!["."; 40]; 6];
        for d in paper.dots() {
            map[d.1][d.0] = "#";
        }
        for l in map {
            println!("{}", l.iter().join(" "));
        }
        0
    }

    #[derive(Hash, Eq, PartialEq)]
    struct Dot(usize, usize);

    impl TryFrom<&str> for Dot {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if let Some((Ok(x), Ok(y))) = value.split(",").map(|s| s.parse::<usize>()).collect_tuple::<(Result<_,_>, Result<_,_>)>(){
                Ok(Self(x, y))
            } else {
                Err("no more values")
            }
        }
    }

    struct Paper {
        dots: HashSet<Dot>,
    }

    impl Paper {
        fn from<'a>(it: &mut impl Iterator<Item = &'a str>) -> Self {
            let mut paper = Self { dots: HashSet::new() };
            for s in it {
                if let Ok(dot) = Dot::try_from(s) {
                    paper.dots.insert(dot);
                } else {
                    break;
                }
            }
            paper
        }

        fn fold(&mut self, instr: &str) {
                let re = regex::Regex::new(r"fold along (x|y)=(\d+)").unwrap();
                let caps = re.captures(instr).unwrap();
                let fold_line = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let folder: Box<dyn Folder> = match caps.get(1).unwrap().as_str() {
                    "x" => Box::new(XFolder(fold_line)),
                    "y" => Box::new(YFolder(fold_line)),
                    _ => unreachable!()
                };
                self.dots = self.dots.drain().map(|d| folder.fold(d)).collect()
        }

        fn dots<'a>(&'a self) -> &'a HashSet<Dot> {
            &self.dots
        }
    }

    trait Folder {
        fn fold(&self, d: Dot)->Dot;
    }

    struct XFolder(usize);
    impl Folder for XFolder {
        fn fold(&self, mut d: Dot)->Dot {
            let dist = self.0.abs_diff(d.0);
            d.0 = self.0 - dist;
            d
        }
    }

    struct YFolder(usize);
    impl Folder for YFolder {
        fn fold(&self, mut d: Dot)->Dot {
            let dist = self.0.abs_diff(d.1);
            d.1 = self.0 - dist;
            d
        }
    }


}