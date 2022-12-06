fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();
    let depths: Vec<_> = lines.map(|l| l.parse::<u32>().unwrap()).collect();

    let mut triplets = depths[..depths.len()-2].iter().zip(depths[1..depths.len()-1].iter().zip(depths[2..].iter())).map(|(a,(b,c))| *a + *b + *c);
    let mut prev = triplets.next().unwrap();
    let mut increases = 0;
    for next in triplets {
        if next > prev {
            increases += 1;
        }
        prev = next;
    }
    println!("{}", increases);
}
