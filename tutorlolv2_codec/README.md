# formula-codec refactor

This directory splits the old monolithic `packer.rs` into a small bytecode crate plus an
API-specific adapter.

## Layout

```text
formula-codec/
├── Cargo.toml
├── src/
│   ├── lib.rs       # public surface/re-exports
│   ├── common.rs    # wire constants/opcodes + Error, EntityKind, DamageSlot
│   ├── build.rs     # source types, lexer/parser, AST, pools, FormulaDbBuilder, encoding
│   └── render.rs    # Class, FormulaDb reader, formula renderer, generic Highlighter
└── integration/
    └── tutorlolv2.rs # move this into tutorlolv2 or a sibling adapter crate
```

The core crate intentionally has no `ChampionId`, `ItemId`, `RuneId`, or `CastId` dependency.
The adapter owns all lookup/name logic tied to those types. Although `AbilityId` and `CtxVar`
would be acceptable dependencies, keeping them in the adapter avoids a dependency cycle if
`tutorlolv2` itself consumes `formula-codec`.

## Behavior changes

- Operators (`+`, `-`, `*`, `/`, `=>`, `..`, `..=`) and ordinary punctuation are raw HTML text,
  so there is no useless `<span>` around them.
- Parentheses/braces/brackets still use `Class::Bracket1/2/3`, since those classes already exist.
- `match` uses `Class::Control`.
- `as` uses `Class::Keyword` and `u8` uses `Class::Primitive`.
- `ctx.foo` is rendered as only `foo` (`Class::Variable`). The bytecode still stores the CtxVar id.
- Match layout is now preserved per formula occurrence. A multiline rustfmt match is reconstructed
  with one arm per line and with its captured arm/closing indentation.
- The wire format version is bumped to `2`; regenerate `packer.bin` after switching to this code.

## Why no bytemuck

The format is explicitly little-endian and contains packed 3-byte table entries. Mapping Rust
structs directly onto the buffer would introduce alignment/padding/endianness concerns for very
little benefit here. `to_le_bytes`/`from_le_bytes` keeps the wire format exact and portable.

## Integration notes

`integration/tutorlolv2.rs` is an adapter example, not part of the generic crate. It deliberately
uses `VALUES.iter().position(...)` for owner indices instead of relying on enum representation or
`CastId`. If your generated API already exposes stable `index()`/`from_repr()` methods directly on
`ChampionId`, `ItemId`, and `RuneId`, replace those three small lookup methods with the direct API.

`CtxVar::from_str` / `CtxVar::from_repr` are used in the adapter to avoid the previous unsafe
`transmute` path.
