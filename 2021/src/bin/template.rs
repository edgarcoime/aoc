use aoc::read_input;

fn part1(input: &str) -> i64 {
    todo!()
}

fn part2(input: &str) -> i64 {
    todo!()
}

fn main() {
    let input1 = read_input("inputs/day03.txt");
    let input2 = read_input("inputs/day03.txt");

    let p1 = part1(&input1);
    let p2 = part2(&input2);

    println!("{p1}");
    println!("{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = read_input("examples/day03.txt");
        assert_eq!(part1(&input), 150);
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input("examples/day03.txt");
        assert_eq!(part2(&input), 900);
    }
}
