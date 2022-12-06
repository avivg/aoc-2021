fn main() {
    for func in [twelve::part1, twelve::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_micros();

        println!("{res} ({dur} ms)");
    }
}

const INPUT: &str = include_str!("input.txt");

#[allow(unused)]
mod twelve {
    use itertools::Itertools;
    use petgraph::{algo::all_simple_paths, prelude::*};
    use std::collections::{HashMap, HashSet};

    pub fn part1(input: &str) -> usize {
        let caverns = Caverns::from(input);
        let smalls = caverns.connected_small_caverns_graph();
        smalls.num_paths()
    }

    pub fn part2(input: &str) -> usize {
        let caverns = Caverns::from(input);
        let smalls = caverns.connected_small_caverns_graph_single_double_take();
        smalls.iter().map(|g| g.num_paths()).sum()
    }

    // #[derive(Debug)]
    struct Caverns {
        graph: UnGraph<String, ()>,
        start: NodeIndex,
        end: NodeIndex,
    }

    impl From<&str> for Caverns {
        fn from(input: &str) -> Self {
            let mut nodes = HashMap::new();
            let mut graph = UnGraph::new_undirected();
            for l in input.lines() {
                let (src, dst) = l.split("-").collect_tuple().unwrap();
                let src_n = *nodes
                    .entry(src)
                    .or_insert_with(|| graph.add_node(String::from(src)));
                let dst_n = *nodes
                    .entry(dst)
                    .or_insert_with(|| graph.add_node(String::from(dst)));
                graph.add_edge(src_n, dst_n, ());
            }

            Self {
                graph,
                start: *nodes.get("start").unwrap(),
                end: *nodes.get("end").unwrap(),
            }
        }
    }

    impl Caverns {
        fn connected_small_caverns_graph(&self) -> Self {
            let mut small_nodes = HashMap::new();

            // First add all small caverns and edges between them
            let mut graph = self.graph.filter_map(
                |_, n| {
                    if n.chars().all(char::is_lowercase) {
                        Some(n.clone())
                    } else {
                        None
                    }
                },
                |_, _| Some(()),
            );

            // Populate the map
            for ni in graph.node_indices() {
                small_nodes.insert(graph.node_weight(ni).unwrap().clone(), ni);
            }

            // Create edges from each large cavern
            for large_ni in self.graph.node_indices() {
                let large_n = self.graph.node_weight(large_ni).unwrap();
                if large_n.chars().all(char::is_uppercase) {
                    let nbrs: Vec<_> = self.graph.neighbors(large_ni).collect();
                    for i in 0..nbrs.len() - 1 {
                        for j in i + 1..nbrs.len() {
                            let small_cav_src = self.graph.node_weight(nbrs[i]).unwrap();
                            let small_cav_src = small_nodes.get(small_cav_src).unwrap();
                            let small_cav_dst = self.graph.node_weight(nbrs[j]).unwrap();
                            let small_cav_dst = small_nodes.get(small_cav_dst).unwrap();

                            graph.add_edge(*small_cav_src, *small_cav_dst, ());
                        }
                    }
                }
            }

            Self {
                graph,
                start: *small_nodes.get("start").unwrap(),
                end: *small_nodes.get("end").unwrap(),
            }
        }

        fn connected_small_caverns_graph_single_double_take(&self) -> Vec<Self> {
            let mut small_nodes = HashMap::new();

            // First add all small caverns and edges between them
            let mut graph = self.graph.filter_map(
                |_, n| {
                    if n.chars().all(char::is_lowercase) {
                        Some(n.clone())
                    } else {
                        None
                    }
                },
                |_, _| Some(()),
            );

            // Populate the map
            for ni in graph.node_indices() {
                small_nodes.insert(graph.node_weight(ni).unwrap().clone(), ni);
            }

            let mut large_nis = vec![];

            // Create edges from each large cavern
            for ni in self.graph.node_indices() {
                let ns = self.graph.node_weight(ni).unwrap();
                if ns.chars().all(char::is_uppercase) {
                    large_nis.push(ni);
                    let nbrs: Vec<_> = self.graph.neighbors(ni).collect();
                    for i in 0..nbrs.len() - 1 {
                        for j in i + 1..nbrs.len() {
                            let small_cav_src = self.graph.node_weight(nbrs[i]).unwrap();
                            let small_cav_src = small_nodes.get(small_cav_src).unwrap();
                            let small_cav_dst = self.graph.node_weight(nbrs[j]).unwrap();
                            let small_cav_dst = small_nodes.get(small_cav_dst).unwrap();

                            graph.add_edge(*small_cav_src, *small_cav_dst, ());
                        }
                    }
                }
            }

            let res = vec![];


            // Self {
            //     graph,
            //     start: *small_nodes.get("start").unwrap(),
            //     end: *small_nodes.get("end").unwrap(),
            // }

            res
        }

        fn num_paths(&self) -> usize {
            let paths = all_simple_paths::<Vec<_>, _>(&self.graph, self.start, self.end, 0, None);
            dbg!(paths.peekable().peek());
            0
            // paths.count()
        }
    }
}
