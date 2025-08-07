// スレッドA,Bがあるとする。
// この時、AがMutexでLockをしていると、Bは当然Lockを取得しようとする。
// この時、この時、Aがロックを保持していると、Bは一度スリープする。
// しかし、すぐにこのLockが解放される場合、スリープせずに再挑戦する方が効率的な場合がある。
// スピンロックはまさにこのような動作をするMutexである。

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, Release},
};
use std::thread;

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

// SyncをSpinLockに対して実装する
unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop(); // スリープせずに再挑戦する
        }
        Guard { lock: self }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<'a, T> Deref for Guard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // guardが存在することが、lockを保持していることを保証する
        unsafe { &*self.lock.value.get() }
    }
}

impl<'a, T> DerefMut for Guard<'a, T> {
    // guardが存在することが、lockを保持していることを保証する
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.value.get() }
    }
}

unsafe impl<T> Send for Guard<'_, T> where T: Send {}
unsafe impl<T> Sync for Guard<'_, T> where T: Send {}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}

fn main() {
    let x = SpinLock::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = x.lock();
                *guard += 1;
            });
        }
    });

    println!("x: {:?}", *x.lock());
}
