#![feature(stdarch)]

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use core::mem;

pub trait CharsCount {
    fn chars_count(&self) -> usize;
}

impl CharsCount for str {
    fn chars_count(&self) -> usize {
        chars_count_mix1(&self)
    }
}

pub fn chars_count_mix1(s: &str) -> usize {
    let slice : &[u8] = s.as_ref();
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


pub fn chars_count_mix2(s: &str) -> usize {
    fn align_part(slice : &[u8]) -> (&[u8],usize,&[u8]) {
        let len = slice.len();
        match len {
            32..=usize::MAX if cfg!(target_arch = "x86_64") && is_x86_feature_detected!("avx2") => unsafe {
                let (pre, mid, suf) = slice.align_to::<__m256i>();
                (pre, count_256(mid), suf)
            },
            8..=usize::MAX => unsafe {
                let (pre, mid, suf) = slice.align_to::<u64>();
                (pre, count_64(mid), suf)
            },
            4..=7 => unsafe {
                let (pre, mid, suf) = slice.align_to::<u32>();
                (pre, count_32(mid), suf)
            },
            0 | 1 => (<&[u8]>::default(),len,<&[u8]>::default()),
            _ => (<&[u8]>::default(),count_u8(slice),<&[u8]>::default()),
        }
    }

    let slice : &[u8] = s.as_ref();
    let (mut pre, mut count,mut suf) = align_part(slice);
    while !pre.is_empty() {
        let (pre_2,m_2,suf_2) = align_part(pre);
        debug_assert!(suf_2.is_empty());
        pre = pre_2;
        count += m_2;
    }
    while !suf.is_empty() {
        let (pre_2,m_2,suf_2) = align_part(suf);
        debug_assert!(pre_2.is_empty());
        suf = suf_2;
        count += m_2;
    }
    count
}

pub fn chars_count_mix3(s: &str) -> usize {
    #[inline]
    unsafe fn align<T>( pre_len : &mut usize, right_len :&mut  usize, right_ptr : &mut *const u8) -> bool{
        let _offset = right_ptr.align_offset(mem::align_of::<T>());
        if _offset < *right_len {
            *right_len -= _offset;
            *pre_len = _offset;
            *right_ptr = right_ptr.add(_offset);
            true
        } else {
            false
        }
    }

    let mut right_len = s.len();
    if(right_len < 2) {return right_len; }
    let mut count = 0;
    let mut pre_len = 0;
    let mut right_ptr = s.as_ptr();// right means middle and suffix here
    let mut used = 0;//for debug (and should be gone in release)
    unsafe{
        if cfg!(target_arch = "x86_64") && is_x86_feature_detected!("avx2") {
            //dummy loop for break
            loop{
                if right_len >= 32 {
                    if !align::<__m256i>(&mut pre_len,&mut right_len,&mut right_ptr) { break; }
                    //let mid_256_len = mid_len / size_256;
                    let mid_256_len = right_len >> 5;
                    if mid_256_len > 0 {
                        count = count_256(core::slice::from_raw_parts(right_ptr as *const __m256i,mid_256_len));
                        //let mid_len = mid_256_len * size_256;
                        let mid_len = mid_256_len << 5;
                        used += mid_len;
                        right_ptr = right_ptr.add(mid_len);//suf_ptr
                        right_len -= mid_len;
                    }
                }
                break;
            }
            // for sse 128?
        }

        //dummy loop for break
        loop{
            if right_len >= 8 {
                // pre_len > 0 means that right_ptr is aligned to __m256i so estimate as it is aligned to u64
                if pre_len == 0 {
                    if !align::<u64>(&mut pre_len,&mut right_len,&mut right_ptr) { break; }
                } else {
                    debug_assert_eq!(right_ptr.align_offset(mem::align_of::<u64>()),0);
                }
                //const size_64 : usize = 8;//mem::size_of::<u64>();
                let mid_64_len = right_len >> 3;
                if mid_64_len > 0 {
                    count += count_64(core::slice::from_raw_parts(right_ptr as *const u64, mid_64_len));
                    let mid_len = mid_64_len << 3;
                    right_len -= mid_len;
                    used += mid_len;
                    right_ptr = right_ptr.add(mid_len);
                }
            }
            break;
        }

        //dummy loop for break
        loop{
            if right_len >= 4 {
                // pre_len > 0 means that right_ptr is aligned to u64 so estimate as it is aligned to u32
                if pre_len == 0 {
                    if !align::<u64>(&mut pre_len,&mut right_len,&mut right_ptr) { break; }
                } else {
                    debug_assert_eq!(right_ptr.align_offset(mem::align_of::<u32>()),0);
                }
                //const size_32 : usize = 4;//mem::size_of::<u64>();
                let mid_32_len = right_len >> 2;
                if mid_32_len > 0 {
                    // we know that mid_32_len is 1 here.
                    /*
                    count += count_32(core::slice::from_raw_parts(right_ptr as *const u32, mid_64_len));
                    let mid_len = mid_32_len << 3;
                    right_len -= mid_len;
                    used += mid_len;
                    */
                    count += {
                        let r_32 = *(right_ptr as *const u32);
                        let f = r_32 | (!r_32 >> 1);
                        let n = f & 0x_4040_4040;
                        n.count_ones() as usize
                    };
                    const mid_len :usize  = 4;
                    right_len -= mid_len;
                    used += mid_len;
                    right_ptr = right_ptr.add(mid_len);
                }
            }
            break;
        }
        let mut left_count = 0;
        let mut right_count = 0;
        if pre_len > 0{
            let slice :  &[u8]  =s.as_ref();
            left_count = count_u8(&slice[..pre_len]);
        }
        if right_len > 0 {
            right_count = count_u8(core::slice::from_raw_parts(right_ptr,right_len));
        }
        count += left_count + right_count;
        used += pre_len + right_len;
    }
    debug_assert_eq!(used,s.len());
    count
}

pub fn chars_count_256(s: &str) -> usize {
    #[cfg(target_arch = "x86_64")]
    {
        let slice: &[u8] = s.as_ref();
        if is_x86_feature_detected!("avx2") {
            unsafe {
                let (pre, mid, suf) = slice.align_to::<__m256i>();
                return count_u8(pre) + count_256(mid) + count_u8(suf);
            }
        }
    }

    //fall back
    chars_count_usize(s)
}

pub fn chars_count_usize(s: &str) -> usize {
    let slice: &[u8] = s.as_ref();
    unsafe {
        let (pre, mid, suf) = slice.align_to::<usize>();
        count_u8(pre) + count_usize(mid) + count_u8(suf)
    }
}

pub fn chars_count_u64(s: &str) -> usize {
    let slice: &[u8] = s.as_ref();
    unsafe {
        let (pre, mid, suf) = slice.align_to::<u64>();
        count_u8(pre) + count_64(mid) + count_u8(suf)
    }
}

pub fn chars_count_u32(s: &str) -> usize {
    let slice: &[u8] = s.as_ref();
    unsafe {
        let (pre, mid, suf) = slice.align_to::<u32>();
        count_u8(pre) + count_32(mid) + count_u8(suf)
    }
}

pub fn chars_count_u8(s: &str) -> usize {
    let slice: &[u8] = s.as_ref();
    count_u8(slice)
}

#[inline]
fn count_u8(slice: &[u8]) -> usize {
    let mut count = 0;
    for c in slice {
        // 0x00 <= c <= 0x 7f | 0xc0 <= c <= 0xff
        //let ci = *c as i8;
        //if ci > -0x41 { //no diff in bench with below
        if c & 0xC0 != 0x80 {
            count += 1;
        }
    }
    count
}

#[inline]
fn count_32(slice: &[u32]) -> usize {
    let mut count = 0;
    for c in slice {
        let f = c | (!c >> 1);
        let n = f & 0x_4040_4040;
        count += n.count_ones() as usize;
    }
    count
}

#[inline]
fn count_64(slice: &[u64]) -> usize {
    let mut count = 0;
    for c in slice {
        let f = c | (!c >> 1);
        let n = f & 0x_4040_4040_4040_4040;
        count += n.count_ones() as usize;
    }
    count
}

#[inline]
fn count_usize(slice: &[usize]) -> usize {
    let mut count = 0;
    for c in slice {
        let f = c | (!c >> 1);
        let n = f & 0x_4040_4040_4040_4040;
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
    let mut result = 0;
    result += (tmp >> 0) & 0xffff;
    result += (tmp >> 16) & 0xffff;
    result += (tmp >> 32) & 0xffff;
    result += (tmp >> 48) & 0xffff;
    result as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count_test_base<F>(f:F) where F : Fn(&str) -> usize {
        let a = "Hello, world!";
        assert_eq!(a.chars().count(), f(a));
        let a = "rust=錆";
        assert_eq!(a.chars().count(), f(a));
        let a = "rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆";
        assert_eq!(a.chars().count(), f(a));
        let a = "rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆
        rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;;rust=錆;rust=錆;rust=錆;;v;rust=錆;rust=錆;;v;rust=錆;rust=錆;v;rust=錆;v;v;v;rust=錆;rust=錆;rust=錆";
        assert_eq!(a.chars().count(), f(a));
    }

    #[test]
    fn count_8() {
        count_test_base(chars_count_u8);
    }

    #[test]
    fn count_32() {
        count_test_base(chars_count_u32);
    }

    #[test]
    fn count_64() {
        count_test_base(chars_count_u64);
    }

    #[test]
    fn count_usize() {
        count_test_base(chars_count_usize);
    }

    #[test]
    fn count_avx() {
        count_test_base(chars_count_256);
    }

    #[test]
    fn count_mix1() {
        count_test_base(chars_count_mix1);
    }    
    
    #[test]
    fn count_mix2() {
        count_test_base(chars_count_mix2);
    }
    
    #[test]
    fn count_mix3() {
        count_test_base(chars_count_mix3);
    }
}
