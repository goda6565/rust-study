// Mutex (mutual exclusion)
// Mutexはその値が使われている間、別のスレッドをブロックする（Lockを持つスレッド）
// Mutexはロック状態と、アンロック状態の２状態しか持たない

use std::time;
use std::{sync::Mutex, thread};

fn main() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap(); // moveがない場合はデフォルトで借用を試みる
                for _ in 0..100 {
                    *guard += 1;
                }
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 1000);
    let m = Mutex::new(0);
    let start = time::SystemTime::now();
    let end = thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = m.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                thread::sleep(time::Duration::from_secs(1)); // これはロック中に行われるので、逐次的にsleepされる
            });
        }
        time::SystemTime::now()
    });
    assert_eq!(m.into_inner().unwrap(), 1000);
    let duration = end.duration_since(start).unwrap();
    println!("Elapsed: {:?}", duration);
}
