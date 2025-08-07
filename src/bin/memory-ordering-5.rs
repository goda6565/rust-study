// Sequential Consistent Ordering
// 名前の通り、これは最も強い順序付けである。
// つまり、あるアトミック命令の前後の順序を保証する。

use std::{
    sync::atomic::{AtomicBool, Ordering::SeqCst},
    thread,
};

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

// このコードでは、aとbのどちらかが競争に勝つことが保証される。
// つまり、(2)がloadした時点で、trueだった場合、
// (3)、(4)のどちらも実行されていることが保証される。

#[allow(static_mut_refs)] // Rust2024では、static mutの参照を使用すると警告が出るため、無視する
fn main() {
    let a = thread::spawn(|| {
        A.store(true, SeqCst); // (1)
        if !B.load(SeqCst) {
            // (2)
            unsafe {
                S.push('x');
            }
        }
    });

    let b = thread::spawn(|| {
        B.store(true, SeqCst); // (3)
        if !A.load(SeqCst) {
            // (4)
            unsafe {
                S.push('y');
            }
        }
    });

    a.join().unwrap();
    b.join().unwrap();

    unsafe {
        println!("{}", S);
    }
}

// SepCstは、Acquire/Releaseと何が違うのか、
// 例として、スレッドA,B,Cがあったとして、
// スレッドAがReleaseしたAtomic変数を、BがAcquireしたすると、
// もちろん、スレッドBから見ると、AのReleaseより前の命令は実行されているように見える。
// しかし、スレッドCからは、その順序は一歳保証されていない。
// しかし、SeqCstは、その順序を保証する。これが、グローバルに一貫した順序保証である。
