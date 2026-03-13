use aoc::read_input;

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.trim().parse::<i32>().expect("input is not a number"))
        .collect()
}

fn part1(input: &str) -> i32 {
    let nums = parse(&input);
    nums
        .windows(2)
        .fold(0, |acc, win| {
            match win {
                [a, b] => {
                    if a < b {
                        acc + 1
                    } else {
                        acc
                    }
                }
                _ => panic!("unexpected window size")
            }
        })
}

fn part2(input: &str) -> i32 {
    0
}

fn main() {
    let input = read_input("inputs/day01_p1.txt");

    let p1 = part1(&input);

    println!("part 1: {}", p1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_input;

    #[test]
    fn test_part1_sample() {
        let input = read_input("examples/day01_p1.txt");
        assert_eq!(part1(&input), 7)
    }
}
