use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task;

struct MyFuture {
    state: Arc<Mutex<MyFutureState>>,
}

struct MyFutureState {
    value: Option<Vec<u8>>,
    waker: Option<Waker>,
}

impl MyFuture {
    fn new() -> (Self, Arc<Mutex<MyFutureState>>) {
        let state = Arc::new(Mutex::new(MyFutureState {
            value: None,
            waker: None,
        }));
        (
            Self {
                state: state.clone(),
            },
            state,
        )
    }
}

impl Future for MyFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("polling the future");
        let mut state = self.state.lock().unwrap();
        if state.value.is_some() {
            let string = state.value.take().unwrap();
            Poll::Ready(String::from_utf8(string).unwrap())
        } else {
            println!("set waker");
            state.waker = Some(cx.waker().clone()); // wakerをstateに設定する
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let (my_future, state) = MyFuture::new();
    let (tx, mut rx) = mpsc::channel::<()>(1);
    let task_handle = task::spawn(async {
        my_future.await // 1度目の実行（pending）
    });
    tokio::time::sleep(Duration::from_secs(1)).await;
    let trigger_task = task::spawn(async move {
        rx.recv().await; // メッセージを受け取るまで待機する（wakeされるまで） 
        println!("received message");
        let mut state = state.lock().unwrap();
        state.value = Some(b"Hello, world!".to_vec());
        println!("set value");
        loop {
            if let Some(waker) = state.waker.take() {
                // wakerが設定されている場合は、wakeする
                println!("looping");
                waker.wake();
                break;
            }
        }
    });
    tx.send(()).await.unwrap(); // メッセージを送信すると、待機している以下のコードが実行される
    let outcome = task_handle.await.unwrap(); // task_handleはreadyになっているので、awaitで終了する
    println!("outcome: {:?}", outcome);
    trigger_task.await.unwrap(); // trigger_taskはreadyになっているので、awaitで終了する
}
