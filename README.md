# Rust Experiments

A collection of Rust performance experiments and systems programming explorations.

## Experiments

### [cache-padding](./cache-padding/)
Demonstrates the performance impact of false sharing and cache padding in concurrent code.

**Key Results:**
- 4.6x speedup with cache line padding
- 289x fewer cache coherency events

**Read More:** [Blog Post](https://ratuldawar.github.io/posts/cache-padding-false-sharing.html)

## Running Experiments

Each experiment is a separate Cargo workspace member:

```bash
# Clone the repository
git clone https://github.com/RatulDawar/rust-experiments
cd rust-experiments

# Run a specific experiment
cargo run --release -p cache-padding --bin demo

# Run benchmarks
cargo bench -p cache-padding
```

## Blog

Technical writeups for these experiments are published at:
- **GitHub Pages**: https://ratuldawar.github.io
- **Medium**: https://medium.com/@ratuldawar11

## Structure

This repository contains **code only**. Blog content is managed in [RatulDawar.github.io](https://github.com/RatulDawar/RatulDawar.github.io).

See [AGENTS.md](./AGENTS.md) for the blog creation workflow.

## Contributing

Feel free to:
- Open issues for questions or suggestions
- Submit PRs for improvements
- Share your own experiments

## License

MIT
