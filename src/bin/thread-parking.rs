// スレッドパーキング
// それぞれのスレッドは、自分自身をparkすることができる。
// parkすると、そのスレッドは、休止して、CPUを消費しない

use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let queue = Mutex::new(VecDeque::new());
    thread::scope(|s| {
        let t = s.spawn(|| {
            loop {
                let item = queue.lock().unwrap().pop_front();
                if let Some(item) = item {
                    dbg!(item);
                } else {
                    thread::park(); // スレッドをパーク
                }
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark(); // スレッドをアンパーク（スレッドを知っている必要がある）
            thread::sleep(Duration::from_secs(1));
        }
    })
}
