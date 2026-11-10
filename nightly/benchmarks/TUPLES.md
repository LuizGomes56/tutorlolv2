# Heterogeneous tuple dispatch performance

Measured on 2026-11-10 with rustc 1.102.0-nightly (ac8310ba6, LLVM 23.1.5), AMD Ryzen 9 8940HX, WSL2, process affinity CPU 4. Three independent runs, 21 samples per strategy per workload per run. All strategies run in the same release executable with opt-level 3, fat LTO, one codegen unit, panic abort, and only `engine` enabled in the library.

## What was measured

The standalone `../tuple-bench/` crate generates concrete function-item tuples for every champion and every melee/ranged item/rune pair. It implements the proposed const `ClosureGroup` for tuple arities 0–18. No tuple elements are cast to pointers and no groups are stored as trait objects. Runtime ID selection happens outside the generic tuple evaluator.

Five strategies are measured:

- `pointers`: conventional per-function pointer slices, using the original evaluator loop and bounds-check behavior.
- `enums`: current per-function const-callable enum slices and the library's evaluator.
- `tuples`: concrete heterogeneous tuples and the generic metadata-driven evaluator.
- `pointer_batches`: the existing optimized generated champion batch functions, invoked through ordinary pointers.
- `enum_batches`: those same optimized batches, invoked through the current batch enum.

Pointer controls refer to the exact same named formula/batch functions as the tuple and enum cases. This avoids comparing different formula implementations or compiler versions. Mode selection is outside the timed kernel; each strategy has its own kernel monomorphization. Runtime champion/item/rune selection is included in timing.

Raw calls evaluate whole champion groups, rather than isolated individual functions. Item/rune pair evaluation includes both damage variants, modifiers, and onhit updates. Ability-row evaluation includes metadata/modifier/onhit processing.

**Full calculator numbers are an isolated prototype, not a migration of the library.** The benchmark snapshots the current stat and enemy preparation and uses one generic calculator implementation with identical allocation/storage for all five strategies. Tuple champion groups are selected once before that calculator processes its rows; item/rune pairs are selected per ID as rows are evaluated. All modes retain the same auxiliary metadata/callable allocations, including allocations a future tuple implementation could eliminate. Full-calculator results therefore describe this prototype and its monomorphization/code-layout costs, not every possible tuple integration.

Generated optimized batches already encode direct calls and metadata-specific operations. A metadata-driven tuple loop does not automatically replace that optimization.

## Grouped workload results

All 173 champions in enum order. Times are pooled medians of 63 samples. Units: ns per whole champion group or ability row, ns per item/rune pair, and ns per complete calculator call. These are new contemporaneous controls; absolute values should not be compared directly with the previous benchmark executable.

| Workload | Pointer loops | Enum loops | Tuples | Pointer batches | Enum batches |
|---|---:|---:|---:|---:|---:|
| raw_groups | 15.81 | 18.90 | 8.45 | — | — |
| ability_rows | 29.56 | 82.60 | 25.63 | 13.61 | 10.17 |
| item_rune_pairs | 9.04 | 8.97 | 7.25 | — | — |
| calculator | 859.62 | 999.89 | 892.39 | 537.08 | 576.58 |

## Tuple changes across all scenarios

Changes use paired sample ratios. Positive means tuples were slower. The range is across the three process-run median ratios; it is not a confidence interval.

