// Release/Acquire ordering
// ReleaseとAcquireは対で使用され、スレッド間でのhappens-before関係（先行発生関係）を確立する。
// Releaseストアと、それを読み取るAcquireロードがペアになると、
// Releaseより前に行われたすべてのメモリ操作は、Acquire以降の操作よりも前に完了したことが保証される。
// つまり、Acquireがある値を読み込んだとき、
// その値を書き込んだRelease操作が一意に特定できる場合、
// そのReleaseの前に行われたすべての操作も、Acquireの後には完了して観測可能になる。

use std::sync::atomic::{
    AtomicBool, AtomicU64,
    Ordering::{Acquire, Relaxed, Release},
};
use std::thread;
use std::time::Duration;

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        DATA.store(123, Relaxed);
        READY.store(true, Release); // (1)
    });

    while !READY.load(Acquire) {
        // (2)
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    let x = DATA.load(Relaxed); // (3)
    println!("x is {x}");
}

// この例では、(2)でtrueが観測された時、(1)の操作以前のすべての操作が完了していることが保証される。
// つまり、(3)で読み込んだ値は、123であることが保証される。
// DATA.store(123, Relaxed);は、atomic操作でなくても、実際は問題がない
