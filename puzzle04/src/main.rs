#[derive(Debug, Clone, Copy)]
struct Cell {
    num: u32,
    marked: bool,
}

impl Cell {
    fn new() -> Self {
        Self {
            num: 0,
            marked: false,
        }
    }

    fn set(&mut self, num: u32) {
        self.num = num;
    }

    fn mark(&mut self, num: u32) {
        if num == self.num {
            self.marked = true;
        }
    }

    fn score(&self) -> u64 {
        if !self.marked {
            self.num as u64
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Cells {
    cells: [Cell; 5],
}

impl Cells {
    fn new() -> Self {
        Self {
            cells: [Cell::new(); 5],
        }
    }

    fn set(&mut self, idx: usize, num: u32) {
        self.cells[idx].set(num);
    }

    fn mark(&mut self, num: u32) {
        self.cells.iter_mut().for_each(|c| c.mark(num));
    }

    fn check(&self) -> bool {
        self.cells.iter().all(|c| c.marked)
    }

    fn score(&self) -> u64 {
        self.cells.iter().fold(0, |s, c| s + c.score())
    }
}

#[derive(Debug)]
struct Board {
    rows: [Cells; 5],
    cols: [Cells; 5],
}

impl Board {
    fn build<'a, I: Iterator<Item = &'a str>>(lines: &mut I) -> Self {
        let mut board = Self {
            rows: [Cells::new(); 5],
            cols: [Cells::new(); 5],
        };

        for i in 0..5 {
            board.parse_line(i, lines.next().unwrap());
        }

        board
    }

    fn parse_line<'a>(&mut self, idx: usize, line: &'a str) {
        let nums: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for i in 0..5 {
            self.rows[idx].set(i, nums[i]);
            self.cols[i].set(idx, nums[i]);
        }
    }

    fn mark(&mut self, n: u32) {
        self.rows.iter_mut().for_each(|row| row.mark(n));
        self.cols.iter_mut().for_each(|col| col.mark(n));
    }

    fn win(&self, n: u32) -> Option<u64> {
        if self.rows.iter().any(|r| r.check()) || self.cols.iter().any(|c| c.check()) {
            return Some(self.score(n));
        }
        None
    }

    fn score(&self, n: u32) -> u64 {
        let board_sum = self.rows.iter().fold(0, |s, cells| s + cells.score());
        board_sum * n as u64
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut lines = input.lines();
    let draw_order: Vec<_> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    // println!("{:?}", draw_order);
    let mut boards = vec![];
    while let Some(_empty) = lines.next() {
        boards.push(Board::build(&mut lines));
    }

    'game: for draw in draw_order {
        // println!("Drawn: {draw}");
        for b in &mut boards {
            if let Some(_) = b.win(draw) {
                continue; // ignore won tables (won before mark)
            }
            b.mark(draw);
            if let Some(final_score) = b.win(draw) {
                // dbg!(b);
                println!("{final_score}");
                // break 'game; // break to see the first winner
            }
        }
    }
}
