# Nightly const calculator experiment

An independent copy of the root library, with a private generator fork in
`build/`. The stable source and generator are unchanged. The package is
`tutorlolv2_nightly`, its library is named `tutorlolv2`, and publishing is disabled.

## Run

The local `rust-toolchain.toml` selects nightly inside this directory:

```console
cd nightly
cargo test --workspace
cargo run --example const_calculator
cargo check --no-default-features
cargo test --no-default-features --features engine
```

From the repository root, use `cargo +nightly` and
`--manifest-path nightly/Cargo.toml`. This directory has its own workspace,
lockfile, and target directory. Source copies do not synchronize automatically.

Tested with rustc 1.102.0-nightly (ac8310ba6 2026-11-09). Nightly currently warns
that generic const expressions require falling back from its new trait solver.
Consumers using these array types also need `#![feature(generic_const_exprs)]`.

## Two calculators, one evaluator

`calculator(game)` returns the ordinary `OutputGame`, backed by boxes.
`calculator_array::<LAYOUT, ENEMIES>(game)` is a `const fn` returning
`ArrayOutputGame<LAYOUT, ENEMIES>`, backed by fixed arrays. Both accept the same
borrowed `InputGame` and use the same stat inference, enemy preparation, formula
calls, and slice-writing evaluation. The boxed wrapper only handles allocation.

For a constant input named `GAME`:

```rust
const LAYOUT: DamageLayout = calculator_layout(GAME);
const OUTPUT: ArrayOutputGame<LAYOUT, { GAME.enemy_players.len() }> =
    calculator_array::<LAYOUT, { GAME.enemy_players.len() }>(GAME);
```

Or infer the dimensions and force compile-time evaluation with:

```rust
let output = tutorlolv2::calculator_array!(GAME);
```

See `examples/const_calculator.rs` for a complete, runnable input and comparison.
The function also accepts runtime inputs when their output dimensions are known.
It checks layout and enemy count before writing any output.

`calculator_layout()` deduplicates items/runes and filters non-damaging entries
exactly as the boxed calculator does. Item/rune widths include their min/max
pairs. Values occupy `(L_MSTR + ENEMIES) * LAYOUT.stride()` elements; array lengths
come from const generic expressions rather than independently supplied lengths.

Both outputs include the current player, enemies, turret damage, item/rune
metadata, and damage rows for monsters and enemies. `ArrayOutputGame` exposes
`enemy_damages()` and `monster_damages()` too. The old separate `const_eval`
module, `ConstInput`, `ConstOutput`, and their simulated-purchase output are gone.

## Const-callable formula tables

Ordinary `fn` pointers are still not called in constant evaluation. Instead,
`Closure` and `AbilityBatch` are generated, payload-free enums implementing
nightly const `Fn`, `FnMut`, and `FnOnce`. Each variant dispatches directly to
its original named const function. Tables are homogeneous and retain ordinary
call syntax, such as `(data.closures[i])(&ctx)`.

This is concrete enum dispatch, not TAIT hiding heterogeneous function types.
The generated ID-specific const dispatch functions and `ChampionId::eval`,
`ItemId::eval`, and `RuneId::eval` are removed. There is one callable dispatch
representation used at compile time and runtime. Named formula functions and
optimized champion batches remain.

The experiment enables `const_trait_impl`, `fn_traits`, `unboxed_closures`,
`const_index`, `generic_const_exprs`, and `adt_const_params`. `DamageLayout` is a
structural const generic parameter. No compiler-internal intrinsics are needed.

The const evaluation methods are `eval`, `eval_abilities`, `fill`, `fill_batch`,
`fill_with_batch`, and `fill_monster_damages` on `DamageTableMut`; the existing
boxed-table evaluation wrappers are const too. `DamageEvalDataRef` supplies
borrowed metadata/callables without requiring boxed construction. Player and
enemy preparation and the shared calculator loop are const. Allocation and the
legacy boxed enemy-collection/callback methods remain runtime operations.

## Generator isolation and validation

`build/` is the separate `tutorlolv2_nightly_build` crate. It reuses sibling
codec/types/wiki crates and repository `cache/`/`internal/` inputs, but all
modified generator source is local. After generating formula modules, it reads
the static callable fields, replaces pointer entries with enum variants, and
emits the const `Fn` implementations into `OUT_DIR/callables.rs`.

The original root library is a dev-only dependency used as an independent
oracle. Tests compare every generated champion/item/rune callable against the
original pointer tables, and full boxed calculator rows against the original
calculator for every champion. Const array outputs are compared with runtime
boxed outputs for items/runes, duplicate and non-damaging loadouts, Mega Gnar,
empty enemy lists, multiple enemies, map effects, and explicit/inferred stats.
Incorrect dimensions are rejected. Existing calculation and realtime regression
tests are retained and adapted to callable-table access.

This establishes functional equivalence for those cases. Dispatch performance
measurements are recorded below; realtime/purchase performance remains unmeasured.

## Dispatch assessment

Runtime enum-versus-pointer measurements and the standalone heterogeneous-tuple
const-trait probe are documented in [benchmarks/README.md](benchmarks/README.md).
The probe validates an alternative without replacing the library's enum dispatch.

Tuple performance measurements are available in [benchmarks/TUPLES.md](benchmarks/TUPLES.md),
with the isolated benchmark crate in `tuple-bench/`.
