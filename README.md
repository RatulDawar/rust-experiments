# Cache Padding Performance Experiment

This project demonstrates the performance benefits of cache padding in concurrent scenarios using Rust.

## The Problem: False Sharing

When multiple threads access different variables that happen to share the same cache line, they cause unnecessary cache coherency traffic. This is called "false sharing" because the threads aren't actually sharing data, but the CPU cache system treats them as if they are.

## The Solution: Cache Padding

By padding structures to ensure each frequently-accessed variable occupies its own cache line (typically 64 bytes), we eliminate false sharing and dramatically improve performance.

## Implementation

### Unpadded Counters (False Sharing)
```rust
pub struct UnpaddedCounters {
    pub counter1: AtomicU64,  // 8 bytes
    pub counter2: AtomicU64,  // 8 bytes - shares cache line with counter1!
}
```

### Padded Counters (No False Sharing)
```rust
#[repr(C, align(64))]
pub struct PaddedCounters {
    pub counter1: AtomicU64,  // 8 bytes
    _pad1: [u8; 56],          // 56 bytes padding
    pub counter2: AtomicU64,  // 8 bytes - on its own cache line
    _pad2: [u8; 56],          // 56 bytes padding
}
```

## Benchmark Results

Run the demo:
```bash
cargo run --release
```

Run full benchmarks:
```bash
cargo bench
```

### Expected Results
- **Unpadded**: ~75ms (false sharing causes cache contention)
- **Padded**: ~16ms (each counter on its own cache line)
- **Speedup**: ~4.6x faster with padding

## Why This Happens

1. CPU cache lines are typically 64 bytes
2. When thread 1 writes to counter1, it invalidates the entire cache line
3. Thread 2's counter2 is on the same cache line, so its cache is invalidated too
4. This causes constant cache thrashing between cores
5. With padding, each counter is on a separate cache line, eliminating the problem

## Key Takeaway

Cache padding is essential for high-performance concurrent data structures where different threads frequently modify nearby memory locations.
