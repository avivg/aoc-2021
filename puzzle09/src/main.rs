fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}

fn part1(input: &str) -> u64 {
    let mut map = vec![];
    for l in input.lines() {
        map.push(l.chars().map(DataPoint::from).collect::<Vec<_>>())
    }

    let width = map[0].len();

    for v in map.iter_mut() {
        for i in 0..(width-1) {
            if v[i].height < v[i+1].height {
                v[i+1].low_point = false;
            } else if v[i].height > v[i+1].height {
                v[i].low_point = false;
            } else {
                v[i].low_point = false;
                v[i + 1].low_point = false;
            }
        }
    }
    for i in 0..(map.len() - 1) {
        for j in 0..width {
            if map[i][j].height < map[i+1][j].height {
                map[i+1][j].low_point = false;
            } else if map[i][j].height > map[i+1][j].height {
                map[i][j].low_point = false;
            } else {
                map[i][j].low_point = false;
                map[i+1][j].low_point = false;
            }
        }
    }
    map.iter().fold(0, |acc, l| {
        acc + l.iter().filter(|dp| dp.low_point).map(|dp| dp.risk()).sum::<u64>()
    })
}

struct DataPoint {
    height: u8,
    low_point: bool,
}

impl DataPoint {
    fn from(c: char) -> Self {
        Self {
            low_point: true,
            height: c as u8 - '0' as u8,
        }
    }

    fn risk(&self) -> u64 {
        self.height as u64 + 1
    }
}

use petgraph::prelude::*;

fn part2(input: &str) -> usize {
    
    let g: UnGraph<u8, ()> = build_graph(input);
    let basins = g.filter_map(|_, n| match *n { 9 => None, d => Some(d)}, |_,_| Some(()));
    let mut ccps = petgraph::algo::tarjan_scc(&basins).iter().map(|ccp| ccp.len()).collect::<Vec<usize>>();
    ccps.sort();
    ccps.iter().rev().take(3).fold(1, |acc, v| acc * *v)
}

fn build_graph(input: &str) -> UnGraph<u8, ()> {
    let mut nodes = vec![];
    let mut graph = UnGraph::new_undirected();
    for l in input.lines() {
        nodes.push(vec![]);
        let cur = nodes.last_mut().unwrap();
        for c in l.as_bytes() {
            cur.push(graph.add_node(c - '0' as u8))
        }
    }
    let h = nodes.len();
    let w = nodes[0].len();
    for row in 0..h {
        for col in 0..w {
            if row < h-1 {
                graph.add_edge(nodes[row][col], nodes[row+1][col], ());
            }
            if col < w-1 {
                graph.add_edge(nodes[row][col], nodes[row][col+1], ());
            }
        }
    }
    graph
}

const INPUT: &str = include_str!("input.txt");
