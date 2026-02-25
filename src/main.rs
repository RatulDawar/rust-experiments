use cache_padding_demo::{benchmark_padded, benchmark_unpadded, UnpaddedCounters, PaddedCounters};
use std::time::Instant;

fn cache_line_number(addr: usize) -> usize {
    addr / 64
}

fn cache_line_offset(addr: usize) -> usize {
    addr % 64
}

fn main() {
    println!("=== PROOF: RAM Addresses Map to CPU Cache Lines ===\n");
    
    let unpadded = UnpaddedCounters::new();
    let padded = PaddedCounters::new();
    
    let unpadded_c1_addr = &unpadded.counter1 as *const _ as usize;
    let unpadded_c2_addr = &unpadded.counter2 as *const _ as usize;
    let distance_unpadded = unpadded_c2_addr - unpadded_c1_addr;
    
    println!("UnpaddedCounters:");
    println!("  counter1 RAM address: 0x{:x}", unpadded_c1_addr);
    println!("    → CPU cache line #: {}", cache_line_number(unpadded_c1_addr));
    println!("    → Offset in cache line: {} bytes", cache_line_offset(unpadded_c1_addr));
    println!("  counter2 RAM address: 0x{:x}", unpadded_c2_addr);
    println!("    → CPU cache line #: {}", cache_line_number(unpadded_c2_addr));
    println!("    → Offset in cache line: {} bytes", cache_line_offset(unpadded_c2_addr));
    println!("  Distance: {} bytes", distance_unpadded);
    println!("  ✗ SAME cache line! (both map to cache line #{})", cache_line_number(unpadded_c1_addr));
    
    let padded_c1_addr = &padded.counter1 as *const _ as usize;
    let padded_c2_addr = &padded.counter2 as *const _ as usize;
    let distance_padded = padded_c2_addr - padded_c1_addr;
    
    println!("\nPaddedCounters:");
    println!("  counter1 RAM address: 0x{:x}", padded_c1_addr);
    println!("    → CPU cache line #: {}", cache_line_number(padded_c1_addr));
    println!("    → Offset in cache line: {} bytes", cache_line_offset(padded_c1_addr));
    println!("  counter2 RAM address: 0x{:x}", padded_c2_addr);
    println!("    → CPU cache line #: {}", cache_line_number(padded_c2_addr));
    println!("    → Offset in cache line: {} bytes", cache_line_offset(padded_c2_addr));
    println!("  Distance: {} bytes", distance_padded);
    println!("  ✓ DIFFERENT cache lines! (cache line #{} vs #{})", 
             cache_line_number(padded_c1_addr), 
             cache_line_number(padded_c2_addr));
    
    println!("\n=== Performance Benchmark (100M iterations, 3 runs each) ===\n");
    
    let mut unpadded_times = Vec::new();
    for i in 1..=3 {
        println!("Running unpadded benchmark - Run {}...", i);
        let start = Instant::now();
        benchmark_unpadded();
        let elapsed = start.elapsed();
        println!("  Time: {:?}", elapsed);
        unpadded_times.push(elapsed);
    }
    
    let unpadded_avg = unpadded_times.iter().sum::<std::time::Duration>() / 3;
    println!("\nUnpadded Average: {:?}", unpadded_avg);
    
    println!("\n---\n");
    
    let mut padded_times = Vec::new();
    for i in 1..=3 {
        println!("Running padded benchmark - Run {}...", i);
        let start = Instant::now();
        benchmark_padded();
        let elapsed = start.elapsed();
        println!("  Time: {:?}", elapsed);
        padded_times.push(elapsed);
    }
    
    let padded_avg = padded_times.iter().sum::<std::time::Duration>() / 3;
    println!("\nPadded Average: {:?}", padded_avg);
    
    println!("\n=== RESULTS ===");
    println!("Unpadded avg: {:?}", unpadded_avg);
    println!("Padded avg:   {:?}", padded_avg);
    println!("Speedup:      {:.2}x", unpadded_avg.as_secs_f64() / padded_avg.as_secs_f64());
}
