use std::sync::atomic::AtomicPtr;
use std::{
    sync::atomic::Ordering::{Acquire, Release},
    thread,
    time::Duration,
};

// (1) ポインタを保持するための原子変数これはすべてのスレッドで共有される
// (2) ポインタを読み込む(Acquire)
// (3) もし(2)の時点でデータが生成されていなければ、データを生成する
// (4) データが生成されていなければ、データを生成する(Release)
// (5) もし他のスレッドがデータを生成していた場合は、データを破棄して、ポインタを更新する(Acquire)
// (6) ポインタを返す

fn generate_data() -> i32 {
    42
}

fn get_data(id: usize) -> &'static i32 {
    static PTR: AtomicPtr<i32> = AtomicPtr::new(std::ptr::null_mut()); // (1)

    let mut p = PTR.load(Acquire); // (2) 

    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data())); // (3)
        match PTR.compare_exchange(std::ptr::null_mut(), p, Release, Acquire) {
            Err(existing) => {
                // (4)
                drop(unsafe { Box::from_raw(p) });
                p = existing;
                println!(
                    "I am thread {}. I lost the race for generating data, but I could compete with other threads.",
                    id
                );
            }
            Ok(_) => {
                println!("I am thread {}. I won the race for generating data.", id);
            }
        }
    }
    unsafe { &*p } // (6)
}

fn main() {
    thread::scope(|s| {
        for id in 0..100000 {
            s.spawn(move || {
                thread::sleep(Duration::from_millis(50));
                let data = get_data(id);
                assert_eq!(*data, 42);
            });
        }
    });
}

// Consume Ordering
// これは、現実的に実装している例はほとんどないが、
// 考え方的には,AcquireとRelaxedの間にある
// たとえば、xがatomicであるとして、
// a+1 (1)
// x+1 (2)
// x.store(Release)
// というコードがあったとして、
// x.load(Consume or Acquire)
// とした場合、
// Acquireの場合、(1)は必ず行われていることが保証される
// Consumeの場合、(1)は行われていることが保証される
// ただ、ともに(2)は行われていることが保証される
// つまり、Consumeの場合、Atomic変数に関連する命令のみ先行発生することが保証される
