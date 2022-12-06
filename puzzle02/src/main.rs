struct Progress {
    aim: i64,
    depth: i64,
    dist: i64,
}

impl Progress {
    fn new() -> Self {
        Self {
            aim: 0,
            depth: 0,
            dist: 0,
        }
    }

    fn down(&mut self, x: i64) {
        self.aim += x;
    }

    fn up(&mut self, x: i64) {
        self.aim -= x;
    }

    fn fwd(&mut self, x: i64) {
        self.dist += x;
        self.depth += self.aim * x;
    }

    pub fn parse(&mut self, isntruction: &str) {
        let mut tokens = isntruction.split_whitespace();
        let cmd = tokens.next().unwrap();
        let param = tokens.next().unwrap().parse::<i64>().unwrap();
        match cmd {
            "up" => self.up(param),
            "down" => self.down(param),
            "forward" => self.fwd(param),
            _ => panic!("Unexpected token"),
        };
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();
    let mut progress = Progress::new();
    for l in lines {
        progress.parse(l);
    }
    println!("{}", progress.depth * progress.dist);
}
