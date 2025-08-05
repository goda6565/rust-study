// Relaxed ordering
// 1つのアトミック変数に対しては、Relaxed メモリオーダーリングであっても、すべての書き込み操作には一意の順序（修正の全順序）が存在する。
// これにより、複数のスレッドが同時に加算を行っても、最終的な合計値は必ず正しくなる。

use std::sync::atomic::{AtomicI32, Ordering::Relaxed};
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn a() {
    X.store(10, Relaxed); // (1)
    Y.store(20, Relaxed); // (2)
}

fn b() {
    let y = Y.load(Relaxed); // (3)
    let x = X.load(Relaxed); // (4)
    println!("{x} {y}");
}

fn main() {
    thread::scope(|s| {
        s.spawn(|| {
            a();
        });
        s.spawn(|| {
            b();
        });
    })
}

// この例では、Relaxed オーダーリングは、
// (3) は、0か10
// (4) は、0か20
// ということしか保証しない。
// したがって、直感には反するが、0,20の組み合わせもありえる。
