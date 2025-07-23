// Rustのスレッド

use std::thread;

fn main() {
    // 基本的な使い方
    thread::spawn(func);

    // クロージャー
    thread::spawn(|| println!("this is closure thread"));

    // join
    let num = 1;
    let t = thread::spawn(move || num + 1);

    let plus = t.join().unwrap();

    println!("num(1) + 1 = {plus}");

    // スコープ付き（joinされていない場合、スコープを抜ける時に自動でjoinされる）
    let numbers = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| {
            for n in &numbers {
                // 参照を受け取れる
                println!("{n}");
            }
        });
    });
}

fn func() {
    println!("this is basic thread")
}
