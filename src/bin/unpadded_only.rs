use cache_padding_demo::benchmark_unpadded;

fn main() {
    for _ in 0..10 {
        benchmark_unpadded();
    }
}
