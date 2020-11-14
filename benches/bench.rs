#![feature(test)]
#![feature(log_syntax)]

extern crate test;
use criterion::{
    criterion_group, criterion_main, measurement::Measurement, AxisScale, BenchmarkGroup,
    BenchmarkId, Criterion, PlotConfiguration,
};
use faster_char_count::*;
//use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
use std::mem::forget;
use std::ptr;

pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = ptr::read_volatile(&dummy);
        forget(dummy);
        ret
    }
}

fn group_count_bench<'a, M: Measurement>(
    mut group: BenchmarkGroup<'a, M>,
    test_strs: impl IntoIterator<Item = &'a (usize, &'a str)>,
) {
    for test_str in test_strs {
        group.bench_with_input(BenchmarkId::new("avx", &test_str.0), &test_str.1, |b, i| {
            b.iter(|| chars_count_256(i))
        });
        //group.bench_with_input(BenchmarkId::new("usize",&test_str.0),&test_str.1, |b,i| b.iter(|| chars_count_usize(i)));
        group.bench_with_input(BenchmarkId::new("u8", &test_str.0), &test_str.1, |b, i| {
            b.iter(|| chars_count_u8(i))
        });
        group.bench_with_input(BenchmarkId::new("u32", &test_str.0), &test_str.1, |b, i| {
            b.iter(|| chars_count_u32(i))
        });
        group.bench_with_input(BenchmarkId::new("u64", &test_str.0), &test_str.1, |b, i| {
            b.iter(|| chars_count_u64(i))
        });
        group.bench_with_input(BenchmarkId::new("mix", &test_str.0), &test_str.1, |b, i| {
            b.iter(|| i.chars_count())
        });
        group.bench_with_input(BenchmarkId::new("std", &test_str.0), &test_str.1, |b, i| {
            b.iter(|| i.chars().count())
        });
    }
}

fn count_bench_1_small(c: &mut Criterion) {
    let mut test_strs_a = vec![];
    let a = "a";
    let a64 = a.repeat(64);
    test_strs_a.push((0, ""));
    test_strs_a.push((1, a));
    test_strs_a.push((64, &a64));
    for i in [2, 4, 8, 16, 32, 48].iter() {
        unsafe {
            test_strs_a.push((*i, a64.get_unchecked(..a.len() * i)));
        }
    }
    let mut group = c.benchmark_group("count_bench_1byte_small");
    group_count_bench(group, test_strs_a.iter());
}

fn count_bench_1(c: &mut Criterion) {
    let mut test_strs_a = vec![];
    let a = "a";
    let a10000 = a.repeat(10000);
    test_strs_a.push((1, a));
    test_strs_a.push((10000, &a10000));
    for i in [10, 100, 1000].iter() {
        unsafe {
            test_strs_a.push((*i, a10000.get_unchecked(..a.len() * i)));
        }
    }
    let mut group = c.benchmark_group("count_bench_1byte");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    group_count_bench(group, test_strs_a.iter());
}

fn count_bench_3(c: &mut Criterion) {
    let mut test_strs_s = vec![];
    let s = "錆";
    let s10000 = s.repeat(10000);
    test_strs_s.push((1, s));
    test_strs_s.push((10000, &s10000));
    for i in [10, 100, 1000].iter() {
        unsafe {
            test_strs_s.push((*i, s10000.get_unchecked(..s.len() * i)));
        }
    }
    let mut group = c.benchmark_group("count_bench_3byte");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    group_count_bench(group, test_strs_s.iter());
}

criterion_group!(benches, count_bench_1, count_bench_3);
criterion_group!(benches_small, count_bench_1_small);
criterion_main!(benches_small);
