use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::task::JoinHandle;

struct UnboundedFuture {
    count: u32,
}

impl Future for UnboundedFuture {
    type Output = u32;

    // このfutureは終了しない
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        println!("polling with result unbounded future {}", self.count);
        cx.waker().wake_by_ref();
        std::thread::sleep(std::time::Duration::from_secs(1));
        Poll::Pending
    }
}

struct UnCalledFuture {
    count: u32,
}

impl Future for UnCalledFuture {
    type Output = u32;

    // このfutureは1度しかpollされない
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("polling with result uncalled future {}", self.count);
        Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    let unbounded_future = UnboundedFuture { count: 0 };
    let uncalled_future = UnCalledFuture { count: 0 };
    let handle_one: JoinHandle<u32> = tokio::task::spawn(async move { unbounded_future.await });
    let handle_two: JoinHandle<u32> = tokio::task::spawn(async move { uncalled_future.await });
    let (result_one, result_two) = tokio::join!(handle_one, handle_two);
    println!("result_one: {:?}, result_two: {:?}", result_one, result_two);
}
