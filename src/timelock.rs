extern crate time;

use std::sync::Arc;
use std::sync::atomic::{AtomicInt, Ordering};

pub fn current() -> int { time::get_time().sec as int }
pub fn get(t: &Arc<AtomicInt>) -> int { t.load(Ordering::SeqCst) }
pub fn new() -> Arc<AtomicInt> { Arc::new(AtomicInt::new(time::get_time().sec as int)) }
pub fn update(t: &Arc<AtomicInt>) { t.store(time::get_time().sec as int, Ordering::SeqCst); }
