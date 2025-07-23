// 参照カウンタ

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

// 参照カウンタは、Boxと似ているが、Boxは、cloneすると、コピーが作成されるのに対して、
// 参照カウンタは、連続するメモリ条に確保されている、カウンタをインクリメントする。
// このカウンタによって、ドロップするべきかを判断する。
// 所有権を共有するような形になる。

fn main() {
    // 参照カウンタを作成
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr());

    // Rcはスレッドセーフではない。
    // なぜなら、複数のスレッドから参照カウンタを変更しようとすると、予期せぬ結果になり得ないからである。
    // それに対して、Arc(atomically reference counted)は、スレッドセーフである。
    // Arcでは、参照カウンタの変更が、アトミックな操作であることが保証されている。

    // Arcを作成
    let a_a = Arc::new([1, 2, 3]);
    let a_b = a_a.clone();

    thread::spawn(|| dbg!(a_a)).join().unwrap();
    thread::spawn(|| dbg!(a_b)).join().unwrap();
}
