#![feature(stdarch)]
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub trait CharsCount {
    fn chars_count(&self) -> usize;
}

impl CharsCount for str {
    fn chars_count(&self) -> usize {
        chars_count(&self)
    }
}

pub fn chars_count(s: &str) -> usize {
    let slice: &[u8] = s.as_ref();
    let (pre, mid_count, suf) = match slice.len() {
        35..=usize::MAX if cfg!(target_arch = "x86_64") && is_x86_feature_detected!("avx2") => unsafe {
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
    //if slice.len() == 0 { return 0 }
    let mut count = 0;
    for c in slice {
        // (u8) 0x00 <= c <= 0x 7f or (i8) 0xc0 <= c <= 0xff
        //let ci = *c as i8;
        //if ci > -0x41 { //no diff in bench with below
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

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn count_256(slice: &[__m256i]) -> usize {
    //if slice.len() == 0 { return 0 }
    let mut count = 0 as usize;
    let chunks = slice.chunks(255);
    for block in chunks {
        let mut sum = _mm256_setzero_si256();
        for s in block {
            sum = _mm256_sub_epi8(
                sum,
                _mm256_cmpgt_epi8(_mm256_load_si256(s), _mm256_set1_epi8(-0x41)),
            );
        }
        count += avx2_horizontal_sum_epi8(sum);
    }
    count
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn avx2_horizontal_sum_epi8(x: __m256i) -> usize {
    let sumhi = _mm256_unpackhi_epi8(x, _mm256_setzero_si256());
    let sumlo = _mm256_unpacklo_epi8(x, _mm256_setzero_si256());
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
        count_test_base(chars_count);
    }
}
