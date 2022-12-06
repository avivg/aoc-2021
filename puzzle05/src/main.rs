use std::mem::swap;

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn from(s: &str) -> Self {
        let mut parts = s.trim().split(",");
        Self {
            x: parts.next().expect("No first part").parse().expect("First part is not a number"),
            y: parts.next().expect("No second part").parse().expect("Second part is not a number"),
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Coord,
    end: Coord,
}

impl Line {
    fn from(s: &str) -> Self {
        let mut parts = s.trim().split("->");
        let line = Self {
            start: Coord::from(parts.next().expect("No start")),
            end: Coord::from(parts.next().expect("No end")),
        };

        assert!(line.horizontal() || line.vertical() || line.diagonal());

        line
    }

    fn horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn diagonal(&self) -> bool {
        use std::cmp::{max, min};
        
        let start_x = min(self.start.x, self.end.x);
        let end_x = max(self.start.x, self.end.x);

        let start_y = min(self.start.y, self.end.y);
        let end_y = max(self.start.y, self.end.y);

        end_x - start_x == end_y - start_y
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Where's that input file??");
    let floor_dim = 1000;

    // let input = "\
    // 0,9 -> 5,9
    // 8,0 -> 0,8
    // 9,4 -> 3,4
    // 2,2 -> 2,1
    // 7,0 -> 7,4
    // 6,4 -> 2,0
    // 0,9 -> 2,9
    // 3,4 -> 1,4
    // 0,0 -> 8,8
    // 5,5 -> 8,2";
    // let floor_dim = 9;

    let mut vents = vec![];
    for s in input.lines() {
        let l = Line::from(s);
        vents.push(l);
    }

    // vents.iter().for_each(|v| println!("{:?}->{:?} h: {} v: {} d: {}", v.start, v.end, v.horizontal(), v.vertical(), v.diagonal()));

    let mut floor = vec![vec![0; floor_dim+1]; floor_dim+1];
    for gv in &vents {
        let xrange: Vec<_> = if gv.start.x <= gv.end.x {
            (gv.start.x ..= gv.end.x).collect()
        } else {
            (gv.end.x ..= gv.start.x).rev().collect()
        };

        let yrange: Vec<_> = if gv.start.y <= gv.end.y {
            (gv.start.y ..= gv.end.y).collect()
        } else {
            (gv.end.y ..= gv.start.y).rev().collect()
        };

        println!("{:?}, {:?}", xrange, yrange);

        if gv.diagonal() {
            for (x, y) in xrange.iter().zip(yrange.iter()) {
                floor[*y][*x] += 1;
            }
        } else {
            for x in &xrange {
                for y in &yrange {
                    floor[*y][*x] += 1;
                }
            }
        }
    }

    // floor.iter().for_each(|v| println!("{:?}", v));

    let hotspots = floor.iter().fold(0, |acc, v| acc + v.iter().filter(|k| k.cmp(&&1) == std::cmp::Ordering::Greater).count());
    println!("{hotspots}");
}
