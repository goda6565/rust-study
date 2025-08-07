// mutex

use std::{
    sync::atomic::{
        AtomicBool,
        Ordering::{Acquire, Relaxed, Release},
    },
    thread,
};

// 他のスレッドが Acquire でロックを取得した時、
// Release 側で行われた操作（DATA += 1）を必ず観測できる

static mut DATA: i32 = 0;
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    while LOCKED
        .compare_exchange(false, true, Acquire, Relaxed) // (2) ここで、falseであれば、別のスレッドの、データ変換が完了していることが保証される
        .is_err()
    {
        thread::yield_now(); // スレッドを休止する
    }
    unsafe {
        DATA += 1;
    }
    LOCKED.store(false, Release); // (1)
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
