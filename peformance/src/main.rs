use faster_chars_count_performance::*;
use std::mem::forget;
use std::ptr;

pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = ptr::read_volatile(&dummy);
        forget(dummy);
        ret
    }
}

//for bloat
fn main() {
    let foo = "foo";
    black_box(foo.chars().count());
    black_box(chars_count_mix1(foo));
    black_box(chars_count_mix2(foo));
    black_box(chars_count_mix3(foo));
    black_box(chars_count_mix3i(foo));
    black_box(chars_count_mix3t(foo));
    black_box(chars_count_mix3ti(foo));
    black_box(chars_count_u8(foo));
    black_box(chars_count_u64(foo));
}
