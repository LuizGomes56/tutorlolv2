# Calculator performance harness

Run from the repository root (only the Rust toolchain is required):

```bash
cargo perf --enemies 5 --output perf/results/baseline.json
```

This builds with the root `perf` profile: optimization level 3, fat LTO,
one codegen unit, abort-on-panic, no incremental compilation, no debug assertions
or overflow checks, and stripped symbols. The launcher additionally uses
`-C target-cpu=native`. Use `--portable` to omit that flag. These are aggressive
stable settings, not a guarantee that every option improves every workload.
Profile settings live at the workspace root, where Cargo applies them.
See [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
and [rustc code generation options](https://doc.rust-lang.org/rustc/codegen-options/index.html#target-cpu).

By default, the first run creates `perf/fixtures/n5-g32-s42-mixed.json`: 32 seeded
games with five enemies each. Later runs reuse the same JSON. Champion, runes,
and inventory stay fixed within each game; the loop cycles through the corpus.
Use `--games 1` for repeated calls on one game. Generated inventories have zero
to six distinct purchasable Summoner's Rift items; rune pages and builds are
synthetic and do not enforce shop exclusivity or rune-tree compatibility.

Only the `calculator` call is timed. Deserialization, random generation,
building borrowed inputs, input selection, warmup, validation, returned-output
destruction, compilation, and file writing are outside the timed intervals.
Allocations performed **inside** `calculator` are part of its runtime.
Each call is timed separately to exclude destruction, so timings include a
small clock/`black_box` overhead. The report measures empty clock pairs as a
reference and does not subtract a noisy estimate from results.

Defaults: 2,000 warmup calls, 20 measured samples of 20,000 calls each
(400,000 measured calls). JSON contains each sample's mean ns/call, overall
mean/median/min/max, standard deviation, throughput, and p95 **of sample means**.
That p95 is not individual-call tail latency. Results also record workload and
output fingerprints, executable fingerprint, compiler/flags, CPU, timestamp,
and Git state. An existing result file is never overwritten.

## Generate, replay, and compare

```bash
# Generate a fixture without measuring it.
cargo perf --enemies 10 --games 64 --seed 123 --generate-only

# Explicit stats bypass inference; inferred stats exercise it; mixed covers both.
cargo perf --enemies 5 --stats inferred --output perf/results/inferred.json
cargo perf --enemies 5 --stats explicit --output perf/results/explicit.json

# Replay the exact baseline input after an optimization.
cargo perf --input perf/fixtures/n5-g32-s42-mixed.json --output perf/results/after.json

# Short smoke run.
cargo perf --enemies 3 --games 4 --iterations 100 --samples 3 --warmup 10

# Test the harness and generator without a long performance run.
cargo test -p tutorlolv2_perf
```

`--enemies`, `--games`, `--seed`, and `--stats` control generation only. When
`--input` already exists, its contents and metadata control the workload. Use
`--regenerate` to explicitly replace a fixture. Zero enemies are supported;
zero games, iterations, or samples are rejected. Without `--output`, results
receive a unique timestamped path under `perf/results/`.

For comparisons, reuse the same JSON, compiler, flags, CPU, sample settings,
and an otherwise idle machine. Compare `measurement.summary.mean_ns_per_call`
and raw sample variation. Improvement percentage is
`100 * (baseline_mean - after_mean) / baseline_mean`.
Matching input fingerprints establish the same input; matching output
fingerprints help check unchanged behavior. Fingerprints are non-cryptographic
FNV-1a identifiers. Repeat runs to distinguish a small speedup from scheduling
or CPU-frequency noise. This harness uses the normal system allocator.

The first optimized/native build can take substantially longer than the actual
measurement. `--portable` retains any caller-supplied Rust flags. Generated
fixtures/results are ignored by Git; source code and this README are not.

## Realtime function

```bash
cargo perf --realtime --enemies 5 --output perf/results/realtime-before.json
cargo perf --realtime --input perf/fixtures/realtime-n5-a4-g32-s42.json --output perf/results/realtime-after.json
cargo test -p tutorlolv2_perf --test realtime
```

The separate `realtime` binary uses the same native build profile, output
fingerprints, timing boundaries, and statistics as the calculator harness.
Defaults are 32 seeded games, four allies excluding the active player, 200
warmup calls, and 20 samples of 500 calls each (10,000 measured calls). Realtime
evaluates every `ItemId::SIML` candidate against every enemy; the report records
that candidate count. It therefore does substantially more work than calculator.

Each JSON game deserializes directly to the production `RiotRealtime` type,
borrowing strings from the input buffer. Parsing and validation are outside
timing, as is destruction of the returned `Result` and its owned output buffers.
Allocations made inside `realtime` remain part of its runtime.

Synthetic fields follow the current library's conventions: matching Riot IDs,
`ORDER`/`CHAOS` teams, recognized names and positions, numeric Riot item/rune IDs,
positive elapsed seconds, map code 11, optional rune arrays, and dragon events
whose killers match player IDs. Percent penetration is encoded as the remaining
fraction and converted by `PlayerStats::base100`. Current player stats are
observed totals; enemies have only levels/items. There are 96 irrelevant events
per game plus relevant dragon kills. The corpus alternates the active team and
occasionally omits rune data. Builds/rune pages are synthetic as described above.

New fixtures include `rawSkinName` and `rawChampionName`, including the `GnarBig`
display key for Mega Gnar, so production champion and form inference sees realistic
input. Existing fixtures remain authoritative: use `--regenerate` to refresh old
fixtures, then establish a new baseline before comparing performance or outputs.

Use `--allies N` to change the ally count (excluding self). Five enemies and four
allies give the usual ten-player shape; higher enemy counts are stress tests,
not claims about real match sizes. Existing JSON is always authoritative, so
replay exactly the same fixture for before/after comparisons. Do not compare
realtime timings directly to calculator timings as equal workloads.
