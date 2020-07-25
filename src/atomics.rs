use std::sync::atomic::{AtomicUsize, Ordering};

pub fn atomics() {
    let x = AtomicUsize::new(0);
    let mut result = x.load(Ordering::Acquire);
    result += 1;
    x.store(result, Ordering::Release); // The value is now 1.

    println!("Atomic value: {}", x.load(Ordering::Acquire));
}