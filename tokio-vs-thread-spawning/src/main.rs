use std::fmt::Debug;
use std::time::{Duration, Instant};

use tokio_vs_thread_spawning::{
    TokioHarness, default_worker_threads, spawn_threads_cpu, spawn_threads_noop,
    spawn_threads_sleep,
};

const RUNS: usize = 5;

struct Measurement {
    label: &'static str,
    runs: Vec<Duration>,
    median: Duration,
    average: Duration,
}

fn main() {
    let worker_threads = default_worker_threads();
    let tokio = TokioHarness::new(worker_threads);

    let spawn_tasks = 1_000;
    let sleep_tasks = 1_000;
    let sleep_for = Duration::from_millis(10);
    let cpu_tasks = worker_threads * 2;
    let cpu_iterations = 20_000_000;

    println!("Tokio vs thread spawning");
    println!("Logical CPUs: {worker_threads}");
    println!("Tokio worker threads: {worker_threads}");
    println!();

    println!("Scenario 1: Spawn overhead ({spawn_tasks} units, work = return task id and exit)");
    let thread_spawn = measure("std::thread::spawn", || spawn_threads_noop(spawn_tasks));
    let tokio_spawn = measure("tokio::spawn", || tokio.spawn_noop(spawn_tasks));
    print_measurements(&[thread_spawn, tokio_spawn]);

    println!();
    println!(
        "Scenario 2: Mostly waiting ({sleep_tasks} units, each waits {} ms)",
        sleep_for.as_millis()
    );
    let thread_sleep = measure("std::thread::spawn + sleep", || {
        spawn_threads_sleep(sleep_tasks, sleep_for)
    });
    let tokio_sleep = measure("tokio::spawn + sleep", || {
        tokio.spawn_sleep(sleep_tasks, sleep_for)
    });
    print_measurements(&[thread_sleep, tokio_sleep]);

    println!();
    println!("Scenario 3: CPU-bound ({cpu_tasks} units, {cpu_iterations} rounds each, no yield)");
    let thread_cpu = measure("std::thread::spawn", || {
        spawn_threads_cpu(cpu_tasks, cpu_iterations)
    });
    let tokio_cpu = measure("tokio::spawn", || {
        tokio.spawn_cpu(cpu_tasks, cpu_iterations)
    });
    let tokio_blocking = measure("tokio::task::spawn_blocking", || {
        tokio.spawn_blocking_cpu(cpu_tasks, cpu_iterations)
    });
    print_measurements(&[thread_cpu, tokio_cpu, tokio_blocking]);
}

fn measure<T>(label: &'static str, mut action: impl FnMut() -> T) -> Measurement
where
    T: Eq + Copy + Debug,
{
    let mut runs = Vec::with_capacity(RUNS);
    let mut expected = None;

    for _ in 0..RUNS {
        let start = Instant::now();
        let result = action();
        let elapsed = start.elapsed();

        if let Some(expected) = expected {
            assert_eq!(expected, result, "measurement result should be stable");
        } else {
            expected = Some(result);
        }

        runs.push(elapsed);
    }

    let average = average_duration(&runs);
    let median = median_duration(&runs);

    Measurement {
        label,
        runs,
        median,
        average,
    }
}

fn print_measurements(measurements: &[Measurement]) {
    let best = measurements
        .iter()
        .map(|measurement| measurement.median)
        .min()
        .expect("at least one measurement");

    for measurement in measurements {
        println!(
            "{:<30} median {:>8.2} ms | avg {:>8.2} ms | runs {}",
            measurement.label,
            duration_ms(measurement.median),
            duration_ms(measurement.average),
            render_runs(&measurement.runs)
        );
    }

    for measurement in measurements {
        println!(
            "{:<30} speedup vs best: {:>5.2}x",
            measurement.label,
            duration_ms(measurement.median) / duration_ms(best)
        );
    }
}

fn render_runs(runs: &[Duration]) -> String {
    runs.iter()
        .map(|duration| format!("{:.2}", duration_ms(*duration)))
        .collect::<Vec<_>>()
        .join(", ")
}

fn duration_ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1_000.0
}

fn average_duration(runs: &[Duration]) -> Duration {
    let total = runs.iter().copied().sum::<Duration>();
    Duration::from_secs_f64(total.as_secs_f64() / runs.len() as f64)
}

fn median_duration(runs: &[Duration]) -> Duration {
    let mut sorted = runs.to_vec();
    sorted.sort_unstable();
    sorted[sorted.len() / 2]
}
