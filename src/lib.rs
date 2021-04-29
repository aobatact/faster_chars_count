//! Library for counting length of chars faster than [`chars()`](`str::chars`).[`count()`](`std::str::Chars::count()`)
//!
//! ```
//! use faster_chars_count::*;
//! let a = "rust=錆;rust=錆;rust=錆;rust=錆;rust=錆;";
//! assert_eq!(a.chars().count(), chars_count_str(a));
//! assert_eq!(a.chars().count(), a.chars_count());
//! ```
//! Idea is from [UTF-8のコードポイントはどうやって高速に数えるか](https://qiita.com/saka1_p/items/ff49d981cfd56f3588cc), and [UTF-8のコードポイントはどうやってもっと高速に数えるか](https://qiita.com/umezawatakeshi/items/ed23935788756c800b86).
//!
//! Idea is that we only needs to count the byte witch is not a continuation byte. This can be done at the same time for 4byte ([`u64`]) or 32byte ([`__m256i`](`core::arch::x86_64::__m256i`) with avx2).

#![feature(stdarch)]
#![feature(destructuring_assignment)]

use core::mem;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub trait CharsCount {
    fn chars_count(&self) -> usize;
}

impl CharsCount for str {
    fn chars_count(&self) -> usize {
        chars_count_mix1(&self)
    }
}

#[inline]
/// Function version of faster `chars_count()`
pub fn chars_count_str(s: &str) -> usize {
    chars_count_mix1(s)
}

//mix1 try to split the aligned block only once.
pub fn chars_count_mix1(s: &str) -> usize {
    let slice: &[u8] = s.as_ref();
    let (pre, mid_count, suf) = match slice.len() {
        // 35 is to ensure that mid.len() > 0
        35..=usize::MAX if cfg!(target_arch = "x86_64") && is_x86_feature_detected!("avx2") => unsafe {
            let (pre, mid, suf) = slice.align_to::<__m256i>();
            (pre, count_256(mid), suf)
        },
        // 11 is to ensure that mid.len() > 0
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

//mix1 + split for u32 too
pub fn chars_count_mix1a(s: &str) -> usize {
    let slice: &[u8] = s.as_ref();
    let (pre, mid_count, suf) = match slice.len() {
        35..=usize::MAX if cfg!(target_arch = "x86_64") && is_x86_feature_detected!("avx2") => unsafe {
            let (pre, mid, suf) = slice.align_to::<__m256i>();
            (pre, count_256(mid), suf)
        },
        11..=usize::MAX => unsafe {
            let (pre, mid, suf) = slice.align_to::<u64>();
            (pre, count_64(mid), suf)
        },
        7..=usize::MAX => unsafe {
            let (pre, mid, suf) = slice.align_to::<u32>();
            (pre, count_32(mid), suf)
        },
        1 => return 1,
        0 => return 0,
        _ => return count_u8(slice),
    };
    count_u8(pre) + count_u8(suf) + mid_count
}

//mix1 + bound is aligned size
pub fn chars_count_mix1b(s: &str) -> usize {
    let slice: &[u8] = s.as_ref();
    let (pre, mid_count, suf) = match slice.len() {
        32..=usize::MAX if cfg!(target_arch = "x86_64") && is_x86_feature_detected!("avx2") => unsafe {
            let (pre, mid, suf) = slice.align_to::<__m256i>();
            (pre, count_256(mid), suf)
        },
        8..=usize::MAX => unsafe {
            let (pre, mid, suf) = slice.align_to::<u64>();
            (pre, count_64(mid), suf)
        },
        4..=usize::MAX => unsafe {
            let (pre, mid, suf) = slice.align_to::<u32>();
            (pre, count_32(mid), suf)
        },
        1 => return 1,
        0 => return 0,
        _ => return count_u8(slice),
    };
    count_u8(pre) + count_u8(suf) + mid_count
}

//mix2 try to split the aligned block to remained prefix and suffix part too.
pub fn chars_count_mix2(s: &str) -> usize {
    fn align_part(slice: &[u8]) -> (&[u8], usize, &[u8]) {
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
            0 => (<&[u8]>::default(), 0, <&[u8]>::default()),
            _ => (<&[u8]>::default(), count_u8(slice), <&[u8]>::default()),
        }
    }

    let slice: &[u8] = s.as_ref();
    let (mut pre, mut count, mut suf) = align_part(slice);
    while !pre.is_empty() {
        let (pre_2, m_2, suf_2) = align_part(pre);
        debug_assert!(suf_2.is_empty());
        pre = pre_2;
        count += m_2;
    }
    while !suf.is_empty() {
        let (pre_2, m_2, suf_2) = align_part(suf);
        debug_assert!(pre_2.is_empty());
        suf = suf_2;
        count += m_2;
    }
    count
}

//mix2_suf try to split the aligned block to remained suffix part too.
pub fn chars_count_mix2_suf(s: &str) -> usize {
    fn align_part(slice: &[u8]) -> (&[u8], usize, &[u8]) {
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
            0 | 1 => (<&[u8]>::default(), len, <&[u8]>::default()),
            _ => (<&[u8]>::default(), count_u8(slice), <&[u8]>::default()),
        }
    }

    let slice: &[u8] = s.as_ref();
    let (pre, mut count, mut suf) = align_part(slice);
    count += count_u8(pre);
    while !suf.is_empty() {
        let (pre_2, m_2, suf_2) = align_part(suf);
        debug_assert!(pre_2.is_empty());
        suf = suf_2;
        count += m_2;
    }
    count
}

