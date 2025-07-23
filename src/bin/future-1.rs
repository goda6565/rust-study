// task
// 非同期操作は、タスクとして表現される
// タスクは、非同期な計算もしくは操作で、エグゼキュータによって管理され、完了されるまで駆動される
// タスクは、フューチャーの実行を表現したものである

// future
// futureは、非同期操作の結果を表す、プレースホルダとなるオブジェクト
// futureは、生成された時点では、アイドル状態である。
// 実行されると、Ready（結果が得られた）、Pending（結果が得られるまで待つ）のいずれかの状態になる
// futureは、pollingを、キャンセルされるか、Readyになるまで、繰り返し行う

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::task::JoinHandle;

struct CounterFuture {
    count: u32,
}

impl Future for CounterFuture {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        println!("polling with result {}", self.count);
        if self.count < 5 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}

#[tokio::main]
async fn main() {
    let counter_one = CounterFuture { count: 0 };
    let counter_two = CounterFuture { count: 0 };
    let handle_one: JoinHandle<u32> = tokio::task::spawn(async move { counter_one.await });
    let handle_two: JoinHandle<u32> = tokio::task::spawn(async move { counter_two.await });
    let (result_one, result_two) = tokio::join!(handle_one, handle_two);
    println!("result_one: {}", result_one.unwrap());
    println!("result_two: {}", result_two.unwrap());
}

// Pinについて
// Rustでは、ポインタが変わることがある。
// futureは、停止され、再開されるので、Pinでメモリアドレスを固定しておく必要がある

// use std::ptr;

// struct SelfReferential {
//     value: String,
//     self_pointer: *const String,
// }

// impl SelfReferential {
//     fn new(val: String) -> Self {
//         let mut me = Self {
//             value: val,
//             self_pointer: ptr::null(),
//         };
//         me.self_pointer = &me.value as *const String;
//         me
//     }

//     fn print(&self) {
//         unsafe {
//             println!("{}", *self.self_pointer);
//         }
//     }
// }

// fn main() {
//     let first = SelfReferential::new("Hello".to_string());
//     let move_first = first;
//     move_first.print(); // セグメンテーションフォルトになる（moveした時にメモリアドレスが変わるため）
// }

// Contextについて
// Contextは、非同期タスクを起こすためのwakerにアクセスできます。
// futureがpendingを返す前に、wakeしておくことで、エグゼキュータが実行可能であると認識します。