| Scenario | Workload | Tuple vs pointer loops | Run range | Tuple vs enum loops | Tuple vs optimized pointer batches |
|---|---|---:|---:|---:|---:|
| grouped | raw_groups | -46.6% | -46.7% to -46.5% | -55.3% | — |
| grouped | ability_rows | -13.5% | -14.2% to -12.9% | -68.9% | +86.0% |
| grouped | item_rune_pairs | -19.4% | -19.5% to -19.3% | -19.5% | — |
| grouped | calculator | +4.2% | +3.1% to +5.5% | -10.7% | +67.0% |
| shuffled | raw_groups | -46.3% | -46.5% to -46.0% | -55.2% | — |
| shuffled | ability_rows | -11.4% | -11.9% to -10.2% | -69.9% | +92.6% |
| shuffled | item_rune_pairs | -18.9% | -19.5% to -18.6% | -19.9% | — |
| shuffled | calculator | +4.7% | +4.1% to +6.0% | -8.1% | +68.7% |
| neeko | raw_groups | -43.7% | -44.0% to -43.4% | -48.2% | — |
| neeko | ability_rows | -4.3% | -4.4% to -4.1% | -44.1% | +96.7% |
| neeko | calculator | -5.8% | -6.5% to -2.1% | -10.8% | +14.2% |
| leblanc | raw_groups | -62.9% | -62.9% to -62.9% | -69.3% | — |
| leblanc | ability_rows | -17.6% | -17.8% to -17.2% | -49.7% | +147.4% |
| leblanc | calculator | -0.6% | -0.9% to +1.6% | -12.0% | +45.5% |

Tuples substantially reduce raw grouped-call overhead and improve generic ability/item/rune evaluation relative to pointer or enum loops in these measurements. They do **not** beat the existing optimized batch path. In the complete-calculator prototype, pointer batches remain fastest; tuples are about 67% slower for the grouped all-champion workload, about 14% slower for fixed Neeko, and about 46% slower for fixed Leblanc. The unbatched all-champion tuple prototype also does not outperform pointer loops end to end, despite winning the evaluator microbenchmarks. Treat this as an integration/code-generation warning rather than a claim that tuples must always have that full-calculator cost.

A useful next implementation direction is tuple support for generic const evaluation while retaining generated optimized batches for the runtime fast path. A tuple-specific optimizer that exposes constant metadata and avoids duplicating preparation could change the full-calculator result; that has not been implemented or measured here.

## Validation and controls

- Every champion tuple is evaluated at compile time; runtime tuple evaluation produces the same aggregate checksum.
- Before each run, every champion, item, and rune is compared across strategies using two contexts, both attack types, non-unit modifiers, and onhit outputs.
- All five complete-calculator strategies match the current library's player/enemy metadata, tower damages, contexts, and every damage row for every champion on two maps.
- Workload construction and validation are outside timing. `black_box` prevents precomputation or elimination of outputs.
- Each sample uses at least 20 ms of calibrated pointer-loop work. Strategy order rotates and reverses across 21 rounds. No samples are discarded.
- Ordered and deterministically shuffled all-champion requests, plus repeated Neeko and Leblanc requests, are included. Shuffled item/rune cases shuffle pair IDs as well.
- Local wall-clock/hypervisor, CPU frequency, branch prediction, instruction cache, code layout, and generic monomorphization effects remain relevant. No realtime/purchase benchmark or hardware-counter diagnosis is claimed.

## Files and reproduction

Sources: `../tuple-bench/src/groups.rs`, `../tuple-bench/src/engine.rs`, and `../tuple-bench/src/main.rs`. The separate benchmark-only `../tuple-bench/build.rs` reuses the existing generator and emits function-item tuple arms into its own `OUT_DIR`. Library source and the library generator were not modified.

Raw samples: `tuples-run-1.json`, `tuples-run-2.json`, `tuples-run-3.json`. Aggregates, per-run ranges, exact compiler, and executable hash: `tuples-summary.json`.

From the repository root:

```console
cargo +nightly build --offline --release --manifest-path nightly/tuple-bench/Cargo.toml --target-dir nightly/target -j 4
nightly/target/release/tutorlolv2_tuple_bench --validate-only
taskset -c 4 nightly/target/release/tutorlolv2_tuple_bench > /tmp/tuples.json
```

The parent toolchain file currently tracks floating nightly; the exact measured commit is recorded. No commits were made.