//check the align manually
pub fn chars_count_mix3(s: &str) -> usize {
    unsafe fn count_aligned<T>(
        pre_len: &mut usize,
        right_len: &mut usize,
        right_ptr: &mut *const u8,
        f: fn(*const u8, usize) -> usize,
    ) -> (usize, usize) {
        if *pre_len == 0 {
            let _offset = right_ptr.align_offset(mem::align_of::<T>());
            if _offset < *right_len {
                *pre_len = _offset;
                *right_len -= _offset;
                *right_ptr = right_ptr.add(_offset);
            } else {
                return (0, 0);
            }
        }
        let m_size = mem::size_of::<T>();
        let mid_t_len = *right_len / m_size;
        if mid_t_len > 0 {
            let count = (f)(*right_ptr, mid_t_len);
            let mid_len = mid_t_len * m_size;
            let used = mid_len;
            *right_ptr = right_ptr.add(mid_len); //suf_ptr
            *right_len -= mid_len;
            return (count, used);
        }
        return (0, 0);
    }

    let mut right_len = s.len();
    if right_len < 2 {
        return right_len;
    }
    let mut count = 0;
    let mut pre_len = 0;
    let mut right_ptr = s.as_ptr(); // right means middle and suffix here
    let mut used = 0; //for debug (and hope to be gone in release)
    unsafe {
        if cfg!(target_arch = "x86_64") && is_x86_feature_detected!("avx2") {
            if right_len >= 32 {
                let (c, u) = count_aligned::<__m256i>(
                    &mut pre_len,
                    &mut right_len,
                    &mut right_ptr,
                    |p, l| -> usize {
                        count_256(core::slice::from_raw_parts(p as *const __m256i, l))
                    },
                );
                count += c;
                used += u;
            }
            // for sse 128?
        }

        if right_len >= 8 {
            let (c, u) = count_aligned::<u64>(
                &mut pre_len,
                &mut right_len,
                &mut right_ptr,
                |p, l| -> usize { count_64(core::slice::from_raw_parts(p as *const u64, l)) },
            );
            count += c;
            used += u;
        }

        if right_len >= 4 {
            let (c, u) = count_aligned::<u32>(
                &mut pre_len,
                &mut right_len,
                &mut right_ptr,
                |p, _| -> usize {
                    let r_32 = *(p as *const u32);
                    let f = r_32 | (!r_32 >> 1);
                    let n = f & 0x_4040_4040;
                    n.count_ones() as usize
                },
            );
            count += c;
            used += u;
        }

        if pre_len > 0 {
            let slice: &[u8] = s.as_ref();
            count += count_u8(&slice[..pre_len]);
        }
        if right_len > 0 {
            count += count_u8(core::slice::from_raw_parts(right_ptr, right_len));
        }
        used += pre_len + right_len;
    }
    debug_assert_eq!(used, s.len());
    count
}

