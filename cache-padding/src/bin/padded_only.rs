use cache_padding::benchmark_padded;

fn main() {
    for _ in 0..10 {
        benchmark_padded();
    }
}
