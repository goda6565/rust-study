// fetch_and_modify

// これは、読み込みと書き込みを不可分な操作として扱う

use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    // fetch_addは、現在の値を返すから、内部的に1つインクリメントされる
    let id = NEXT_ID.fetch_add(1, Relaxed);
    assert!(id < 1000, "too many ids");
    println!("allocated id: {}", id);
    id
}

fn main() {
    let handle = std::thread::spawn(move || {
        for _ in 0..100000 {
            allocate_new_id();
        }
    });

    handle.join().unwrap();
}
