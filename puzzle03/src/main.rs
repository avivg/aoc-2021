type NumInBits = Vec<u8>;

fn to_bits(l: &str) -> NumInBits {
    l.as_bytes().iter().map(|b| b - '0' as u8).collect()
}

fn to_num(bits: &NumInBits) -> i64 {
    i64::from_str_radix(&bits.iter().fold(String::new(), |s, b| s + &b.to_string()), 2).unwrap()
}

fn bit_count(nums: &Vec<NumInBits>) -> Vec<usize> {
    let mut res = vec![0usize; nums[0].len()];
    for num in nums {
        for (c, b) in res.iter_mut().zip(num.iter()) {
            *c += *b as usize;
        }
    }
    res
}

fn filter(nums: Vec<NumInBits>, idx: usize, what: u8) -> Vec<NumInBits> {
    let count = bit_count(&nums);
    let threashold = nums.len() as f64 / 2f64;
    let req_bit = match (count[idx] as f64).total_cmp(&threashold) {
        std::cmp::Ordering::Less => 1 - what,
        _ => what
    };
    nums.into_iter().filter(|nb| nb[idx] == req_bit).collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();
    let o2 = {
        let mut nums: Vec<_> = lines.clone().map(to_bits).collect();
        'filterloop: for filter_idx in 0..12 {
            nums = filter(nums, filter_idx, 1);
            if nums.len() < 2 {
                break 'filterloop;
            }
        }
        assert_eq!(1, nums.len());
        to_num(nums.first().unwrap())
    };
    let co2 = {
        let mut nums: Vec<_> = lines.map(to_bits).collect();
        'filterloop: for filter_idx in 0..12 {
            nums = filter(nums, filter_idx, 0);
            if nums.len() < 2 {
                break 'filterloop;
            }
        }
        assert_eq!(1, nums.len());
        to_num(nums.first().unwrap())
    };

    println!("{} * {} = {}", o2, co2, o2 * co2);
}
