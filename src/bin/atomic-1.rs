// atomic（原子性）
// 完全に行われるか、まったく行われないかのどちらかであることを保証する
// success or failure

// アトミック型は共有参照を通して、変更することができる
// 全てのアトミック型は同じメゾットを持つ
// fetch_and_modify
// compare_and_exchange

// 全てのアトミック型は、std::sync::atomic::Orderingの引数を持つ

// 例: ストップフラグ

use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // 何かをするスレッド
    let background_task = std::thread::spawn(move || {
        while !STOP.load(Relaxed) {
            println!("running");
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    });

    // user input
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {}", cmd),
        }
    }

    // ストップフラグを立てる
    STOP.store(true, Relaxed);

    // スレッドをjoinする
    background_task.join().unwrap();
}
