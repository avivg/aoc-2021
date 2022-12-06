fn main() {
    for func in [eleven::part1, eleven::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} ({dur} ns)");
    }
}

mod eleven {
    use petgraph::prelude::UnGraph;

    
    pub fn part1(input: &str) -> u64 {
        let mut octo_graph = build_graph(input);
        let mut flashes = 0;
        print(&octo_graph);
        for _ in 0..100 {
            flashes += step(&mut octo_graph);
            print(&octo_graph);
        }
        flashes
    }
    
    pub fn part2(input: &str) -> u64 {
        let mut octo_graph = build_graph(input);
        let mut step_count = 0;
        loop {
            let flashes = step(&mut octo_graph);
            step_count += 1;
            if flashes == octo_graph.node_count().try_into().unwrap() {
                return step_count;
            }
        }
    }

    fn print(graph: &UnGraph<u32, ()>) {
        #![allow(unused)]
        // use crate::DIM;
        // println!();
        // let mut ws = graph.node_weights();
        // for _ in 0..DIM {
        //     for _ in 0..DIM {
        //         print!("{}", ws.next().unwrap());
        //     }
        //     println!();
        // }
    }

    fn build_graph(input: &str) -> UnGraph<u32, ()> {
        let mut nodes = vec![];
        let mut graph = UnGraph::new_undirected();
        for l in input.lines() {
            nodes.push(vec![]);
            let cur = nodes.last_mut().unwrap();
            for c in l.as_bytes() {
                cur.push(graph.add_node((c - '0' as u8) as u32))
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
                if row < h-1 && col < w-1 {
                    graph.add_edge(nodes[row][col], nodes[row+1][col+1], ());
                }
                if row > 0 && col < w-1 {
                    graph.add_edge(nodes[row][col], nodes[row-1][col+1], ());
                }
            }
        }
        graph
    }

    fn step(graph: &mut UnGraph<u32, ()>) -> u64 {
        let mut flashing_stack = vec![];

        for ni in graph.node_indices() {
            let n = graph.node_weight_mut(ni).unwrap();
            *n += 1;
            if *n == 10 {
                flashing_stack.push(ni);
            }
        }

        let mut num_flashes = 0;
        while !flashing_stack.is_empty() {
            let next_flashing = flashing_stack.pop().unwrap();
            let energy = graph.node_weight_mut(next_flashing).unwrap();
            *energy = 0; // flashed
            num_flashes += 1;

            let flashing_neighbors: Vec<_> = graph.neighbors(next_flashing).collect();
            for neighbor in flashing_neighbors {
                let neighbor_energy = graph.node_weight_mut(neighbor).unwrap();
                if *neighbor_energy == 0 {
                    continue; // already flashed
                }
                *neighbor_energy += 1;
                if *neighbor_energy == 10 {
                    flashing_stack.push(neighbor);
                }
            }
        }

        num_flashes
    }
}

const INPUT: &str = include_str!("input.txt");

// const INPUT: &str = "11111
// 19991
// 19191
// 19991
// 11111";

// const DIM: usize = 5;