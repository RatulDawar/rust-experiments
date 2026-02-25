use cache_padding_demo::benchmark_padded;

fn main() {
    for _ in 0..10 {
        benchmark_padded();
    }
}
