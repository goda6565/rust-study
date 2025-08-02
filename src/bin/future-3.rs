// future間でのデータ共有

use core::task::Poll;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Context;
use std::time::Duration;
use tokio::task::JoinHandle;

#[derive(Debug)]
enum CounterType {
    Increment,
    Decrement,
}

struct SharedData {
    counter: i32,
}

impl SharedData {
    fn increment(&mut self) {
        self.counter += 1;
    }

    fn decrement(&mut self) {
        self.counter -= 1;
    }
}

struct CounterFuture {
    counter_type: CounterType,
    data_reference: Arc<Mutex<SharedData>>,
    count: u32,
}

impl Future for CounterFuture {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        std::thread::sleep(Duration::from_secs(1));
        let mut guard = match self.data_reference.try_lock() {
            // ロックを取得
            Ok(guard) => guard,
            Err(error) => {
                println!("error: {:?}: {}", self.counter_type, error);
                cx.waker().wake_by_ref(); // 失敗したらwakerを起動してpendingにする
                return Poll::Pending;
            }
        };
        let value = &mut *guard;
        match self.counter_type {
            CounterType::Increment => {
                value.increment();
                println!("increment: {:?}", value.counter);
            }
            CounterType::Decrement => {
                value.decrement();
                println!("decrement: {:?}", value.counter);
            }
        }
        std::mem::drop(guard); // ロックを開放
        self.count += 1;
        if self.count < 3 {
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
        Poll::Ready(self.count)
    }
}

#[tokio::main]
async fn main() {
    let shared_data = Arc::new(Mutex::new(SharedData { counter: 0 }));
    let counter_one = CounterFuture {
        counter_type: CounterType::Increment,
        data_reference: shared_data.clone(),
        count: 0,
    };
    let counter_two = CounterFuture {
        counter_type: CounterType::Decrement,
        data_reference: shared_data.clone(),
        count: 0,
    };
    let handle_one: JoinHandle<u32> = tokio::task::spawn(async move { counter_one.await });
    let handle_two: JoinHandle<u32> = tokio::task::spawn(async move { counter_two.await });
    let (result_one, result_two) = tokio::join!(handle_one, handle_two);
    println!("result_one: {:?}, result_two: {:?}", result_one, result_two);
    println!("shared_data: {:?}", shared_data.lock().unwrap().counter);
}
