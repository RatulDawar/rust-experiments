use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

const NUM_ITERATIONS: u64 = 100_000_000;

#[repr(C)]
pub struct UnpaddedCounters {
    pub counter1: AtomicU64,
    pub counter2: AtomicU64,
}

impl UnpaddedCounters {
    pub fn new() -> Self {
        Self {
            counter1: AtomicU64::new(0),
            counter2: AtomicU64::new(0),
        }
    }
}

#[repr(C, align(64))]
pub struct PaddedCounters {
    pub counter1: AtomicU64,
    _pad1: [u8; 56],
    pub counter2: AtomicU64,
    _pad2: [u8; 56],
}

impl PaddedCounters {
    pub fn new() -> Self {
        Self {
            counter1: AtomicU64::new(0),
            _pad1: [0; 56],
            counter2: AtomicU64::new(0),
            _pad2: [0; 56],
        }
    }
}

pub fn benchmark_unpadded() {
    let counters = Arc::new(UnpaddedCounters::new());
    
    let counter1 = counters.clone();
    let handle1 = thread::spawn(move || {
        for _ in 0..NUM_ITERATIONS {
            counter1.counter1.fetch_add(1, Ordering::Relaxed);
        }
    });
    
    let counter2 = counters.clone();
    let handle2 = thread::spawn(move || {
        for _ in 0..NUM_ITERATIONS {
            counter2.counter2.fetch_add(1, Ordering::Relaxed);
        }
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
}

pub fn benchmark_padded() {
    let counters = Arc::new(PaddedCounters::new());
    
    let counter1 = counters.clone();
    let handle1 = thread::spawn(move || {
        for _ in 0..NUM_ITERATIONS {
            counter1.counter1.fetch_add(1, Ordering::Relaxed);
        }
    });
    
    let counter2 = counters.clone();
    let handle2 = thread::spawn(move || {
        for _ in 0..NUM_ITERATIONS {
            counter2.counter2.fetch_add(1, Ordering::Relaxed);
        }
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
}
