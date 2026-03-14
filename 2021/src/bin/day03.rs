use aoc::{bits_to_num, read_input};

#[derive(Debug)]
struct Acc {
    total: i32,
    bits: Vec<i32>,
}

impl Acc {
    fn new(size: usize) -> Self {
        Self {
            total: 0,
            bits: vec![0; size],
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let width = first.len();

    let res = std::iter::once(first).chain(lines).fold(
        Acc::new(width),
        |mut acc, line| {
            acc.total += 1;
            for (i, b) in line.bytes().enumerate() {
                if b == b'1' {
                    acc.bits[i] += 1;
                }
            }
            acc
        },
    );

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

fn part2(input: &str) -> i64 {
    todo!()
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
        assert_eq!(part2(&input), 900);
    }
}
