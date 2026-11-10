# Tuple dispatch benchmark

Standalone benchmark and generator, kept separate from the library experiment.
It generates real heterogeneous function-item groups for every champion and
item/rune pair, compares outputs with the existing library, and measures tuples,
pointer slices, enum slices, and the existing optimized batch strategies.

See [the results and methodology](../benchmarks/TUPLES.md).

From the repository root:

```console
cargo +nightly build --offline --release --manifest-path nightly/tuple-bench/Cargo.toml --target-dir nightly/target -j 4
nightly/target/release/tutorlolv2_tuple_bench --validate-only
taskset -c 4 nightly/target/release/tutorlolv2_tuple_bench > /tmp/tuples.json
```

The full calculator in `src/engine.rs` is a benchmark snapshot with a common
pipeline, not a replacement for the library calculator. It deliberately keeps
allocation and preparation common to the strategies; it does not attempt every
possible tuple-specific optimization. The library and its generator are unchanged.
