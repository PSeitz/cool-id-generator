
# cool-id-generator

[![](http://meritbadge.herokuapp.com/cool-id-generator)](https://crates.io/crates/cool-id-generator)

This is a no_std cool-id-generator.
It makes memorable ids.
honest-turbo-tailor-gregory, romantic-robot-chicken-kenneth and happy-ultra-barista-shane would approve.

### Installation

Simply add a corresponding entry to your `Cargo.toml` dependency list:

```toml,ignore
[dependencies]
cool-id-generator = "1.0"
```

### Tests

`cargo test -- --nocapture`

### Benchmark

`cargo bench`

```
running 3 tests
test tests::bench_id           ... bench:         243 ns/iter (+/- 51)
test tests::bench_long_id      ... bench:         298 ns/iter (+/- 63)
test tests::bench_very_long_id ... bench:         328 ns/iter (+/- 46)
```
