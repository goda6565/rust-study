// スレッド安全性：Sync Send
// Rustでは、SyncもしくはSendトレイトを持っているかどうかで、unsafeかを判定する

// Send
// ある型が、Sendであれば安全に別のスレッドに送信できる

// Sync
// ある型が、Syncであれば、安全に別のスレッドと共有できる
// つまり、&TがSendであれば、Syncである

// これらのトレイトは自動トレイトである
// このトレイトが、自動で実装されるのを防ぐには、これらを持たない、フィールドを持たせればいい
// PhantomData<T>はコンパイラにはと扱われるが、サイズは０である

use std::marker::PhantomData;
use std::thread;

#[derive(Debug)]
struct X {
    _x: i32, // sendでもあり、syncでもある
}

#[derive(Debug)]
struct XNotSync {
    _x: i32,                   // sendでもあり、syncでもある
    _y: PhantomData<*mut i32>, // ポインタ型は、syncでもsendでもない
}

#[derive(Debug)]
struct XImpl {
    _y: PhantomData<*mut i32>, // ポインタ型は、syncでもsendでもない
}

// 手動実装も可能だが、保証はされない
unsafe impl Sync for XImpl {}

unsafe impl Send for XImpl {}

fn main() {
    let a = X { _x: 1 };
    thread::spawn(move || {
        dbg!(a);
    });

    let _b = XNotSync {
        _x: 1,
        _y: PhantomData,
    };
    // thread::spawn( move || { // エラーが出る（Sendできない）
    //     dbg!(_b);
    // });

    let c = XImpl { _y: PhantomData };
    thread::spawn(move || {
        dbg!(c);
    });
}
