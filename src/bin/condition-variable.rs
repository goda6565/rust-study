// 条件変数
// 条件変数には、2つの操作がある。
// waitと、notifyである。
// 同じ条件変数に対して、通知をすると、待機していたスレッドが起こされる

use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| {
            loop {
                let mut q = queue.lock().unwrap();
                let item = loop {
                    if let Some(item) = q.pop_front() {
                        break item;
                    } else {
                        q = not_empty.wait(q).unwrap(); // 条件変数を待つ
                    }
                };
                drop(q);
                dbg!(item);
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one(); // 条件変数を通知
            thread::sleep(Duration::from_secs(1));
        }
    })
}
