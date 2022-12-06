fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}

fn part1(input: &str) -> u64 {
    input.lines()
        .map(|l| line_score(l))
        .filter(|s| s.is_err())
        .map(|e| e.err().unwrap())
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut autocompletes: Vec<u64> = input.lines()
        .map(|l| line_score(l))
        .filter(|s| s.is_ok())
        .map(|e| e.unwrap())
        .collect();
    autocompletes.sort();
    autocompletes[autocompletes.len() / 2]
}

fn line_score(l: &str) -> Result<u64, u64> {
    let mut stack = vec![];
    for c in l.chars() {
        match c {
            '[' | '(' | '{' | '<' => stack.push(c),
            ')' => match stack.pop() { Some('(') => (), _=> return Err(invalid_score(')'))},
            ']' => match stack.pop() { Some('[') => (), _=> return Err(invalid_score(']'))},
            '}' => match stack.pop() { Some('{') => (), _=> return Err(invalid_score('}'))},
            '>' => match stack.pop() { Some('<') => (), _=> return Err(invalid_score('>'))},
            _=>unreachable!()
        }
    }
    Ok(valid_score(stack))
}

fn invalid_score(closing: char) -> u64 {
    match closing {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!()
    }
}

fn valid_score(mut stack: Vec<char>) -> u64 {
    let valid_char_score = |c| match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!()
    };
    stack.reverse();
    stack.iter().fold(0, |acc, c| 5 * acc + valid_char_score(*c))
}

const INPUT: &str = include_str!("input.txt");
