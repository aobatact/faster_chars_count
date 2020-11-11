#![feature(test)]
#![feature(log_syntax)]

extern crate test;
use std::ptr;
use std::mem::forget;
use faster_char_count::*;

pub fn black_box<T>(dummy: T) -> T{    unsafe {
    let ret = ptr::read_volatile(&dummy);
    forget(dummy);
    ret
}}

const TEST_STR1 : &str = "錆";
const TEST_STR2 : &str = "錆,rust;";
const TEST_STR3 : &str = "錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;";
const TEST_STR4 : &str = "錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;
錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;";
const TEST_STR5 : &str = "錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;
錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;
錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;";
const TEST_STR6 : &str = "錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;
錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;
錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;
錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;
錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;
錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;
錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;
錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;
錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;
錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;
錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;
錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;
錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;
錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;錆,rust;";


fn chars_count_my_bench(b: &mut test::Bencher,s: &str) {
    b.iter(|| black_box(s.chars_count()))
}

fn chars_count_std_bench(b: &mut test::Bencher,s: &str) {
    b.iter(|| black_box(s.chars().count()))
}

fn u32_bench(b: &mut test::Bencher,s: &str) {
    b.iter(|| black_box(chars_count_u32(s)))
}

fn avx2_bench(b: &mut test::Bencher,s: &str) {
    b.iter(|| black_box(chars_count_256(s)))
}

#[bench]
fn chars_count_std_bench_1(b: &mut test::Bencher) {
    chars_count_std_bench(b,TEST_STR1)
}

#[bench]
fn u32_bench_1(b: &mut test::Bencher) {
    u32_bench(b,TEST_STR1)
}

#[bench]
fn avx2_bench_1(b: &mut test::Bencher) {
    avx2_bench(b,TEST_STR1)
}

#[bench]
fn chars_count_std_bench_2(b: &mut test::Bencher) {
    chars_count_std_bench(b,TEST_STR2)
}

#[bench]
fn u32_bench_2(b: &mut test::Bencher) {
    u32_bench(b,TEST_STR2)
}

#[bench]
fn avx2_bench_2(b: &mut test::Bencher) {
    avx2_bench(b,TEST_STR2)
}
#[bench]
fn chars_count_std_bench_3(b: &mut test::Bencher) {
    chars_count_std_bench(b,TEST_STR3)
}

#[bench]
fn u32_bench_3(b: &mut test::Bencher) {
    u32_bench(b,TEST_STR3)
}

#[bench]
fn avx2_bench_3(b: &mut test::Bencher) {
    avx2_bench(b,TEST_STR3)
}
#[bench]
fn chars_count_std_bench_4(b: &mut test::Bencher) {
    chars_count_std_bench(b,TEST_STR4)
}

#[bench]
fn u32_bench_4(b: &mut test::Bencher) {
    u32_bench(b,TEST_STR4)
}

#[bench]
fn avx2_bench_4(b: &mut test::Bencher) {
    avx2_bench(b,TEST_STR4)
}
#[bench]
fn chars_count_std_bench_5(b: &mut test::Bencher) {
    chars_count_std_bench(b,TEST_STR5)
}

#[bench]
fn u32_bench_5(b: &mut test::Bencher) {
    u32_bench(b,TEST_STR5)
}

#[bench]
fn avx2_bench_5(b: &mut test::Bencher) {
    avx2_bench(b,TEST_STR5)
}

#[bench]
fn chars_count_std_bench_6(b: &mut test::Bencher) {
    chars_count_std_bench(b,TEST_STR6)
}

#[bench]
fn u32_bench_6(b: &mut test::Bencher) {
    u32_bench(b,TEST_STR6)
}

#[bench]
fn avx2_bench_6(b: &mut test::Bencher) {
    avx2_bench(b,TEST_STR6)
}


fn u64_bench(b: &mut test::Bencher,s: &str) {
    b.iter(|| black_box(faster_char_count::chars_count_u64(s)))
}