pub fn chars_count_mix3i(s: &str) -> usize {
    #[inline]
    unsafe fn count_aligned<T>(
        pre_len: &mut usize,
        right_len: &mut usize,
        right_ptr: &mut *const u8,
        f: fn(*const u8, usize) -> usize,
    ) -> (usize, usize) {
        if *pre_len == 0 {
            let _offset = right_ptr.align_offset(mem::align_of::<T>());
            if _offset < *right_len {
                *pre_len = _offset;
                *right_len -= _offset;
                *right_ptr = right_ptr.add(_offset);
            } else {
                return (0, 0);
            }
        }
        let m_size = mem::size_of::<T>();
        let mid_t_len = *right_len / m_size;
        if mid_t_len > 0 {
            let count = (f)(*right_ptr, mid_t_len);
            let mid_len = mid_t_len * m_size;
            let used = mid_len;
            *right_ptr = right_ptr.add(mid_len); //suf_ptr
            *right_len -= mid_len;
            return (count, used);
        }
        return (0, 0);
    }

    let mut right_len = s.len();
    if right_len < 2 {
        return right_len;
    }
    let mut count = 0;
    let mut pre_len = 0;
    let mut right_ptr = s.as_ptr(); // right means middle and suffix here
    let mut used = 0; //for debug (and hope to be gone in release)
    unsafe {
        if cfg!(target_arch = "x86_64") && is_x86_feature_detected!("avx2") {
            if right_len >= 32 {
                let (c, u) = count_aligned::<__m256i>(
                    &mut pre_len,
                    &mut right_len,
                    &mut right_ptr,
                    |p, l| -> usize {
                        count_256(core::slice::from_raw_parts(p as *const __m256i, l))
                    },
                );
                count += c;
                used += u;
            }
            // for sse 128?
        }

        if right_len >= 8 {
            let (c, u) = count_aligned::<u64>(
                &mut pre_len,
                &mut right_len,
                &mut right_ptr,
                |p, l| -> usize { count_64(core::slice::from_raw_parts(p as *const u64, l)) },
            );
            count += c;
            used += u;
        }

        if right_len >= 4 {
            let (c, u) = count_aligned::<u32>(
                &mut pre_len,
                &mut right_len,
                &mut right_ptr,
                |p, _| -> usize {
                    let r_32 = *(p as *const u32);
                    let f = r_32 | (!r_32 >> 1);
                    let n = f & 0x_4040_4040;
                    n.count_ones() as usize
                },
            );
            count += c;
            used += u;
        }

        if pre_len > 0 {
            let slice: &[u8] = s.as_ref();
            count += count_u8(&slice[..pre_len]);
        }
        if right_len > 0 {
            count += count_u8(core::slice::from_raw_parts(right_ptr, right_len));
        }
        used += pre_len + right_len;
    }
    debug_assert_eq!(used, s.len());
    count
}

