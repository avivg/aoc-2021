use std::collections::HashMap;

fn main() {
    let res = part1(INPUT);
    println!("{res}");
}

const INPUT: &str = include_str!("input.txt");

fn part1(input: &str) -> usize {
    use itertools::Itertools;

    let mut lines = input.lines();
    let template_str = lines.next().unwrap();
    lines.next(); // skip empty
    let a = &template_str[..template_str.len()-1];
    let b = &template_str[1..];
    let template_pairs = a.chars().zip(b.chars());
    let mut rules = HashMap::new();
    for l in lines {
        let (p, c) = l.split(" -> ").collect_tuple::<(&str, &str)>().unwrap();
        let c = c.chars().nth(0).unwrap();
        let (c1, c2) = p.chars().collect_tuple::<(char, char)>().unwrap();
        rules.insert((c1, c2), ((c1, c), (c, c2)));
    }
    let mut pair_counters = HashMap::new();
    // starting state
    for p in template_pairs {
        pair_counters.entry(p).and_modify(|count| *count += 1).or_insert(1usize);
    }

    for _ in 0..40 {
        let mut next_count = HashMap::new();
        for (p, count) in pair_counters {
            let (p1, p2) = rules.get(&p).unwrap();
            next_count.entry(*p1).and_modify(|cnt| *cnt += count).or_insert(count);
            next_count.entry(*p2).and_modify(|cnt| *cnt += count).or_insert(count);
        }
        pair_counters = next_count;
    }

    let mut c_counters = HashMap::new();
    for ((c, _), count) in pair_counters {
        c_counters.entry(c).and_modify(|cnt| *cnt += count).or_insert(count);
    }
    c_counters.entry(template_str.chars().last().unwrap()).and_modify(|cnt| *cnt += 1).or_insert(1);

    dbg!(&c_counters);
    
    let max = dbg!{c_counters.values().max().unwrap()};
    let min = dbg!{c_counters.values().min().unwrap()};
    max - min

}