#[bench]
fn u64_bench_1(b: &mut test::Bencher) {
    u64_bench(b,TEST_STR1)
}
#[bench]
fn u64_bench_2(b: &mut test::Bencher) {
    u64_bench(b,TEST_STR2)
}
#[bench]
fn u64_bench_3(b: &mut test::Bencher) {
    u64_bench(b,TEST_STR3)
}
#[bench]
fn u64_bench_4(b: &mut test::Bencher) {
    u64_bench(b,TEST_STR4)
}
#[bench]
fn u64_bench_5(b: &mut test::Bencher) {
    u64_bench(b,TEST_STR5)
}
#[bench]
fn u64_bench_6(b: &mut test::Bencher) {
    u64_bench(b,TEST_STR6)
}


fn usize_bench(b: &mut test::Bencher,s: &str) {
    b.iter(|| black_box(faster_char_count::chars_count_usize(s)))
}

#[bench]
fn usize_bench_1(b: &mut test::Bencher) {
    usize_bench(b,TEST_STR1)
}
#[bench]
fn usize_bench_2(b: &mut test::Bencher) {
    usize_bench(b,TEST_STR2)
}
#[bench]
fn usize_bench_3(b: &mut test::Bencher) {
    usize_bench(b,TEST_STR3)
}
#[bench]
fn usize_bench_4(b: &mut test::Bencher) {
    usize_bench(b,TEST_STR4)
}
#[bench]
fn usize_bench_5(b: &mut test::Bencher) {
    usize_bench(b,TEST_STR5)
}
#[bench]
fn usize_bench_6(b: &mut test::Bencher) {
    usize_bench(b,TEST_STR6)
}


fn u8_bench(b: &mut test::Bencher,s: &str) {
    b.iter(|| black_box(faster_char_count::chars_count_u8(s)))
}

#[bench]
fn u8_bench_1(b: &mut test::Bencher) {
    u8_bench(b,TEST_STR1)
}
#[bench]
fn u8_bench_2(b: &mut test::Bencher) {
    u8_bench(b,TEST_STR2)
}
#[bench]
fn u8_bench_3(b: &mut test::Bencher) {
    u8_bench(b,TEST_STR3)
}
#[bench]
fn u8_bench_4(b: &mut test::Bencher) {
    u8_bench(b,TEST_STR4)
}
#[bench]
fn u8_bench_5(b: &mut test::Bencher) {
    u8_bench(b,TEST_STR5)
}
#[bench]
fn u8_bench_6(b: &mut test::Bencher) {
    u8_bench(b,TEST_STR6)
}


#[bench]
fn chars_count_my_bench_1(b: &mut test::Bencher) {
    chars_count_my_bench(b,TEST_STR1)
}
#[bench]
fn chars_count_my_bench_2(b: &mut test::Bencher) {
    chars_count_my_bench(b,TEST_STR2)
}
#[bench]
fn chars_count_my_bench_3(b: &mut test::Bencher) {
    chars_count_my_bench(b,TEST_STR3)
}
#[bench]
fn chars_count_my_bench_4(b: &mut test::Bencher) {
    chars_count_my_bench(b,TEST_STR4)
}
#[bench]
fn chars_count_my_bench_5(b: &mut test::Bencher) {
    chars_count_my_bench(b,TEST_STR5)
}
#[bench]
fn chars_count_my_bench_6(b: &mut test::Bencher) {
    chars_count_my_bench(b,TEST_STR6)
}





/*
#[bench]
fn chars_count_std_bench(b: &mut test::Bencher) {
    b.iter(|| black_box(TEST_STR1.chars().count()))
}

#[bench]
fn u32_bench(b: &mut test::Bencher) {
    b.iter(|| black_box(faster_char_count::chars_count_u32(TEST_STR1)))
}

#[bench]
fn avx2_bench(b: &mut test::Bencher) {
    b.iter(|| black_box(faster_char_count::chars_count_256(TEST_STR1)))
}
*/
