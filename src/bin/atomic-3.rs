// 比較交換操作

use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let mut id = NEXT_ID.load(Relaxed); // 現在の値を読み込む 
    loop {
        assert!(id < 1000, "too many ids");
        // 現在の値と値を比較して、一致したら新しい値に更新する（今回の例では、他のスレッドがないので、storeでも問題ない）
        // ただ、他のスレッドがいる場合は、compare_exchangeを使う必要がある
        match NEXT_ID.compare_exchange(id, id + 1, Relaxed, Relaxed) {
            Ok(_) => return id,
            Err(e) => id = e,
        }
    }
}

fn main() {
    let handle = std::thread::spawn(move || {
        for _ in 0..100000 {
            let id = allocate_new_id();
            println!("allocated id: {}", id);
        }
    });

    handle.join().unwrap();
}
