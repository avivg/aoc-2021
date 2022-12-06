fn main() {
    for func in [fifteen::part1, fifteen::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_micros();

        println!("{res} [{dur} us]");
    }
}

const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = r"1163751742
// 1381373672
// 2136511328
// 3694931569
// 7463417111
// 1319128137
// 1359912421
// 3125421639
// 1293138521
// 2311944581";

#[allow(unused)]
mod fifteen {
    use petgraph::{
        algo::{astar, dijkstra},
        prelude::*,
    };

    pub fn part1(input: &str) -> u64 {
        let chiton_density = DensityMap::from(input);
        let riskmap = RiskMap::from(chiton_density);
        riskmap.shortest()
    }

    pub fn part2(input: &str) -> u64 {
        let mut chiton_density = DensityMap::from(input);
        chiton_density.dup_horizontally(5);
        chiton_density.dup_verically(5);

        let riskmap = RiskMap::from(chiton_density);
        riskmap.shortest()
    }

    struct DensityMap {
        chitons: Vec<Vec<u64>>,
    }

    impl DensityMap {
        fn from(input: &str) -> Self {
            let mut map = vec![];
            for l in input.lines() {
                let mut cur = vec![];
                for c in l.chars() {
                    cur.push(c as u64 - '0' as u64);
                }
                map.push(cur);
            }
            Self { chitons: map }
        }

        fn dup_horizontally(&mut self, times: usize) {
            for i in 0..self.chitons.len() {
                let line = self.chitons[i].clone();
                for dup_idx in 1..times {
                    let mut dup_line = Self::duplicate(&line, dup_idx as u64);
                    self.chitons[i].append(&mut dup_line);
                }
            }
        }

        fn dup_verically(&mut self, times: usize) {
            let orig_len = self.chitons.len();
            for dup_idx in 1..times {
                for i in 0..orig_len {
                    let dup_line = Self::duplicate(&self.chitons[i], dup_idx as u64);
                    self.chitons.push(dup_line);
                }
            }
        }

        fn duplicate(line: &Vec<u64>, offset: u64) -> Vec<u64> {
            line.iter().map(|n| (((n - 1) + offset) % 9) + 1).collect()
        }
    }

    impl std::fmt::Debug for DensityMap {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut dm = f.debug_struct("DensityMap");
            for (i, v) in self.chitons.iter().enumerate() {
                dm.field(&format!("{i:03}"), &format!("{v:?}"));
            }
            dm.finish()
        }
    }

    struct RiskMap {
        graph: UnGraph<u64, ()>,
        start: NodeIndex,
        end: NodeIndex,
    }

    impl RiskMap {
        fn from(densities: DensityMap) -> Self {
            let mut graph = UnGraph::new_undirected();
            let mut node_map = vec![];
            for cd_line in densities.chitons {
                let mut cur = vec![];
                for chiton_density in cd_line {
                    cur.push(graph.add_node(chiton_density));
                }
                node_map.push(cur);
            }

            let h = node_map.len();
            let w = node_map[0].len();

            for r in 0..h {
                for c in 0..w {
                    if r < h - 1 {
                        graph.add_edge(node_map[r][c], node_map[r + 1][c], ());
                    }
                    if c < w - 1 {
                        graph.add_edge(node_map[r][c], node_map[r][c + 1], ());
                    }
                }
            }

            Self {
                graph,
                start: node_map[0][0],
                end: node_map[h - 1][w - 1],
            }
        }

        fn shortest(&self) -> u64 {
            // let costs = dijkstra(&self.graph, self.start, Some(self.end), |e| *self.graph.node_weight(e.target()).unwrap());
            // costs[&self.end]
            let sp = astar(
                &self.graph,
                self.start,
                |ni| ni == self.end,
                |e| *self.graph.node_weight(e.target()).unwrap(),
                |_| 0,
            );
            let (cost, nodes) = sp.unwrap();
            dbg! {
                nodes.iter().map(|ni| self.graph.node_weight(*ni).unwrap()).sum::<u64>()//collect::<Vec<_>>()
            };
            cost
        }
    }
}
