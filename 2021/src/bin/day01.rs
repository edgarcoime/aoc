use aoc::read_input;
use aoc::days::day01::{part1, part2};

fn main() {
    let input1 = read_input("inputs/day01_p1.txt");
    let input2 = read_input("inputs/day01_p1.txt");

    let p1 = part1(&input1);
    let p2 = part2(&input2);

    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}
