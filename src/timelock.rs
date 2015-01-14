extern crate time;

use std::sync::Arc;
use std::sync::atomic::{AtomicInt, Ordering};

pub fn current() -> isize { time::get_time().sec as isize }
pub fn get(t: &Arc<AtomicInt>) -> isize { t.load(Ordering::SeqCst) }
pub fn new() -> Arc<AtomicInt> { Arc::new(AtomicInt::new(time::get_time().sec as isize)) }
pub fn update(t: &Arc<AtomicInt>) { t.store(time::get_time().sec as isize, Ordering::SeqCst); }
