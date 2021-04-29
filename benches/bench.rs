#![feature(test)]
#![feature(log_syntax)]
use criterion::{
    criterion_group, criterion_main, measurement::Measurement, AxisScale, BenchmarkGroup,
    BenchmarkId, Criterion, PlotConfiguration,
};
use faster_chars_count::*;
//use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};

/// benchmark for all (except mix2*) functions to compare.
fn group_count_bench_base<'a, 'b, M: Measurement>(
    mut group: BenchmarkGroup<'a, M>,
    test_funcs: Vec<(&'b str, fn(&str) -> usize)>,
    test_str_base: &'static str,
    test_str_sizes: Vec<usize>,
    // to make it unaligned
    test_str_offset: usize,
) {
    let max = test_str_sizes.iter().max().unwrap();
    let test_str_m = test_str_base.repeat(
        *max + if test_str_base.len() == 0 {
            0
        } else {
            test_str_offset / test_str_base.len() + 1
        },
    );
    for test_size in test_str_sizes {
        let test_str = &test_str_m[test_str_offset * test_str_base.len()
            ..(test_size + test_str_offset) * test_str_base.len()];
        for test_func in &test_funcs {
            group.bench_with_input(
                BenchmarkId::new(test_func.0, test_str.len()),
                test_str,
                |b, i| b.iter(|| criterion::black_box((test_func.1)(i))),
            );
        }
    }
}

fn opt_bench_list() -> Vec<(&'static str, fn(&str) -> usize)> {
    vec![
        ("faster_chars_count(mix1)", chars_count_mix1),
        ("faster_chars_count(usize)", chars_count_usize),
        ("faster_chars_count(avx2)", chars_count_256),
    ]
}

fn show_bench_list() -> Vec<(&'static str, fn(&str) -> usize)> {
    vec![
        ("std", |s| s.chars().count()),
        ("faster_chars_count(mix1)", chars_count_mix1),
        ("faster_chars_count(usize)", chars_count_usize),
        ("faster_chars_count(avx2)", chars_count_256),
    ]
}

fn all_bench_list() -> Vec<(&'static str, fn(&str) -> usize)> {
    vec![
        ("std", |s| s.chars().count()),
        ("u8", chars_count_u8),
        ("u32", chars_count_u32),
        ("u64", chars_count_u64),
        ("usize", chars_count_usize),
        ("u128", chars_count_u128),
        ("avx2", chars_count_256),
        ("mix1", chars_count_mix1),
        ("mix3", chars_count_mix3),
    ]
}

fn mix1_bench_list() -> Vec<(&'static str, fn(&str) -> usize)> {
    vec![
        ("avx", chars_count_256),
        ("mix1", chars_count_mix1),
        ("mix1a", chars_count_mix1a),
        ("mix1c", chars_count_mix1c),
    ]
}

fn mix1_3_bench_list() -> Vec<(&'static str, fn(&str) -> usize)> {
    vec![
        ("avx2", chars_count_256),
        ("mix1", chars_count_mix1),
        ("mix1a", chars_count_mix1a),
        ("mix1c", chars_count_mix1c),
        ("mix3", chars_count_mix3),
        ("mix3t", chars_count_mix3t),
        ("mix3ti", chars_count_mix3ti),
    ]
}

///Count of repeated 'a' for small size for mix1.
fn count_bench_1_small_mix1(c: &mut Criterion) {
    group_count_bench_base(
        c.benchmark_group("count_bench_1byte_small_mix1"),
        mix1_bench_list(),
        "a",
        (0..=65).into_iter().collect(),
        0,
    )
}

///Count of repeated 'a' for small size.
fn count_bench_1_small(c: &mut Criterion) {
    group_count_bench_base(
        c.benchmark_group("count_bench_1byte_small_all"),
        all_bench_list(),
        "a",
        (0..=65).into_iter().collect(),
        0,
    )
}

///Count of repeated 'a' for small size. May be unaligned.
fn count_bench_1_s1_small(c: &mut Criterion) {
    group_count_bench_base(
        c.benchmark_group("count_bench_1byte_small_all_o1"),
        all_bench_list(),
        "a",
        (0..=65).into_iter().collect(),
        1,
    )
}

///Count of repeated 'a' for small size. May be unaligned.
fn count_bench_1_s1_small100(c: &mut Criterion) {
    group_count_bench_base(
        c.benchmark_group("count_bench_1byte_around100_opt"),
        opt_bench_list(),
        "a",
        vec![310],
        1,
    )
}

///Count of repeated 'a' for large size.
fn count_bench_1_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_bench_1byte_all_large");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    group_count_bench_base(
        group,
        all_bench_list(),
        "a",
        vec![1, 10, 20, 50, 100, 200, 500, 1000, 2000, 5000, 10000],
        0,
    )
}

fn count_bench_1_mid_show(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_bench_1byte_mid");
    //let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    //group.plot_config(plot_config);

    group_count_bench_base(
        group,
        show_bench_list(),
        "a",
        vec![100, 200, 300, 400, 500],
        0,
    )
}

fn count_bench_1_s1_mid_show(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_bench_1byte_mid_offset1");
    //let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    //group.plot_config(plot_config);

    group_count_bench_base(
        group,
        show_bench_list(),
        "a",
        vec![100, 200, 300, 400, 500],
        1,
    )
}

fn count_bench_1_large_show(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_bench_1byte_large");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    group_count_bench_base(
        group,
        show_bench_list(),
        "a",
        vec![1, 10, 20, 50, 100, 200, 500, 1000, 2000, 5000, 10000],
        0,
    )
}

///Count of repeated 'a' for small size.
fn count_bench_3_small(c: &mut Criterion) {
    group_count_bench_base(
        c.benchmark_group("count_bench_3byte_small_all"),
        all_bench_list(),
        "錆",
        (0..=65).into_iter().collect(),
        0,
    )
}

///Count of repeated 'a' for small size. May be unaligned.
fn count_bench_3_s1_small(c: &mut Criterion) {
    group_count_bench_base(
        c.benchmark_group("count_bench_3byte_small_all"),
        all_bench_list(),
        "錆",
        (0..=65).into_iter().collect(),
        1,
    )
}

///Count of repeated 'a' for large size.
fn count_bench_3_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_bench_3byte_all_large");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    group_count_bench_base(
        group,
        all_bench_list(),
        "錆",
        vec![1, 10, 20, 50, 100, 200, 500, 1000, 2000, 5000, 10000],
        0,
    )
}

fn count_bench_3_large_show(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_bench_3byte_large");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    group_count_bench_base(
        group,
        show_bench_list(),
        "錆",
        vec![1, 10, 20, 50, 100, 200, 500, 1000, 2000, 5000, 10000],
        1,
    )
}

criterion_group!(benches_large, count_bench_1_large, count_bench_3_large);
criterion_group!(
    benches_show,
    count_bench_1_large_show,
    count_bench_3_large_show,
    count_bench_1_mid_show,
    count_bench_1_s1_mid_show,
);
criterion_group!(
    benches_small_1,
    count_bench_1_small,
    count_bench_1_s1_small,
    count_bench_1_s1_small100
);
criterion_group!(benches_small_3, count_bench_3_small, count_bench_3_s1_small);
criterion_group!(benches_small_mix1, count_bench_1_small_mix1);
criterion_main!(
    benches_large,
    benches_small_1,
    benches_small_3,
    benches_small_mix1,
    benches_show
);
