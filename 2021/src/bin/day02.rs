use aoc::read_input;

enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl Command {
    fn parse(line: &str) -> Self {
        let mut parts = line.split_whitespace();

        let a = parts.next().expect("No command part").trim();
        let b = parts
            .next()
            .expect("No value part")
            .parse::<i64>()
            .expect("Not a vaid value");

        match a {
            "forward" => Command::Forward(b),
            "down" => Command::Down(b),
            "up" => Command::Up(b),
            _ => panic!("No command found"),
        }
    }
}

fn part1(input: &str) -> i64 {
    let res = input
        .lines()
        .map(|line| Command::parse(line))
        .fold((0, 0), |(dist, depth), cmd| match cmd {
            Command::Forward(x) => (dist + x, depth),
            Command::Down(x) => (dist, depth + x),
            Command::Up(x) => (dist, depth - x),
        });
    res.0 * res.1
}

fn part2(input: &str) -> i64 {
    let res = input
        .lines()
        .map(|line| Command::parse(line))
        .fold((0, 0, 0), |(dist, depth, aim), cmd| match cmd {
            Command::Forward(x) => (dist + x, depth + (aim*x), aim),
            Command::Down(x) =>    (dist, depth, aim + x),
            Command::Up(x) =>      (dist, depth, aim - x),
        });
    dbg!("{}", res);
    res.0 * res.1
}

fn main() {
    let input1 = read_input("inputs/day02_p1.txt");
    let input2 = read_input("inputs/day02_p1.txt");

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
        let input = read_input("examples/day02_p1.txt");
        assert_eq!(part1(&input), 150);
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input("examples/day02_p1.txt");
        assert_eq!(part2(&input), 900);
    }
}
