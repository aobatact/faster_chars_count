//! Library for counting length of chars faster than [`chars()`](`str::chars`).[`count()`](`std::str::Chars::count()`)
//!
//! ```
//! use faster_chars_count::*;
//! let a = "rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;";
//! assert_eq!(a.chars().count(), chars_count_str(a));
//! assert_eq!(a.chars().count(), a.chars_count());
//! ```
//!
//! Idea is that we only needs to count the byte witch is not a continuation byte. This can be done at the same time for 4byte ([`u64`]) or 32byte ([`__m256i`](`core::arch::x86_64::__m256i`) with avx2).

#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

/// Trait for counting chars faster than [`chars()`](`str::chars`).[`count()`](`std::str::Chars::count()`)
pub trait CharsCount {
    /// Counting chars faster than [`chars()`](`str::chars`).[`count()`](`std::str::Chars::count()`)
    fn chars_count(&self) -> usize;
}

impl CharsCount for str {
    fn chars_count(&self) -> usize {
        chars_count_str(&self)
    }
}

#[inline]
/// Function version of faster `chars_count()`
pub fn chars_count_str(s: &str) -> usize {
    chars_count_byte(s.as_ref())
}

#[cfg(feature = "runtime_detect")]
fn use_avx2() -> bool {
    cfg!(target_arch = "x86_64") && is_x86_feature_detected!("avx2")
}
#[cfg(not(feature = "runtime_detect"))]
const fn use_avx2() -> bool {
    cfg!(target_arch = "x86_64") && cfg!(target_feature = "avx2")
}

/// Function version of faster `chars_count()` for `&[u8]`
pub fn chars_count_byte(slice: &[u8]) -> usize {
    let (pre, mid_count, suf) = match slice.len() {
        //320 is from benchmark of unaligned byte slice.
        320..=usize::MAX if use_avx2() => unsafe {
            let (pre, mid, suf) = slice.align_to::<__m256i>();
            (pre, count_256(mid), suf)
        },
        11..=usize::MAX => unsafe {
            let (pre, mid, suf) = slice.align_to::<usize>();
            (pre, count_usize(mid), suf)
        },
        1 => return 1,
        0 => return 0,
        _ => return count_u8(slice),
    };
    count_u8(pre) + count_u8(suf) + mid_count
}

#[inline]
fn count_u8(slice: &[u8]) -> usize {
    let mut count = 0;
    for c in slice {
        if c & 0xC0 != 0x80 {
            count += 1;
        }
    }
    count
}

#[inline]
fn count_usize(slice: &[usize]) -> usize {
    let mut count = 0;
    for c in slice {
        let f = c | (!c >> 1);
        let n = f & 0x_4040_4040_4040_4040_usize;
        count += n.count_ones() as usize;
    }
    count
}

const ZERO: __m256i = unsafe { core::mem::transmute([0_u8; 32]) };

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn count_256(slice: &[__m256i]) -> usize {
    let mut count = 0 as usize;
    let chunks = slice.chunks(255);
    const GT: __m256i = unsafe { core::mem::transmute([-0x41_i8; 32]) };
    for block in chunks {
        let mut sum = ZERO;
        for s in block {
            sum = _mm256_sub_epi8(sum, _mm256_cmpgt_epi8(_mm256_load_si256(s), GT));
        }
        count += avx2_horizontal_sum_epi8(sum);
    }
    count
}

#[inline]
#[allow(non_snake_case)]
const fn _MM_SHUFFLE(z: u32, y: u32, x: u32, w: u32) -> i32 {
    ((z << 6) | (y << 4) | (x << 2) | w) as i32
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn avx2_horizontal_sum_epi8(x: __m256i) -> usize {
    let sumhi = _mm256_unpackhi_epi8(x, ZERO);
    let sumlo = _mm256_unpacklo_epi8(x, ZERO);
    let sum16x16 = _mm256_add_epi16(sumhi, sumlo);
    let sum16x8 = _mm256_add_epi16(sum16x16, _mm256_permute2x128_si256(sum16x16, sum16x16, 1));
    let sum16x4 = _mm256_add_epi16(
        sum16x8,
        _mm256_shuffle_epi32(sum16x8, _MM_SHUFFLE(0, 0, 2, 3)),
    );
    let tmp = _mm256_extract_epi64(sum16x4, 0);
    let mut result = (tmp >> 0) & 0xffff;
    result += (tmp >> 16) & 0xffff;
    result += (tmp >> 32) & 0xffff;
    result += (tmp >> 48) & 0xffff;
    result as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count_test_base<F>(f: F)
    where
        F: Fn(&str) -> usize,
    {
        let a = "Hello, world!";
        assert_eq!(a.chars().count(), f(a));
        let a = "rust=錆";
        assert_eq!(a.chars().count(), f(a));
        let a = "rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆";
        assert_eq!(a.chars().count(), f(a));
        let a = "rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆
        rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;;rust=錆;rust=錆;rust=錆;;v;rust=錆;rust=錆;;v;rust=錆;rust=錆;v;rust=錆;v;v;v;rust=錆;rust=錆;rust=錆";
        assert_eq!(a.chars().count(), f(a));
        let a = "rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆
        rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;;rust=錆;rust=錆;rust=錆;;v;rust=錆;rust=錆;;v;rust=錆;rust=錆;v;rust=錆;v;v;v;rust=錆;rust=錆;rust=錆;錆、酸化鉄;錆、酸化鉄;ÁÁÁÁ;😀😁😂";
        assert_eq!(a.chars().count(), f(a));
    }

    #[test]
    fn count_mix1() {
        count_test_base(chars_count_str);
    }
}
