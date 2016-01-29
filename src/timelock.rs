extern crate time;

use std::sync::Arc;
use std::sync::atomic::{AtomicIsize, Ordering};

pub fn current() -> isize {
    time::get_time().sec as isize
}
pub fn get(t: &Arc<AtomicIsize>) -> isize {
    t.load(Ordering::SeqCst)
}
pub fn new() -> Arc<AtomicIsize> {
    Arc::new(AtomicIsize::new(time::get_time().sec as isize))
}
pub fn update(t: &Arc<AtomicIsize>) {
    t.store(time::get_time().sec as isize, Ordering::SeqCst);
}
