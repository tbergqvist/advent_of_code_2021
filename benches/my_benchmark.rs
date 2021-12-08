use std::fs;

use criterion::{criterion_group, criterion_main, Criterion};
use advent_of_code_2021::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("./inputs/1.txt").unwrap();
    c.bench_function("day1::a", |b| b.iter(|| day_1::a(&input)));
    c.bench_function("day1::b", |b| b.iter(|| day_1::b(&input)));

    let input = fs::read_to_string("./inputs/2.txt").unwrap();
    c.bench_function("day2::a", |b| b.iter(|| day_2::a(&input)));
    c.bench_function("day2::b", |b| b.iter(|| day_2::b(&input)));

    let input = fs::read_to_string("./inputs/3.txt").unwrap();
    c.bench_function("day3::a", |b| b.iter(|| day_3::a(&input)));
    c.bench_function("day3::b", |b| b.iter(|| day_3::b(&input)));

    let input = fs::read_to_string("./inputs/4.txt").unwrap();
    c.bench_function("day4::a", |b| b.iter(|| day_4::a(&input)));
    c.bench_function("day4::b", |b| b.iter(|| day_4::b(&input)));

    let input = fs::read_to_string("./inputs/5.txt").unwrap();
    c.bench_function("day5::a", |b| b.iter(|| day_5::a(&input)));
    c.bench_function("day5::b", |b| b.iter(|| day_5::b(&input)));

    let input = fs::read_to_string("./inputs/6.txt").unwrap();
    c.bench_function("day6::a", |b| b.iter(|| day_6::a(&input)));
    c.bench_function("day6::b", |b| b.iter(|| day_6::b(&input)));

    let input = fs::read_to_string("./inputs/7.txt").unwrap();
    c.bench_function("day7::a", |b| b.iter(|| day_7::a(&input)));
    c.bench_function("day7::b", |b| b.iter(|| day_7::b(&input)));

    let input = fs::read_to_string("./inputs/8.txt").unwrap();
    c.bench_function("day8::a", |b| b.iter(|| day_8::a(&input)));
    c.bench_function("day8::b", |b| b.iter(|| day_8::b(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);