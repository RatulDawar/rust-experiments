use cache_padding::benchmark_unpadded;

fn main() {
    for _ in 0..10 {
        benchmark_unpadded();
    }
}
