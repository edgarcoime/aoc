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
    let nums = parse(&input);
    nums
        .windows(3)
        .map(|win| win
            .iter()
            .sum()
        )
        .collect::<Vec<i32>>()
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

fn part2_win4(input: &str) -> i32 {
    let nums = parse(&input);
    nums
        .windows(4)
        .fold(0, |acc, win| {
            match win {
                [a, _, _, d] => {
                    if a < d {
                        acc + 1
                    } else {
                        acc
                    }
                }
                _ => panic!("unexpected window size")
            }
        })
}


fn main() {
    let input1 = read_input("inputs/day01_p1.txt");
    let input2 = read_input("inputs/day01_p1.txt");

    let p1 = part1(&input1);
    let p2 = part2(&input2);
    let p2v2 = part2_win4(&input2);

    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
    println!("part 2 ver.2: {}", p2v2);
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

    #[test]
    fn test_part2_sample() {
        let input = read_input("examples/day01_p1.txt");
        assert_eq!(part2(&input), 5)
    }
}
