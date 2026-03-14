use std::hint::black_box;

use aoc::days::day01::{part2, part2_win4};
use aoc::read_input;
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_day01_part2(c: &mut Criterion) {
    let input = read_input("inputs/day01_p1.txt");

    let mut group = c.benchmark_group("day01_part2");
    group.bench_function("windows3_sum", |b| b.iter(|| part2(black_box(&input))));
    group.bench_function("windows4_compare", |b| {
        b.iter(|| part2_win4(black_box(&input)))
    });
    group.finish();
}

criterion_group!(benches, bench_day01_part2);
criterion_main!(benches);
