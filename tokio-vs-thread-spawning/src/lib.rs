use std::thread;
use std::time::Duration;

use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinSet;

pub fn default_worker_threads() -> usize {
    thread::available_parallelism()
        .map(|value| value.get())
        .unwrap_or(4)
}

pub struct TokioHarness {
    runtime: Runtime,
}

impl TokioHarness {
    pub fn new(worker_threads: usize) -> Self {
        let runtime = Builder::new_multi_thread()
            .worker_threads(worker_threads)
            .enable_time()
            .build()
            .expect("tokio runtime should build");

        Self { runtime }
    }

    pub fn spawn_noop(&self, task_count: usize) -> usize {
        self.runtime.block_on(async move {
            let mut set = JoinSet::new();

            for task_id in 0..task_count {
                set.spawn(async move { task_id });
            }

            collect_usize(&mut set).await
        })
    }

    pub fn spawn_sleep(&self, task_count: usize, sleep: Duration) -> usize {
        self.runtime.block_on(async move {
            let mut set = JoinSet::new();

            for task_id in 0..task_count {
                set.spawn(async move {
                    tokio::time::sleep(sleep).await;
                    task_id
                });
            }

            collect_usize(&mut set).await
        })
    }

    pub fn spawn_cpu(&self, task_count: usize, iterations: u64) -> u64 {
        self.runtime.block_on(async move {
            let mut set = JoinSet::new();

            for task_id in 0..task_count {
                set.spawn(async move { cpu_burn(iterations, task_id as u64) });
            }

            collect_u64(&mut set).await
        })
    }

    pub fn spawn_blocking_cpu(&self, task_count: usize, iterations: u64) -> u64 {
        self.runtime.block_on(async move {
            let mut set = JoinSet::new();

            for task_id in 0..task_count {
                set.spawn_blocking(move || cpu_burn(iterations, task_id as u64));
            }

            collect_u64(&mut set).await
        })
    }
}

pub fn spawn_threads_noop(task_count: usize) -> usize {
    let mut handles = Vec::with_capacity(task_count);

    for task_id in 0..task_count {
        handles.push(thread::spawn(move || task_id));
    }

    handles
        .into_iter()
        .map(|handle| handle.join().expect("thread should not panic"))
        .sum()
}

pub fn spawn_threads_sleep(task_count: usize, sleep: Duration) -> usize {
    let mut handles = Vec::with_capacity(task_count);

    for task_id in 0..task_count {
        handles.push(thread::spawn(move || {
            thread::sleep(sleep);
            task_id
        }));
    }

    handles
        .into_iter()
        .map(|handle| handle.join().expect("thread should not panic"))
        .sum()
}

pub fn spawn_threads_cpu(task_count: usize, iterations: u64) -> u64 {
    let mut handles = Vec::with_capacity(task_count);

    for task_id in 0..task_count {
        handles.push(thread::spawn(move || cpu_burn(iterations, task_id as u64)));
    }

    let mut total: u64 = 0;

    for handle in handles {
        total = total.wrapping_add(handle.join().expect("thread should not panic"));
    }

    total
}

pub fn cpu_burn(iterations: u64, seed: u64) -> u64 {
    let mut state = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);

    for round in 0..iterations {
        state ^= round.wrapping_mul(0xBF58_476D_1CE4_E5B9);
        state = state.rotate_left(13);
        state = state.wrapping_mul(0x94D0_49BB_1331_11EB);
        state ^= state >> 17;
    }

    state
}

async fn collect_usize(set: &mut JoinSet<usize>) -> usize {
    let mut total = 0;

    while let Some(result) = set.join_next().await {
        total += result.expect("task should not panic");
    }

    total
}

async fn collect_u64(set: &mut JoinSet<u64>) -> u64 {
    let mut total: u64 = 0;

    while let Some(result) = set.join_next().await {
        total = total.wrapping_add(result.expect("task should not panic"));
    }

    total
}

#[cfg(test)]
mod tests {
    use super::{
        TokioHarness, cpu_burn, default_worker_threads, spawn_threads_cpu, spawn_threads_noop,
        spawn_threads_sleep,
    };
    use std::time::Duration;

    #[test]
    fn noop_counts_match_between_threads_and_tokio() {
        let task_count = 128;
        let tokio = TokioHarness::new(default_worker_threads());

        assert_eq!(spawn_threads_noop(task_count), tokio.spawn_noop(task_count));
    }

    #[test]
    fn sleep_counts_match_between_threads_and_tokio() {
        let task_count = 64;
        let tokio = TokioHarness::new(default_worker_threads());

        assert_eq!(
            spawn_threads_sleep(task_count, Duration::from_millis(1)),
            tokio.spawn_sleep(task_count, Duration::from_millis(1))
        );
    }

    #[test]
    fn cpu_counts_match_between_threads_and_tokio() {
        let task_count = 24;
        let iterations = 50_000;
        let tokio = TokioHarness::new(default_worker_threads());

        assert_eq!(
            spawn_threads_cpu(task_count, iterations),
            tokio.spawn_cpu(task_count, iterations)
        );
        assert_eq!(
            spawn_threads_cpu(task_count, iterations),
            tokio.spawn_blocking_cpu(task_count, iterations)
        );
    }

    #[test]
    fn cpu_burn_changes_with_seed() {
        assert_ne!(cpu_burn(10_000, 1), cpu_burn(10_000, 2));
    }
}