pub fn chars_count_mix3t(s: &str) -> usize {
    unsafe fn count_aligned<T>(
        pre_len: usize,
        right_len: usize,
        right_ptr: *const u8,
        f: fn(*const u8, usize) -> usize,
    ) -> (usize, usize, usize, usize, *const u8) {
        let mut pre_len = pre_len;
        let mut right_len = right_len;
        let mut right_ptr = right_ptr;
        if pre_len == 0 {
            let _offset = right_ptr.align_offset(mem::align_of::<T>());
            if _offset < right_len {
                pre_len = _offset;
                right_len -= _offset;
                right_ptr = right_ptr.add(_offset);
            } else {
                return (0, 0, pre_len, right_len, right_ptr);
            }
        }
        let m_size = mem::size_of::<T>();
        let mid_t_len = right_len / m_size;
        if mid_t_len > 0 {
            let count = (f)(right_ptr, mid_t_len);
            let mid_len = mid_t_len * m_size;
            let used = mid_len;
            right_ptr = right_ptr.add(mid_len); //suf_ptr
            right_len -= mid_len;
            return (count, used, pre_len, right_len, right_ptr);
        }
        return (0, 0, pre_len, right_len, right_ptr);
    }

    let mut right_len = s.len();
    if right_len < 2 {
        return right_len;
    }
    let mut count = 0;
    let mut pre_len = 0;
    let mut right_ptr = s.as_ptr(); // right means middle and suffix here
    let mut used = 0; //for debug (and hope to be gone in release)
    unsafe {
        if cfg!(target_arch = "x86_64") && is_x86_feature_detected!("avx2") {
            if right_len >= 32 {
                let c: usize;
                let u: usize;
                (c, u, pre_len, right_len, right_ptr) =
                    count_aligned::<__m256i>(pre_len, right_len, right_ptr, |p, l| -> usize {
                        count_256(core::slice::from_raw_parts(p as *const __m256i, l))
                    });
                count += c;
                used += u;
            }
            // for sse 128?
        }

        if right_len >= 8 {
            let c: usize;
            let u: usize;
            (c, u, pre_len, right_len, right_ptr) =
                count_aligned::<u64>(pre_len, right_len, right_ptr, |p, l| -> usize {
                    count_64(core::slice::from_raw_parts(p as *const u64, l))
                });
            count += c;
            used += u;
        }

        if right_len >= 4 {
            let c: usize;
            let u: usize;
            (c, u, pre_len, right_len, right_ptr) =
                count_aligned::<u32>(pre_len, right_len, right_ptr, |p, _l| -> usize {
                    // we know that right_len is at 4..8
                    let r_32 = *(p as *const u32);
                    let f = r_32 | (!r_32 >> 1);
                    let n = f & 0x_4040_4040;
                    n.count_ones() as usize
                });
            count += c;
            used += u;
        }

        if pre_len > 0 {
            let slice: &[u8] = s.as_ref();
            count += count_u8(&slice[..pre_len]);
        }
        if right_len > 0 {
            count += count_u8(core::slice::from_raw_parts(right_ptr, right_len));
        }
        used += pre_len + right_len;
    }
    debug_assert_eq!(used, s.len());
    count
}

