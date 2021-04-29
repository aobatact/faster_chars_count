#![feature(test)]
#![feature(log_syntax)]

extern crate test;
use criterion::{
    criterion_group, criterion_main, measurement::Measurement, AxisScale, BenchmarkGroup,
    BenchmarkId, Criterion, PlotConfiguration,
};
use faster_chars_count::*;
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
        //group.bench_with_input(BenchmarkId::new("usize",&test_str.0),&test_str.1, |b,i| b.iter(|| chars_count_usize(i)));
        group.bench_with_input(BenchmarkId::new("u8", &test_str.0), &test_str.1, |b, i| {
            b.iter(|| chars_count_u8(i))
        });
        group.bench_with_input(BenchmarkId::new("avx", &test_str.0), &test_str.1, |b, i| {
            b.iter(|| chars_count_256(i))
        });
        group.bench_with_input(BenchmarkId::new("u32", &test_str.0), &test_str.1, |b, i| {
            b.iter(|| chars_count_u32(i))
        });
        group.bench_with_input(BenchmarkId::new("u64", &test_str.0), &test_str.1, |b, i| {
            b.iter(|| chars_count_u64(i))
        });
        group.bench_with_input(
            BenchmarkId::new("mix1", &test_str.0),
            &test_str.1,
            |b, i| b.iter(|| chars_count_mix1(i)),
        );
        //mix 2 is slow so skip it.
        // group.bench_with_input(BenchmarkId::new("mix2", &test_str.0), &test_str.1, |b, i| {
        // b.iter(|| chars_count_mix2(i))
        // });
        // group.bench_with_input(BenchmarkId::new("mix2_suf", &test_str.0), &test_str.1, |b, i| {
        // b.iter(|| chars_count_mix2_suf(i))
        // });
        group.bench_with_input(
            BenchmarkId::new("mix3", &test_str.0),
            &test_str.1,
            |b, i| b.iter(|| chars_count_mix3(i)),
        );
        group.bench_with_input(
            BenchmarkId::new("mix3t", &test_str.0),
            &test_str.1,
            |b, i| b.iter(|| chars_count_mix3t(i)),
        );
        group.bench_with_input(
            BenchmarkId::new("mix3i", &test_str.0),
            &test_str.1,
            |b, i| b.iter(|| chars_count_mix3(i)),
        );
        group.bench_with_input(
            BenchmarkId::new("mix3ti", &test_str.0),
            &test_str.1,
            |b, i| b.iter(|| chars_count_mix3t(i)),
        );
        group.bench_with_input(BenchmarkId::new("std", &test_str.0), &test_str.1, |b, i| {
            b.iter(|| i.chars().count())
        });
    }
}

fn group_count_bench_show<'a, M: Measurement>(
    mut group: BenchmarkGroup<'a, M>,
    test_strs: impl IntoIterator<Item = &'a (usize, &'a str)>,
) {
    for test_str in test_strs {
        group.bench_with_input(
            BenchmarkId::new("faster_chars_count", &test_str.0),
            &test_str.1,
            |b, i| b.iter(|| chars_count_u64(i)),
        );
        group.bench_with_input(
            BenchmarkId::new("faster_chars_count(avx2)", &test_str.0),
            &test_str.1,
            |b, i| b.iter(|| chars_count_mix1(i)),
        );
        group.bench_with_input(BenchmarkId::new("std", &test_str.0), &test_str.1, |b, i| {
            b.iter(|| i.chars().count())
        });
    }
}

fn group_count_bench_avx<'a, M: Measurement>(
    mut group: BenchmarkGroup<'a, M>,
    test_strs: impl IntoIterator<Item = &'a (usize, &'a str)>,
) {
    for test_str in test_strs {
        group.bench_with_input(BenchmarkId::new("avx", &test_str.0), &test_str.1, |b, i| {
            b.iter(|| chars_count_256(i))
        });
        group.bench_with_input(
            BenchmarkId::new("avx_iter", &test_str.0),
            &test_str.1,
            |b, i| b.iter(|| chars_count_256_iter(i)),
        );
    }
}

