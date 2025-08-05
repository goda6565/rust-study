// mutex

use std::{
    sync::atomic::{
        AtomicBool,
        Ordering::{Acquire, Relaxed, Release},
    },
    thread,
};

static mut DATA: i32 = 0;
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    while LOCKED
        .compare_exchange(false, true, Acquire, Relaxed)
        .is_err()
    {
        thread::yield_now(); // スレッドを休止する
    }
    unsafe {
        DATA += 1;
    }
    LOCKED.store(false, Release);
}
fn main() {
    thread::scope(|s| {
        for _ in 0..100000 {
            s.spawn(|| {
                f();
            });
        }
    });
    println!("DATA: {}", unsafe { DATA });
}