pub fn chars_count_mix3ti(s: &str) -> usize {
    #[inline]
    unsafe fn count_aligned<T>(
        pre_len: usize,
        right_len: usize,
        right_ptr: *const u8,
        f: fn(*const u8, usize) -> usize,
    ) -> (usize, usize, usize, usize, *const u8) {
        let mut pre_len = pre_len;
        let mut right_len = right_len;
        let mut right_ptr = right_ptr;
        if pre_len == 0 {
            let _offset = right_ptr.align_offset(mem::align_of::<T>());
            if _offset < right_len {
                pre_len = _offset;
                right_len -= _offset;
                right_ptr = right_ptr.add(_offset);
            } else {
                return (0, 0, pre_len, right_len, right_ptr);
            }
        }
        let m_size = mem::size_of::<T>();
        let mid_t_len = right_len / m_size;
        if mid_t_len > 0 {
            let count = (f)(right_ptr, mid_t_len);
            let mid_len = mid_t_len * m_size;
            let used = mid_len;
            right_ptr = right_ptr.add(mid_len); //suf_ptr
            right_len -= mid_len;
            return (count, used, pre_len, right_len, right_ptr);
        }
        return (0, 0, pre_len, right_len, right_ptr);
    }

    let mut right_len = s.len();
    if right_len < 2 {
        return right_len;
    }
    let mut count = 0;
    let mut pre_len = 0;
    let mut right_ptr = s.as_ptr(); // right means middle and suffix here
    let mut used = 0; //for debug (and hope to be gone in release)
    unsafe {
        if cfg!(target_arch = "x86_64") && is_x86_feature_detected!("avx2") {
            if right_len >= 32 {
                let c: usize;
                let u: usize;
                (c, u, pre_len, right_len, right_ptr) =
                    count_aligned::<__m256i>(pre_len, right_len, right_ptr, |p, l| -> usize {
                        count_256(core::slice::from_raw_parts(p as *const __m256i, l))
                    });
                count += c;
                used += u;
            }
            // for sse 128?
        }

        if right_len >= 8 {
            let c: usize;
            let u: usize;
            (c, u, pre_len, right_len, right_ptr) =
                count_aligned::<u64>(pre_len, right_len, right_ptr, |p, l| -> usize {
                    count_64(core::slice::from_raw_parts(p as *const u64, l))
                });
            count += c;
            used += u;
        }

        if right_len >= 4 {
            let c: usize;
            let u: usize;
            (c, u, pre_len, right_len, right_ptr) =
                count_aligned::<u32>(pre_len, right_len, right_ptr, |p, _l| -> usize {
                    // we know that right_len is at 4..8
                    let r_32 = *(p as *const u32);
                    let f = r_32 | (!r_32 >> 1);
                    let n = f & 0x_4040_4040;
                    n.count_ones() as usize
                });
            count += c;
            used += u;
        }

        if pre_len > 0 {
            let slice: &[u8] = s.as_ref();
            count += count_u8(&slice[..pre_len]);
        }
        if right_len > 0 {
            count += count_u8(core::slice::from_raw_parts(right_ptr, right_len));
        }
        used += pre_len + right_len;
    }
    debug_assert_eq!(used, s.len());
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

pub fn chars_count_256_iter(s: &str) -> usize {
    #[cfg(target_arch = "x86_64")]
    {
        let slice: &[u8] = s.as_ref();
        if is_x86_feature_detected!("avx2") {
            unsafe {
                let (pre, mid, suf) = slice.align_to::<__m256i>();
                return count_u8(pre) + count_256_iter(mid) + count_u8(suf);
            }
        }
    }

    //fall back
    chars_count_usize(s)
}

pub fn chars_count_u128(s: &str) -> usize {
    let slice: &[u8] = s.as_ref();
    unsafe {
        let (pre, mid, suf) = slice.align_to::<u128>();
        count_u8(pre) + count_u128(mid) + count_u8(suf)
    }
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
    //if slice.len() == 0 { return 0 }
    let mut count = 0;
    for c in slice {
        // (0x00 <= c <= 0x7f | 0xc0 <= c <= 0xff) == (c as i8 > -0x41)
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
fn count_u128(slice: &[u128]) -> usize {
    let mut count = 0;
    for c in slice {
        let f = c | (!c >> 1);
        let n = f & 0x_4040_4040_4040_4040_4040_4040_4040_4040;
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
unsafe fn count_256_iter(slice: &[__m256i]) -> usize {
    slice
        .chunks(255)
        .map(|chunk| {
            avx2_horizontal_sum_epi8(chunk.iter().fold(_mm256_setzero_si256(), |sum, item| {
                _mm256_sub_epi8(
                    sum,
                    _mm256_cmpgt_epi8(_mm256_load_si256(item), _mm256_set1_epi8(-0x41)),
                )
            }))
        })
        .sum()
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
    fn count_128() {
        count_test_base(chars_count_u128);
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
    fn count_avx_iter() {
        count_test_base(chars_count_256_iter);
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

    #[test]
    fn count_mix3_t() {
        count_test_base(chars_count_mix3t);
    }
}
