use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use tokio_vs_thread_spawning::{
    TokioHarness, default_worker_threads, spawn_threads_cpu, spawn_threads_noop,
};

fn bench_spawn_overhead(c: &mut Criterion) {
    let tokio = TokioHarness::new(default_worker_threads());
    let mut group = c.benchmark_group("spawn_overhead");

    for &task_count in &[100_usize, 1_000] {
        group.throughput(Throughput::Elements(task_count as u64));

        group.bench_with_input(
            BenchmarkId::new("std_thread_spawn_noop", task_count),
            &task_count,
            |b, &task_count| b.iter(|| black_box(spawn_threads_noop(task_count))),
        );

        group.bench_with_input(
            BenchmarkId::new("tokio_spawn_noop", task_count),
            &task_count,
            |b, &task_count| b.iter(|| black_box(tokio.spawn_noop(task_count))),
        );
    }

    group.finish();
}

fn bench_cpu_work(c: &mut Criterion) {
    let tokio = TokioHarness::new(default_worker_threads());
    let task_count = default_worker_threads();
    let iterations = 2_000_000_u64;
    let mut group = c.benchmark_group("cpu_bound");
    group.sample_size(10);
    group.throughput(Throughput::Elements(task_count as u64 * iterations));

    group.bench_function("std_thread_spawn_cpu", |b| {
        b.iter(|| black_box(spawn_threads_cpu(task_count, iterations)))
    });

    group.bench_function("tokio_spawn_cpu", |b| {
        b.iter(|| black_box(tokio.spawn_cpu(task_count, iterations)))
    });

    group.bench_function("tokio_spawn_blocking_cpu", |b| {
        b.iter(|| black_box(tokio.spawn_blocking_cpu(task_count, iterations)))
    });

    group.finish();
}

criterion_group!(benches, bench_spawn_overhead, bench_cpu_work);
criterion_main!(benches);
