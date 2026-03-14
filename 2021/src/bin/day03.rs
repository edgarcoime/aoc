use aoc::read_input;

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

fn convert_bits_to_num(bits: &[u8]) -> i64 {
    let value = bits.iter().fold(0u32, |acc, &b| (acc << 1) | b as u32);
    value as i64
}

fn part1(input: &str) -> i64 {
    // Feels hacky
    let width = input.lines().next().unwrap().len();

    let res = input.lines().fold(
        Acc::new(width),
        |mut acc, line| {
            acc.total += 1;
            for (i, c) in line.chars().enumerate() {
                if c == '1' {
                    acc.bits[i] += 1
                }
            }
            acc
        }

    );

    // Create final bit map
    let final_bits = res
        .bits
        .iter()
        .map(|count| if count * 2 <= res.total { 0 } else { 1 })
        .collect::<Vec<u8>>();

    // Calculate
    let gamma = convert_bits_to_num(&final_bits);
    let epsilon = convert_bits_to_num(&final_bits.iter().map(|b| b ^ 1).collect::<Vec<u8>>());

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
