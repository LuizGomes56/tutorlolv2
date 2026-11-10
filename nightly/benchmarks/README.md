# Enum dispatch measurement and tuple probe

Measured on 2026-11-10 with rustc 1.102.0-nightly (ac8310ba6, LLVM 23.1.5), on an AMD Ryzen 9 8940HX under WSL2. Both libraries use the same nightly compiler and only the `engine` feature. The toolchain file tracks floating `nightly`; the exact compiler commit is recorded in `summary.json`.

## Method

- Original generated function pointers and experimental generated const-callable enums run in the same executable, using the same release profile: opt-level 3, fat LTO, one codegen unit, panic abort.
- Process affinity is CPU 4. Three independent process runs, 21 paired samples per workload per run; pointer/enum timing order alternates within each run. Calibration selects at least 20 ms of pointer work per sample. No samples are removed.
- Workloads are prepared outside timing. `black_box` prevents precomputation and removal of evaluated outputs. Preflight result checksums must agree before every comparison.
- Multi-champion cases cover all 173 champions. Raw formula cases execute 1,405 calls including damaging item/rune pairs. Grouped and deterministic shuffled orders repeat; this is a warm workload, not a cold-start or continuously randomized test.
- Fixed champion cases use Neeko (6 abilities) and Leblanc (12, the current maximum). Complete calculator fixtures use the same three items, one rune, one enemy, and normal monster rows. Allocation and destruction are timed for complete calculators.
- Formula/batch comparisons isolate callable dispatch in matching loops. Ability rows compare matching metadata/evaluation loops. **Complete calculator comparisons include the whole experimental calculator refactor and changed storage, so their difference cannot be attributed solely to the enum.**
- Results are local wall-clock measurements under a hypervisor, not isolated hardware performance counters. In particular, branch/layout/cache/system effects vary across process runs. The run range below is retained rather than treating a pooled median as a universal cost. Realtime/purchase workloads have not been benchmarked.

## Results

Times are pooled medians across 63 samples. Change is the median paired enum/pointer ratio minus one. Range is the minimum and maximum of the three per-process median changes. Positive means the enum was slower.

| Workload | Pointer ns/op | Enum ns/op | Paired change | Run change range |
|---|---:|---:|---:|---:|
| formulas_grouped | 5.76 | 7.13 | +23.5% | +22.7% to +23.8% |
| batches_grouped | 10.94 | 10.75 | -4.1% | -10.1% to +23.0% |
| ability_rows_grouped | 29.95 | 34.90 | +13.4% | +12.7% to +21.9% |
| calculator_grouped | 481.37 | 547.71 | +14.8% | +13.2% to +18.3% |
| formulas_shuffled | 6.91 | 7.82 | +13.6% | +12.2% to +14.3% |
| batches_shuffled | 11.45 | 10.82 | -8.8% | -13.0% to +24.3% |
| ability_rows_shuffled | 30.39 | 35.35 | +16.3% | +14.1% to +18.5% |
| calculator_shuffled | 488.63 | 554.49 | +14.8% | +13.2% to +19.0% |
| formulas_neeko | 1.93 | 2.18 | +15.1% | +10.7% to +16.4% |
| batches_neeko | 9.68 | 11.03 | +13.7% | +10.4% to +14.8% |
| ability_rows_neeko | 18.58 | 20.88 | +12.2% | -8.5% to +15.2% |
| calculator_neeko | 451.12 | 500.68 | +11.8% | +10.8% to +17.2% |
| formulas_leblanc | 1.80 | 2.24 | +24.5% | +22.8% to +25.0% |
| batches_leblanc | 12.63 | 13.88 | +9.3% | +6.9% to +9.9% |
| ability_rows_leblanc | 33.31 | 41.43 | +23.9% | +2.4% to +24.9% |
| calculator_leblanc | 469.21 | 521.59 | +10.7% | +9.8% to +11.1% |

`formulas`: ns per formula call; `batches`: ns per champion batch; `ability_rows`: ns per champion ability row; `calculator`: ns per complete calculator call.

The formula enum is 2 bytes and the batch enum 1 byte, versus 8-byte function pointers on this target. Smaller tables did not eliminate dispatch cost. Raw formula dispatch was slower in every measured scenario/run; batch and row behavior was less consistent, including wins for the enum in some runs. The experiment therefore has measurable runtime differences, but a blanket statement that every enum call is slower is unsupported.

Raw sample files are `dispatch-run-1.json`, `dispatch-run-2.json`, and `dispatch-run-3.json`; aggregate numbers, compiler information, and executable hash are in `summary.json`.

## Tuple validity

The standalone `../examples/closure_group_probe.rs` validates the proposed design without changing the library evaluator or generator:

- Macro implementations for tuple arities 0–18, with `[const] Fn(&Ctx) -> f32` bounds.
- On this compiler the implementation syntax is **`const impl`**, rather than `impl const`.
- Const/runtime agreement for empty, one-function, mixed-function/const-closure, captured const-closure, and 18-function groups.
- Direct function-item tuples occupy zero bytes; there are no pointer casts or `dyn Fn` objects.
- Actual Neeko and Leblanc function-item groups, selected through a runtime/const champion match, agree with the existing evaluator.
- The item/rune probe preserves two functions per metadata entry, damage modifiers, output order, and onhit updates. Abilities retain one function per metadata entry and ability modifiers.
- Invalid indices panic at runtime. With `--cfg invalid_index`, the compiler rejects the invalid constant with E0080 and the expected out-of-bounds panic.
- Groups and metadata are borrowed; modifiers are Copy, so the generic evaluator introduces no by-value generic destruction. Captures containing non-const destruction would need separate treatment.

**Integration constraint:** heterogeneous tuples are valid inputs to the generic evaluator, but cannot replace the uniform `Champion.closures` field directly. Generate a champion match whose arms invoke the evaluator with that champion's concrete tuple. Dispatch once before the group rather than once for every formula. Item/rune selection needs a similar per-ID/per-pair boundary, or a fully generic loadout. Do not erase groups into a slice or const trait object. Current champion groups fit arity 18; arbitrary runtime item/rune input slices are not intrinsically limited to 18 combined functions.

The tuple approach is compiler-validated, but has not replaced the library and its performance has not been measured.

## Reproduce

From the repository root:

```console
cargo +nightly build --offline --release --manifest-path nightly/Cargo.toml --no-default-features --features engine --example dispatch_benchmark -j 4
taskset -c 4 nightly/target/release/examples/dispatch_benchmark > /tmp/dispatch.json
cargo +nightly run --offline --manifest-path nightly/Cargo.toml --example closure_group_probe
```

Expected compile failure for an invalid constant index:

```console
cargo +nightly rustc --offline --manifest-path nightly/Cargo.toml --example closure_group_probe -- --cfg invalid_index
```

No library dispatch replacement or commits were made for this assessment.

## Follow-up tuple measurements

The subsequent three-way tuple/pointer/enum measurements, including comparisons
with optimized batches and a shared calculator prototype, are in [TUPLES.md](TUPLES.md).
