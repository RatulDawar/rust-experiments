use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn increment_with_atomic(threads: usize, increments_per_thread: u64) -> u64 {
    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::with_capacity(threads);

    for _ in 0..threads {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..increments_per_thread {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for handle in handles {
        handle.join().expect("thread should not panic");
    }

    counter.load(Ordering::Relaxed)
}

pub fn increment_with_mutex(threads: usize, increments_per_thread: u64) -> u64 {
    let counter = Arc::new(Mutex::new(0_u64));
    let mut handles = Vec::with_capacity(threads);

    for _ in 0..threads {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..increments_per_thread {
                let mut lock = counter.lock().expect("mutex should not be poisoned");
                *lock += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().expect("thread should not panic");
    }

    *counter.lock().expect("mutex should not be poisoned")
}

#[cfg(test)]
mod tests {
    use super::{increment_with_atomic, increment_with_mutex};

    #[test]
    fn atomic_counter_reaches_expected_total() {
        let threads = 4;
        let increments_per_thread = 1_000;
        let expected = threads as u64 * increments_per_thread;

        assert_eq!(increment_with_atomic(threads, increments_per_thread), expected);
    }

    #[test]
    fn mutex_counter_reaches_expected_total() {
        let threads = 4;
        let increments_per_thread = 1_000;
        let expected = threads as u64 * increments_per_thread;

        assert_eq!(increment_with_mutex(threads, increments_per_thread), expected);
    }
}