fn group_count_bench_mix1<'a, M: Measurement>(
    mut group: BenchmarkGroup<'a, M>,
    test_strs: impl IntoIterator<Item = &'a (usize, &'a str)>,
) {
    for test_str in test_strs {
        //group.bench_with_input(BenchmarkId::new("usize",&test_str.0),&test_str.1, |b,i| b.iter(|| chars_count_usize(i)));
        group.bench_with_input(
            BenchmarkId::new("mix1", &test_str.0),
            &test_str.1,
            |b, i| b.iter(|| chars_count_mix1(i)),
        );
        group.bench_with_input(
            BenchmarkId::new("mix1a", &test_str.0),
            &test_str.1,
            |b, i| b.iter(|| chars_count_mix1a(i)),
        );
        group.bench_with_input(
            BenchmarkId::new("mix1b", &test_str.0),
            &test_str.1,
            |b, i| b.iter(|| chars_count_mix1b(i)),
        );
        group.bench_with_input(BenchmarkId::new("std", &test_str.0), &test_str.1, |b, i| {
            b.iter(|| i.chars().count())
        });
    }
}

fn count_bench_1_small_mix1(c: &mut Criterion) {
    let mut test_strs_a = vec![];
    let a = "a";
    let a64 = a.repeat(64);
    test_strs_a.push((0, ""));
    test_strs_a.push((1, a));
    test_strs_a.push((64, &a64));
    for i in [2, 3, 4, 7, 8, 12, 15, 16, 17, 24, 31, 32, 33, 48, 49, 63].iter() {
        unsafe {
            test_strs_a.push((*i, a64.get_unchecked(..a.len() * i)));
        }
    }
    let group = c.benchmark_group("count_bench_1byte_small_mix1");
    group_count_bench_mix1(group, test_strs_a.iter());
}

fn count_bench_1_small(c: &mut Criterion) {
    let mut test_strs_a = vec![];
    let a = "a";
    let a64 = a.repeat(64);
    test_strs_a.push((0, ""));
    test_strs_a.push((1, a));
    test_strs_a.push((64, &a64));
    for i in [2, 4, 8, 12, 16, 24, 32, 48, 63].iter() {
        unsafe {
            test_strs_a.push((*i, a64.get_unchecked(..a.len() * i)));
        }
    }
    let group = c.benchmark_group("count_bench_1byte_small");
    group_count_bench(group, test_strs_a.iter());
}

fn count_bench_1_s1_small(c: &mut Criterion) {
    let mut test_strs_a = vec![];
    let a = "a";
    let a64 = a.repeat(67);
    for i in (2..67).into_iter()
    /*
    for i in [
        2, 3, 5, 7, 9, 13, 16, 17, 19, 25, 32, 33, 35, 36, 49, 64, 65, 67,
    ]
    .iter()
    */
    {
        unsafe {
            test_strs_a.push((i - 1, a64.get_unchecked(1..a.len() * i)));
        }
    }
    let group = c.benchmark_group("count_bench_1byte_s1_small");
    group_count_bench(group, test_strs_a.iter());
}

fn count_bench_1_large(c: &mut Criterion) {
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
    let mut group = c.benchmark_group("count_bench_1byte_detailed");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    group_count_bench(group, test_strs_a.iter());
}

fn count_bench_1_large_avx_iter(c: &mut Criterion) {
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
    let mut group = c.benchmark_group("count_bench_1byte_avx_iter");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    group_count_bench_avx(group, test_strs_a.iter());
}

fn count_bench_1_large_show(c: &mut Criterion) {
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
    group_count_bench_show(group, test_strs_a.iter());
}

fn count_bench_3_large(c: &mut Criterion) {
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
    let mut group = c.benchmark_group("count_bench_3byte_detailed");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    group_count_bench(group, test_strs_s.iter());
}

fn count_bench_3_large_show(c: &mut Criterion) {
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
    group_count_bench_show(group, test_strs_s.iter());
}

criterion_group!(benches_large, count_bench_1_large, count_bench_3_large);
criterion_group!(benches_avx, count_bench_1_large_avx_iter);
criterion_group!(
    benches_show,
    count_bench_1_large_show,
    count_bench_3_large_show
);
criterion_group!(benches_small, count_bench_1_small, count_bench_1_s1_small);
criterion_group!(benches_small_mix1, count_bench_1_small_mix1);
criterion_main!(benches_show);
