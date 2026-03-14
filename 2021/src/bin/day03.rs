use aoc::{bits_to_num, read_input};

#[derive(Debug, Clone)]
struct Acc {
    total: i32,
    bits: Vec<i32>,
    width: usize,
}

impl Acc {
    fn new(size: usize) -> Self {
        Self {
            total: 0,
            bits: vec![0; size],
            width: size,
        }
    }
}

fn get_one_counts(input: &str) -> Acc {
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let width = first.len();

    std::iter::once(first)
        .chain(lines)
        .fold(Acc::new(width), |mut acc, line| {
            acc.total += 1;
            for (i, b) in line.bytes().enumerate() {
                if b == b'1' {
                    acc.bits[i] += 1;
                }
            }
            acc
        })
}

fn part1(input: &str) -> i32 {
    let res = get_one_counts(&input);

    let gamma = bits_to_num(
        res.bits
            .iter()
            .map(|&count| if count * 2 > res.total { 1 } else { 0 }),
    );

    let epsilon = bits_to_num(
        res.bits
            .iter()
            .map(|&count| if count * 2 > res.total { 0 } else { 1 }),
    );

    println!("Gamma: {}, Epsilon: {}", gamma, epsilon);
    gamma * epsilon
}

fn reduce_candidates<'a, F>(candidates: Vec<&'a str>, idx: usize, pred: F) -> Vec<&'a str>
where
    F: Fn(char) -> bool,
{
    candidates
        .into_iter()
        .filter(|entry| {
            let c = entry.as_bytes()[idx] as char;
            pred(c)
        })
        .collect()
}

fn find_candidate<'a, F>(mut candidates: Vec<&'a str>, width: usize, pred: F) -> &'a str
where
    F: Fn(usize, usize, usize, char) -> bool,
{
    for idx in 0..width {
        if candidates.len() <= 1 {
            break;
        }

        let total = candidates.len();

        // count ones in this column
        let ones = candidates
            .iter()
            .filter(|entry| entry.chars().nth(idx).unwrap() == '1')
            .count();

        candidates = reduce_candidates(candidates, idx, |c| pred(ones, total, idx, c));
    }

    candidates[0]
}

fn part2(input: &str) -> i32 {
    let candidates: Vec<&str> = input.lines().collect();
    let width = candidates[0].len();

    let oxygen = find_candidate(candidates.clone(), width, |ones, total, _, c| {
        let keep_one = ones * 2 >= total;
        if keep_one { c == '1' } else { c == '0' }
    });

    let co2 = find_candidate(candidates, width, |ones, total, _, c| {
        let keep_one = ones * 2 >= total;
        if keep_one { c == '0' } else { c == '1' }
    });

    let o = i32::from_str_radix(oxygen, 2).unwrap();
    let c = i32::from_str_radix(co2, 2).unwrap();

    println!("Oxygen: {}, CO2: {}", o, c);
    o * c
}

fn main() {
    let input1 = read_input("inputs/day03.txt");
    let input2 = read_input("inputs/day03.txt");

    let p1 = part1(&input1);
    println!("{p1}");

    let p2 = part2(&input2);
    println!("{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = read_input("examples/day03.txt");
        assert_eq!(part1(&input), 198);
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input("examples/day03.txt");
        assert_eq!(part2(&input), 230);
    }
}
