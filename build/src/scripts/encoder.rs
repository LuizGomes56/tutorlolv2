//! Compact formula bytecode for champion/item/rune damage expressions.
//!
//! Design goals:
//! - std-only: no serde/bincode/syn dependency.
//! - Formulas are never evaluated; bytecode exists only to reconstruct highlighted text.
//! - `ctx.foo` is encoded as one byte (`CtxVar` supplied by the caller).
//! - Local function calls are encoded as `RefLocal(u8)` using a build-time HashMap<String, u8>.
//! - Matches are deduplicated into a global match table.
//! - Frequently repeated non-small f32 constants are deduplicated into a global constant pool.
//! - Binary operators encode rustfmt line-break information directly in their opcode.
//! - Champions use dense local ability indices.
//! - Items and runes use logical `DamageSlot`s but physically store only formulas that exist.
//!
//! The on-disk/in-memory format is custom and little-endian.

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt;

pub const MAGIC: [u8; 4] = *b"FBC1";
pub const VERSION: u8 = 1;
const HEADER_LEN: usize = 24;

// Number opcodes.
const OP_NUM_U8: u8 = 0x00;
const OP_NUM_I8: u8 = 0x01;
const OP_NUM_CONST: u8 = 0x02;
const OP_NUM_F32: u8 = 0x03;

// Leaf/structural opcodes.
const OP_CTX: u8 = 0x10;
const OP_REF_LOCAL: u8 = 0x11;
const OP_MATCH_REF: u8 = 0x12;
const OP_GROUP: u8 = 0x13;
const OP_NEG: u8 = 0x14;

// 0x20..=0x2b are binary operators.
// low 2 bits: operator  (0 +, 1 -, 2 *, 3 /)
// bits 2..3: break kind (0 inline, 1 before op, 2 after op)
const OP_BIN_BASE: u8 = 0x20;
const OP_BIN_MAX: u8 = 0x2b;

#[derive(Debug, Clone)]
pub enum Error {
    Parse {
        line: u32,
        column: u32,
        message: String,
    },
    UnknownCtxVar(String),
    UnknownFunction(String),
    OwnerOutOfRange {
        kind: EntityKind,
        owner: u16,
        count: u16,
    },
    DuplicateOwner {
        kind: EntityKind,
        owner: u16,
    },
    DuplicateLocal(u8),
    ChampionLocalsMustBeDense,
    InvalidSlotCombination(&'static str),
    TooManyMatches,
    TooManyMatchArms,
    TooManyFormulas,
    TooManyOwners,
    InvalidIndent {
        line: u32,
        column: u32,
    },
    Corrupt(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parse {
                line,
                column,
                message,
            } => {
                write!(f, "parse error at {line}:{column}: {message}")
            }
            Error::UnknownCtxVar(v) => write!(f, "unknown ctx variable: {v}"),
            Error::UnknownFunction(v) => write!(f, "unknown local function reference: {v}"),
            Error::OwnerOutOfRange { kind, owner, count } => {
                write!(
                    f,
                    "{kind:?} owner {owner} is outside table of {count} owners"
                )
            }
            Error::DuplicateOwner { kind, owner } => {
                write!(f, "{kind:?} owner {owner} was inserted twice")
            }
            Error::DuplicateLocal(local) => write!(f, "duplicate local formula index {local}"),
            Error::ChampionLocalsMustBeDense => {
                write!(
                    f,
                    "champion local formula indices must be 0..len with no gaps"
                )
            }
            Error::InvalidSlotCombination(msg) => write!(f, "invalid damage slots: {msg}"),
            Error::TooManyMatches => write!(f, "more than 256 unique matches"),
            Error::TooManyMatchArms => write!(f, "a match has more than 255 arms"),
            Error::TooManyFormulas => write!(f, "more than 65535 formulas"),
            Error::TooManyOwners => write!(f, "an owner table exceeds u16::MAX"),
            Error::InvalidIndent { line, column } => write!(
                f,
                "line-break indentation at {line}:{column} is not an even number of spaces or exceeds 510 spaces"
            ),
            Error::Corrupt(msg) => write!(f, "corrupt formula database: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    Champion,
    Item,
    Rune,
}

/// Logical formula slot used by both Items and Runes.
///
/// Physical storage is sparse; this value NEVER means "the Nth physically stored formula".
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DamageSlot {
    MeleeMin = 0,
    MeleeMax = 1,
    RangedMin = 2,
    RangedMax = 3,
}

impl DamageSlot {
    pub const ALL: [Self; 4] = [
        Self::MeleeMin,
        Self::MeleeMax,
        Self::RangedMin,
        Self::RangedMax,
    ];

    #[inline]
    pub const fn bit(self) -> u8 {
        1 << self as u8
    }
}

/// One build-time formula. For champions, `local` must be dense 0..len.
#[derive(Debug, Clone)]
pub struct FormulaSource {
    pub local: u8,
    pub source: String,
}

/// Build-time conversion used for `ctx.foo` -> one byte.
///
/// A wrapper around your existing `CtxVar::from_str` is ideal:
///
/// ```ignore
/// fn ctx_id(s: &str) -> Option<u8> {
///     CtxVar::from_str(s).map(|v| v as u8)
/// }
/// ```
pub type CtxResolver = fn(&str) -> Option<u8>;

#[derive(Debug, Clone)]
enum Ast {
    Number(u32), // f32::to_bits
    Ctx(u8),
    RefLocal(u8),
    Binary {
        op: BinaryOp,
        layout: OpLayout,
        left: Box<Ast>,
        right: Box<Ast>,
    },
    Group(Box<Ast>),
    Neg(Box<Ast>),
    Match(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BinaryOp {
    Add = 0,
    Sub = 1,
    Mul = 2,
    Div = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BreakKind {
    Inline = 0,
    BeforeOperator = 1,
    AfterOperator = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OpLayout {
    kind: BreakKind,
    /// Number of 2-space units. Ignored for Inline.
    indent_units: u8,
}

impl OpLayout {
    const INLINE: Self = Self {
        kind: BreakKind::Inline,
        indent_units: 0,
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MatchDef {
    ctx: u8,
    cast_u8: bool,
    arms: Vec<MatchArm>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MatchArm {
    pattern: MatchPattern,
    value: u32, // f32 bits
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MatchPattern {
    Exact(u8),
    ToExclusive(u8),
    ToInclusive(u8),
    RangeExclusive(u8, u8),
    RangeInclusive(u8, u8),
    From(u8),
}

#[derive(Clone, Debug)]
struct ChampionPending {
    formulas: Vec<Ast>,
}

#[derive(Clone, Debug)]
struct SparsePending {
    /// Sorted by logical DamageSlot value.
    formulas: Vec<(DamageSlot, Ast)>,
    mask: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct BuilderStats {
    pub champions_present: usize,
    pub champion_formulas: usize,
    pub items_present: usize,
    pub item_formulas: usize,
    pub runes_present: usize,
    pub rune_formulas: usize,
}

/// Build-time database builder.
///
/// `item_owner_count` and `rune_owner_count` describe the OWNER INDEX TABLE,
/// not the number of owners that have damage. An owner with no damage costs only
/// one 3-byte table entry and zero formula bytes.
#[derive(Clone)]
pub struct FormulaDbBuilder {
    champions: Vec<Option<ChampionPending>>,
    items: Vec<Option<SparsePending>>,
    runes: Vec<Option<SparsePending>>,
    ctx_resolver: CtxResolver,
    match_defs: Vec<MatchDef>,
    match_ids: HashMap<MatchDef, u8>,
}

impl FormulaDbBuilder {
    pub fn new(
        champion_owner_count: u16,
        item_owner_count: u16,
        rune_owner_count: u16,
        ctx_resolver: CtxResolver,
    ) -> Self {
        Self {
            champions: (0..champion_owner_count).map(|_| None).collect(),
            items: (0..item_owner_count).map(|_| None).collect(),
            runes: (0..rune_owner_count).map(|_| None).collect(),
            ctx_resolver,
            match_defs: Vec::new(),
            match_ids: HashMap::new(),
        }
    }

    pub fn stats(&self) -> BuilderStats {
        BuilderStats {
            champions_present: self.champions.iter().filter(|v| v.is_some()).count(),

            champion_formulas: self
                .champions
                .iter()
                .filter_map(Option::as_ref)
                .map(|v| v.formulas.len())
                .sum(),

            items_present: self.items.iter().filter(|v| v.is_some()).count(),

            item_formulas: self
                .items
                .iter()
                .filter_map(Option::as_ref)
                .map(|v| v.formulas.len())
                .sum(),

            runes_present: self.runes.iter().filter(|v| v.is_some()).count(),

            rune_formulas: self
                .runes
                .iter()
                .filter_map(Option::as_ref)
                .map(|v| v.formulas.len())
                .sum(),
        }
    }

    /// Adds one champion.
    ///
    /// `refs` is the per-champion HashMap<String, u8> you described. Function calls
    /// such as `gnar_qmin(ctx)` are replaced by `RefLocal(refs["gnar_qmin"])`.
    pub fn push_champion(
        &mut self,
        owner_index: u16,
        formulas: impl Iterator<Item = FormulaSource>,
        refs: &HashMap<String, u8>,
    ) -> Result<(), Error> {
        let slot = self
            .champions
            .get(owner_index as usize)
            .ok_or(Error::OwnerOutOfRange {
                kind: EntityKind::Champion,
                owner: owner_index,
                count: self.champions.len() as u16,
            })?;
        if slot.is_some() {
            return Err(Error::DuplicateOwner {
                kind: EntityKind::Champion,
                owner: owner_index,
            });
        }

        let mut parsed = Vec::new();
        for f in formulas {
            let ast = self.parse_formula(&f.source, refs)?;
            parsed.push((f.local, ast));
        }
        parsed.sort_by_key(|(local, _)| *local);

        for (i, (local, _)) in parsed.iter().enumerate() {
            if *local as usize != i {
                return Err(Error::ChampionLocalsMustBeDense);
            }
        }

        self.champions[owner_index as usize] = Some(ChampionPending {
            formulas: parsed.into_iter().map(|(_, ast)| ast).collect(),
        });
        Ok(())
    }

    pub fn push_item_or_rune(
        &mut self,
        kind: EntityKind,
        owner_index: u16,
        formulas: impl Iterator<Item = FormulaSource>,
        refs: &HashMap<String, u8>,
    ) -> Result<(), Error> {
        let table_len = match kind {
            EntityKind::Item => self.items.len(),
            EntityKind::Rune => self.runes.len(),
            EntityKind::Champion => unreachable!(),
        };
        if owner_index as usize >= table_len {
            return Err(Error::OwnerOutOfRange {
                kind,
                owner: owner_index,
                count: table_len as u16,
            });
        }

        let already = match kind {
            EntityKind::Item => self.items[owner_index as usize].is_some(),
            EntityKind::Rune => self.runes[owner_index as usize].is_some(),
            EntityKind::Champion => unreachable!(),
        };
        if already {
            return Err(Error::DuplicateOwner {
                kind,
                owner: owner_index,
            });
        }

        let mut seen = [false; 4];
        let mut parsed = Vec::new();
        let mut mask = 0u8;
        for f in formulas {
            let local = f.local as usize;
            if seen[local] {
                return Err(Error::DuplicateLocal(f.local as u8));
            }
            seen[local] = true;
            let slot = DamageSlot::ALL[f.local as usize];
            mask |= slot.bit();
            parsed.push((slot, self.parse_formula(&f.source, refs)?));
        }
        parsed.sort_by_key(|(slot, _)| *slot as u8);
        validate_sparse_mask(mask)?;

        let pending = SparsePending {
            formulas: parsed,
            mask,
        };
        match kind {
            EntityKind::Item => self.items[owner_index as usize] = Some(pending),
            EntityKind::Rune => self.runes[owner_index as usize] = Some(pending),
            EntityKind::Champion => unreachable!(),
        }
        Ok(())
    }

    fn parse_formula(&mut self, source: &str, refs: &HashMap<String, u8>) -> Result<Ast, Error> {
        let tokens = tokenize(source)?;
        let mut parser = Parser {
            tokens,
            pos: 0,
            ctx_resolver: self.ctx_resolver,
            refs,
            match_defs: &mut self.match_defs,
            match_ids: &mut self.match_ids,
        };
        let expr = parser.parse_expression(0)?;
        if !parser.is_eof() {
            let t = parser.peek().unwrap();
            return Err(parse_err(t, "unexpected token after end of formula"));
        }
        Ok(expr.ast)
    }

    /// Finalizes all pools/tables and returns the exact custom binary representation.
    pub fn finish(self) -> Result<Vec<u8>, Error> {
        let champion_count =
            u16::try_from(self.champions.len()).map_err(|_| Error::TooManyOwners)?;
        let item_count = u16::try_from(self.items.len()).map_err(|_| Error::TooManyOwners)?;
        let rune_count = u16::try_from(self.runes.len()).map_err(|_| Error::TooManyOwners)?;

        let mut formulas = Vec::<Ast>::new();
        let mut champion_bases = Vec::with_capacity(self.champions.len());

        for pending in self.champions {
            champion_bases.push(as_u16_formula_index(formulas.len())?);
            if let Some(p) = pending {
                formulas.extend(p.formulas);
            }
        }

        let mut item_entries = Vec::with_capacity(self.items.len());
        for pending in self.items {
            let first = as_u16_formula_index(formulas.len())?;
            let mask = pending.as_ref().map_or(0, |p| p.mask);
            item_entries.push(EntityEntry {
                first_formula: first,
                mask,
            });
            if let Some(p) = pending {
                for (_, ast) in p.formulas {
                    formulas.push(ast);
                }
            }
        }

        let mut rune_entries = Vec::with_capacity(self.runes.len());
        for pending in self.runes {
            let first = as_u16_formula_index(formulas.len())?;
            let mask = pending.as_ref().map_or(0, |p| p.mask);
            rune_entries.push(EntityEntry {
                first_formula: first,
                mask,
            });
            if let Some(p) = pending {
                for (_, ast) in p.formulas {
                    formulas.push(ast);
                }
            }
        }

        let formula_count = as_u16_formula_index(formulas.len())?;
        let match_count =
            u16::try_from(self.match_defs.len()).map_err(|_| Error::TooManyMatches)?;

        // Count numbers as they will actually be stored: every formula AST + every UNIQUE match.
        let mut freq = HashMap::<u32, u32>::new();
        for ast in &formulas {
            count_ast_numbers(ast, &mut freq);
        }
        for m in &self.match_defs {
            for arm in &m.arms {
                *freq.entry(arm.value).or_default() += 1;
            }
        }

        let (constant_pool, constant_ids) = choose_constant_pool(freq);
        let constant_count = constant_pool.len() as u16;

        let mut match_data = Vec::new();
        let mut match_offsets = Vec::with_capacity(self.match_defs.len() + 1);
        for m in &self.match_defs {
            match_offsets.push(match_data.len() as u32);
            encode_match(m, &constant_ids, &mut match_data)?;
        }
        match_offsets.push(match_data.len() as u32);

        let mut formula_data = Vec::new();
        let mut formula_offsets = Vec::with_capacity(formulas.len() + 1);
        for ast in &formulas {
            formula_offsets.push(formula_data.len() as u32);
            encode_ast(ast, &constant_ids, &mut formula_data);
        }
        formula_offsets.push(formula_data.len() as u32);

        let match_data_len = u32::try_from(match_data.len())
            .map_err(|_| Error::Corrupt("match data exceeds u32"))?;

        let mut out = Vec::with_capacity(
            HEADER_LEN
                + constant_pool.len() * 4
                + match_offsets.len() * 4
                + champion_bases.len() * 2
                + item_entries.len() * 3
                + rune_entries.len() * 3
                + formula_offsets.len() * 4
                + match_data.len()
                + formula_data.len(),
        );

        // Header, exactly 24 bytes.
        out.extend_from_slice(&MAGIC);
        out.push(VERSION);
        out.push(0); // flags/reserved
        push_u16(&mut out, HEADER_LEN as u16);
        push_u16(&mut out, constant_count);
        push_u16(&mut out, match_count);
        push_u16(&mut out, champion_count);
        push_u16(&mut out, item_count);
        push_u16(&mut out, rune_count);
        push_u16(&mut out, formula_count);
        push_u32(&mut out, match_data_len);
        debug_assert_eq!(out.len(), HEADER_LEN);

        for bits in constant_pool {
            push_u32(&mut out, bits);
        }
        for offset in match_offsets {
            push_u32(&mut out, offset);
        }
        for base in champion_bases {
            push_u16(&mut out, base);
        }
        for entry in item_entries {
            push_u16(&mut out, entry.first_formula);
            out.push(entry.mask);
        }
        for entry in rune_entries {
            push_u16(&mut out, entry.first_formula);
            out.push(entry.mask);
        }
        for offset in formula_offsets {
            push_u32(&mut out, offset);
        }
        out.extend_from_slice(&match_data);
        out.extend_from_slice(&formula_data);

        Ok(out)
    }
}

fn validate_sparse_mask(mask: u8) -> Result<(), Error> {
    let melee_min = mask & DamageSlot::MeleeMin.bit() != 0;
    let melee_max = mask & DamageSlot::MeleeMax.bit() != 0;
    let ranged_min = mask & DamageSlot::RangedMin.bit() != 0;
    let ranged_max = mask & DamageSlot::RangedMax.bit() != 0;

    if melee_max && !melee_min {
        return Err(Error::InvalidSlotCombination(
            "MeleeMax cannot exist without MeleeMin",
        ));
    }
    if ranged_max && !ranged_min {
        return Err(Error::InvalidSlotCombination(
            "RangedMax cannot exist without RangedMin",
        ));
    }
    if (melee_min || melee_max) && !ranged_min {
        return Err(Error::InvalidSlotCombination(
            "melee damage requires ranged damage for context",
        ));
    }
    Ok(())
}

fn as_u16_formula_index(len: usize) -> Result<u16, Error> {
    u16::try_from(len).map_err(|_| Error::TooManyFormulas)
}

#[derive(Debug, Clone, Copy)]
struct EntityEntry {
    first_formula: u16,
    mask: u8,
}

fn count_ast_numbers(ast: &Ast, freq: &mut HashMap<u32, u32>) {
    match ast {
        Ast::Number(bits) => *freq.entry(*bits).or_default() += 1,
        Ast::Binary { left, right, .. } => {
            count_ast_numbers(left, freq);
            count_ast_numbers(right, freq);
        }
        Ast::Group(inner) | Ast::Neg(inner) => count_ast_numbers(inner, freq),
        Ast::Ctx(_) | Ast::RefLocal(_) | Ast::Match(_) => {}
    }
}

/// Repeated values only enter the pool when a 2-byte ConstRef is cheaper than a 5-byte inline f32.
/// Small signed/unsigned immediates are never pooled because they already cost 2 bytes.
fn choose_constant_pool(freq: HashMap<u32, u32>) -> (Vec<u32>, HashMap<u32, u8>) {
    let mut candidates = freq
        .into_iter()
        .filter(|(bits, count)| *count >= 2 && immediate_number(*bits).is_none())
        .map(|(bits, count)| {
            // Inline: 5*n. Pooled: 2*n + 4. Saving = 3*n - 4.
            let saving = count.saturating_mul(3).saturating_sub(4);
            (saving, count, bits)
        })
        .collect::<Vec<_>>();

    // Deterministic and most-profitable first.
    candidates.sort_by(|a, b| {
        b.0.cmp(&a.0)
            .then_with(|| b.1.cmp(&a.1))
            .then_with(|| a.2.cmp(&b.2))
    });
    candidates.truncate(256);

    let pool = candidates
        .iter()
        .map(|(_, _, bits)| *bits)
        .collect::<Vec<_>>();
    let ids = pool
        .iter()
        .enumerate()
        .map(|(i, bits)| (*bits, i as u8))
        .collect::<HashMap<_, _>>();
    (pool, ids)
}

#[derive(Debug, Clone, Copy)]
enum ImmediateNumber {
    U8(u8),
    I8(i8),
}

fn immediate_number(bits: u32) -> Option<ImmediateNumber> {
    let v = f32::from_bits(bits);
    if !v.is_finite() || v.fract() != 0.0 {
        return None;
    }
    if (0.0..=255.0).contains(&v) {
        return Some(ImmediateNumber::U8(v as u8));
    }
    if (-128.0..0.0).contains(&v) {
        return Some(ImmediateNumber::I8(v as i8));
    }
    None
}

fn encode_number(bits: u32, constants: &HashMap<u32, u8>, out: &mut Vec<u8>) {
    if let Some(immediate) = immediate_number(bits) {
        match immediate {
            ImmediateNumber::U8(v) => {
                out.push(OP_NUM_U8);
                out.push(v);
            }
            ImmediateNumber::I8(v) => {
                out.push(OP_NUM_I8);
                out.push(v as u8);
            }
        }
    } else if let Some(&id) = constants.get(&bits) {
        out.push(OP_NUM_CONST);
        out.push(id);
    } else {
        out.push(OP_NUM_F32);
        push_u32(out, bits);
    }
}

fn encode_ast(ast: &Ast, constants: &HashMap<u32, u8>, out: &mut Vec<u8>) {
    match ast {
        Ast::Number(bits) => encode_number(*bits, constants, out),
        Ast::Ctx(id) => {
            out.push(OP_CTX);
            out.push(*id);
        }
        Ast::RefLocal(local) => {
            out.push(OP_REF_LOCAL);
            out.push(*local);
        }
        Ast::Match(id) => {
            out.push(OP_MATCH_REF);
            out.push(*id);
        }
        Ast::Group(inner) => {
            out.push(OP_GROUP);
            encode_ast(inner, constants, out);
        }
        Ast::Neg(inner) => {
            out.push(OP_NEG);
            encode_ast(inner, constants, out);
        }
        Ast::Binary {
            op,
            layout,
            left,
            right,
        } => {
            let opcode = OP_BIN_BASE | ((*op as u8) & 0x03) | ((layout.kind as u8) << 2);
            out.push(opcode);
            if layout.kind != BreakKind::Inline {
                out.push(layout.indent_units);
            }
            encode_ast(left, constants, out);
            encode_ast(right, constants, out);
        }
    }
}

fn encode_match(
    m: &MatchDef,
    constants: &HashMap<u32, u8>,
    out: &mut Vec<u8>,
) -> Result<(), Error> {
    if m.arms.len() > u8::MAX as usize {
        return Err(Error::TooManyMatchArms);
    }
    out.push(m.ctx);
    out.push(m.cast_u8 as u8);
    out.push(m.arms.len() as u8);
    for arm in &m.arms {
        match arm.pattern {
            MatchPattern::Exact(v) => {
                out.push(0);
                out.push(v);
            }
            MatchPattern::ToExclusive(v) => {
                out.push(1);
                out.push(v);
            }
            MatchPattern::ToInclusive(v) => {
                out.push(2);
                out.push(v);
            }
            MatchPattern::RangeExclusive(a, b) => {
                out.push(3);
                out.push(a);
                out.push(b);
            }
            MatchPattern::RangeInclusive(a, b) => {
                out.push(4);
                out.push(a);
                out.push(b);
            }
            MatchPattern::From(v) => {
                out.push(5);
                out.push(v);
            }
        }
        encode_number(arm.value, constants, out);
    }
    Ok(())
}

/// Free-function form of bytecode generation if you prefer a single final call.
#[inline]
pub fn generate_bytecode(builder: FormulaDbBuilder) -> Result<Vec<u8>, Error> {
    builder.finish()
}

// -------------------------------------------------------------------------------------------------
// Runtime database view
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
struct Layout {
    constant_pool: usize,
    match_offsets: usize,
    champion_bases: usize,
    item_entries: usize,
    rune_entries: usize,
    formula_offsets: usize,
    match_data: usize,
    formula_data: usize,
}

/// Zero-copy view over an embedded byte buffer.
pub struct FormulaDb<'a> {
    bytes: &'a [u8],
    constants: u16,
    matches: u16,
    champions: u16,
    items: u16,
    runes: u16,
    formulas: u16,
    match_data_len: u32,
    layout: Layout,
}

impl<'a> FormulaDb<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, Error> {
        if bytes.len() < HEADER_LEN {
            return Err(Error::Corrupt("buffer is shorter than header"));
        }
        if bytes[0..4] != MAGIC {
            return Err(Error::Corrupt("bad magic"));
        }
        if bytes[4] != VERSION {
            return Err(Error::Corrupt("unsupported version"));
        }
        let header_len = read_u16_at(bytes, 6)? as usize;
        if header_len != HEADER_LEN {
            return Err(Error::Corrupt("unexpected header length"));
        }

        let constants = read_u16_at(bytes, 8)?;
        let matches = read_u16_at(bytes, 10)?;
        let champions = read_u16_at(bytes, 12)?;
        let items = read_u16_at(bytes, 14)?;
        let runes = read_u16_at(bytes, 16)?;
        let formulas = read_u16_at(bytes, 18)?;
        let match_data_len = read_u32_at(bytes, 20)?;

        if constants > 256 || matches > 256 {
            return Err(Error::Corrupt("u8-indexed pool exceeds 256 entries"));
        }

        let constant_pool = HEADER_LEN;
        let match_offsets = checked_add(constant_pool, constants as usize * 4)?;
        let champion_bases = checked_add(match_offsets, (matches as usize + 1) * 4)?;
        let item_entries = checked_add(champion_bases, champions as usize * 2)?;
        let rune_entries = checked_add(item_entries, items as usize * 3)?;
        let formula_offsets = checked_add(rune_entries, runes as usize * 3)?;
        let match_data = checked_add(formula_offsets, (formulas as usize + 1) * 4)?;
        let formula_data = checked_add(match_data, match_data_len as usize)?;
        if formula_data > bytes.len() {
            return Err(Error::Corrupt("section layout exceeds buffer"));
        }

        let db = Self {
            bytes,
            constants,
            matches,
            champions,
            items,
            runes,
            formulas,
            match_data_len,
            layout: Layout {
                constant_pool,
                match_offsets,
                champion_bases,
                item_entries,
                rune_entries,
                formula_offsets,
                match_data,
                formula_data,
            },
        };
        db.validate_offsets()?;
        Ok(db)
    }

    pub const fn champion_count(&self) -> u16 {
        self.champions
    }
    pub const fn item_count(&self) -> u16 {
        self.items
    }
    pub const fn rune_count(&self) -> u16 {
        self.runes
    }
    pub const fn formula_count(&self) -> u16 {
        self.formulas
    }
    pub const fn match_count(&self) -> u16 {
        self.matches
    }
    pub const fn constant_count(&self) -> u16 {
        self.constants
    }

    /// Champion lookup: owner -> champion base + dense local ability index.
    /// The caller already knows the valid local range from `ChampionId::abilities()`.
    pub fn champion_formula_id(&self, owner_index: u16, local: u8) -> Option<u16> {
        if owner_index >= self.champions {
            return None;
        }
        let pos = self.layout.champion_bases + owner_index as usize * 2;
        let base = read_u16_at(self.bytes, pos).ok()?;
        base.checked_add(local as u16)
            .filter(|id| *id < self.formulas)
    }

    /// Sparse Item lookup. `mask=0` means the item has no damage and immediately returns None.
    pub fn item_formula_id(&self, owner_index: u16, slot: DamageSlot) -> Option<u16> {
        self.sparse_formula_id(EntityKind::Item, owner_index, slot)
    }

    /// Sparse Rune lookup. Same representation as Item.
    pub fn rune_formula_id(&self, owner_index: u16, slot: DamageSlot) -> Option<u16> {
        self.sparse_formula_id(EntityKind::Rune, owner_index, slot)
    }

    fn sparse_formula_id(
        &self,
        kind: EntityKind,
        owner_index: u16,
        slot: DamageSlot,
    ) -> Option<u16> {
        let (count, start) = match kind {
            EntityKind::Item => (self.items, self.layout.item_entries),
            EntityKind::Rune => (self.runes, self.layout.rune_entries),
            EntityKind::Champion => return None,
        };
        if owner_index >= count {
            return None;
        }
        let pos = start + owner_index as usize * 3;
        let first = read_u16_at(self.bytes, pos).ok()?;
        let mask = *self.bytes.get(pos + 2)?;
        let bit = slot.bit();
        if mask & bit == 0 {
            return None;
        }

        // Count physically stored slots before this logical slot.
        let before_mask = if slot as u8 == 0 {
            0
        } else {
            mask & ((1u8 << slot as u8) - 1)
        };
        let rank = before_mask.count_ones() as u16;
        first.checked_add(rank).filter(|id| *id < self.formulas)
    }

    /// Exact byte slice for one formula record, useful if the caller wants to cache/pass it directly.
    pub fn formula_bytes(&self, formula_id: u16) -> Option<&'a [u8]> {
        if formula_id >= self.formulas {
            return None;
        }
        let a = self.formula_offset(formula_id).ok()? as usize;
        let b = self.formula_offset(formula_id + 1).ok()? as usize;
        self.bytes
            .get(self.layout.formula_data + a..self.layout.formula_data + b)
    }

    /// Plain-text renderer. Function calls intentionally render WITHOUT `(ctx)`.
    pub fn render_formula_plain<FC, FR>(
        &self,
        formula_id: u16,
        mut ctx_name: FC,
        mut local_ref_name: FR,
    ) -> Result<String, Error>
    where
        FC: FnMut(u8) -> String,
        FR: FnMut(u8) -> String,
    {
        let mut emit = |out: &mut String, _class: RenderClass, text: &str| {
            out.push_str(text);
        };
        self.render_formula_with(formula_id, &mut ctx_name, &mut local_ref_name, &mut emit)
    }

    /// HTML renderer for your `<span class="C{n}">...</span>` scheme.
    ///
    /// Map `RenderClass` to the numeric value of your existing `Class` enum.
    pub fn render_formula_html<FC, FR, FM>(
        &self,
        formula_id: u16,
        mut ctx_name: FC,
        mut local_ref_name: FR,
        mut class_id: FM,
    ) -> Result<String, Error>
    where
        FC: FnMut(u8) -> String,
        FR: FnMut(u8) -> String,
        FM: FnMut(RenderClass) -> u8,
    {
        let mut emit = |out: &mut String, class: RenderClass, text: &str| {
            out.push_str("<span class=\"C");
            out.push_str(&class_id(class).to_string());
            out.push_str("\">");
            push_html_escaped(out, text);
            out.push_str("</span>");
        };
        self.render_formula_with(formula_id, &mut ctx_name, &mut local_ref_name, &mut emit)
    }

    /// Most flexible renderer. `emit` receives semantic token classes, while spaces/newlines
    /// are written directly and therefore never need their own span.
    pub fn render_formula_with(
        &self,
        formula_id: u16,
        ctx_name: &mut dyn FnMut(u8) -> String,
        local_ref_name: &mut dyn FnMut(u8) -> String,
        emit: &mut dyn FnMut(&mut String, RenderClass, &str),
    ) -> Result<String, Error> {
        let bytes = self
            .formula_bytes(formula_id)
            .ok_or(Error::Corrupt("formula id out of range"))?;
        let mut cursor = Cursor::new(bytes);
        let mut out = String::new();
        let mut renderer = Renderer {
            db: self,
            ctx_name,
            local_ref_name,
            emit,
        };
        renderer.expr(&mut cursor, &mut out)?;
        if !cursor.is_eof() {
            return Err(Error::Corrupt("formula has trailing bytecode"));
        }
        Ok(out)
    }

    fn constant(&self, id: u8) -> Result<f32, Error> {
        if id as u16 >= self.constants {
            return Err(Error::Corrupt("constant id out of range"));
        }
        let pos = self.layout.constant_pool + id as usize * 4;
        Ok(f32::from_bits(read_u32_at(self.bytes, pos)?))
    }

    fn match_bytes(&self, id: u8) -> Result<&'a [u8], Error> {
        if id as u16 >= self.matches {
            return Err(Error::Corrupt("match id out of range"));
        }
        let a = self.match_offset(id as u16)? as usize;
        let b = self.match_offset(id as u16 + 1)? as usize;
        if b < a || b > self.match_data_len as usize {
            return Err(Error::Corrupt("bad match offset"));
        }
        self.bytes
            .get(self.layout.match_data + a..self.layout.match_data + b)
            .ok_or(Error::Corrupt("match slice out of bounds"))
    }

    fn match_offset(&self, index: u16) -> Result<u32, Error> {
        let pos = self.layout.match_offsets + index as usize * 4;
        read_u32_at(self.bytes, pos)
    }

    fn formula_offset(&self, index: u16) -> Result<u32, Error> {
        let pos = self.layout.formula_offsets + index as usize * 4;
        read_u32_at(self.bytes, pos)
    }

    fn validate_offsets(&self) -> Result<(), Error> {
        let mut last = 0u32;
        for i in 0..=self.matches {
            let v = self.match_offset(i)?;
            if v < last || v > self.match_data_len {
                return Err(Error::Corrupt("non-monotonic match offsets"));
            }
            last = v;
        }
        if last != self.match_data_len {
            return Err(Error::Corrupt("match offsets do not end at match_data_len"));
        }

        let formula_data_len = self.bytes.len() - self.layout.formula_data;
        let mut last = 0u32;
        for i in 0..=self.formulas {
            let v = self.formula_offset(i)?;
            if v < last || v as usize > formula_data_len {
                return Err(Error::Corrupt("non-monotonic formula offsets"));
            }
            last = v;
        }
        if last as usize != formula_data_len {
            return Err(Error::Corrupt(
                "formula offsets do not end at formula data end",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderClass {
    Number,
    Context,
    Field,
    Operator,
    Keyword,
    Type,
    Function,
    Punctuation,
}

struct Renderer<'db, 'bytes, 'cb> {
    db: &'db FormulaDb<'bytes>,
    ctx_name: &'cb mut dyn FnMut(u8) -> String,
    local_ref_name: &'cb mut dyn FnMut(u8) -> String,
    emit: &'cb mut dyn FnMut(&mut String, RenderClass, &str),
}

impl Renderer<'_, '_, '_> {
    fn expr(&mut self, cursor: &mut Cursor<'_>, out: &mut String) -> Result<(), Error> {
        let opcode = cursor.u8()?;
        match opcode {
            OP_NUM_U8 => {
                let v = cursor.u8()?;
                self.token(out, RenderClass::Number, &v.to_string());
            }
            OP_NUM_I8 => {
                let v = cursor.u8()? as i8;
                self.token(out, RenderClass::Number, &v.to_string());
            }
            OP_NUM_CONST => {
                let id = cursor.u8()?;
                let v = self.db.constant(id)?;
                self.token(out, RenderClass::Number, &format_f32(v));
            }
            OP_NUM_F32 => {
                let bits = cursor.u32()?;
                self.token(out, RenderClass::Number, &format_f32(f32::from_bits(bits)));
            }
            OP_CTX => {
                let id = cursor.u8()?;
                let name = (self.ctx_name)(id);
                self.token(out, RenderClass::Context, "ctx");
                self.token(out, RenderClass::Punctuation, ".");
                self.token(out, RenderClass::Field, &name);
            }
            OP_REF_LOCAL => {
                let local = cursor.u8()?;
                let name = (self.local_ref_name)(local);
                self.token(out, RenderClass::Function, &name);
            }
            OP_MATCH_REF => {
                let id = cursor.u8()?;
                self.render_match(id, out)?;
            }
            OP_GROUP => {
                self.token(out, RenderClass::Punctuation, "(");
                self.expr(cursor, out)?;
                self.token(out, RenderClass::Punctuation, ")");
            }
            OP_NEG => {
                self.token(out, RenderClass::Operator, "-");
                self.expr(cursor, out)?;
            }
            OP_BIN_BASE..=OP_BIN_MAX => {
                let op = match opcode & 0x03 {
                    0 => "+",
                    1 => "-",
                    2 => "*",
                    3 => "/",
                    _ => unreachable!(),
                };
                let break_kind = (opcode >> 2) & 0x03;
                let indent = if break_kind == 0 { 0 } else { cursor.u8()? };

                self.expr(cursor, out)?;
                match break_kind {
                    0 => {
                        out.push(' ');
                        self.token(out, RenderClass::Operator, op);
                        out.push(' ');
                    }
                    1 => {
                        out.push('\n');
                        push_indent(out, indent);
                        self.token(out, RenderClass::Operator, op);
                        out.push(' ');
                    }
                    2 => {
                        out.push(' ');
                        self.token(out, RenderClass::Operator, op);
                        out.push('\n');
                        push_indent(out, indent);
                    }
                    _ => return Err(Error::Corrupt("reserved binary break kind")),
                }
                self.expr(cursor, out)?;
            }
            _ => return Err(Error::Corrupt("unknown expression opcode")),
        }
        Ok(())
    }

    fn render_match(&mut self, id: u8, out: &mut String) -> Result<(), Error> {
        let bytes = self.db.match_bytes(id)?;
        let mut c = Cursor::new(bytes);
        let ctx = c.u8()?;
        let flags = c.u8()?;
        let arm_count = c.u8()?;

        self.token(out, RenderClass::Keyword, "match");
        out.push(' ');
        self.token(out, RenderClass::Context, "ctx");
        self.token(out, RenderClass::Punctuation, ".");
        let name = (self.ctx_name)(ctx);
        self.token(out, RenderClass::Field, &name);
        if flags & 1 != 0 {
            out.push(' ');
            self.token(out, RenderClass::Keyword, "as");
            out.push(' ');
            self.token(out, RenderClass::Type, "u8");
        }
        out.push(' ');
        self.token(out, RenderClass::Punctuation, "{");
        out.push(' ');

        for i in 0..arm_count {
            let pattern = c.u8()?;
            self.render_pattern(&mut c, out, pattern)?;
            out.push(' ');
            self.token(out, RenderClass::Operator, "=>");
            out.push(' ');
            self.render_encoded_number(&mut c, out)?;
            if i + 1 != arm_count {
                self.token(out, RenderClass::Punctuation, ",");
                out.push(' ');
            }
        }

        out.push(' ');
        self.token(out, RenderClass::Punctuation, "}");
        if !c.is_eof() {
            return Err(Error::Corrupt("match has trailing bytes"));
        }
        Ok(())
    }

    fn render_pattern(
        &mut self,
        c: &mut Cursor<'_>,
        out: &mut String,
        pattern: u8,
    ) -> Result<(), Error> {
        match pattern {
            0 => {
                let v = c.u8()?;
                self.token(out, RenderClass::Number, &v.to_string());
            }
            1 => {
                self.token(out, RenderClass::Operator, "..");
                let v = c.u8()?;
                self.token(out, RenderClass::Number, &v.to_string());
            }
            2 => {
                self.token(out, RenderClass::Operator, "..=");
                let v = c.u8()?;
                self.token(out, RenderClass::Number, &v.to_string());
            }
            3 | 4 => {
                let a = c.u8()?;
                let b = c.u8()?;
                self.token(out, RenderClass::Number, &a.to_string());
                self.token(
                    out,
                    RenderClass::Operator,
                    if pattern == 3 { ".." } else { "..=" },
                );
                self.token(out, RenderClass::Number, &b.to_string());
            }
            5 => {
                let v = c.u8()?;
                self.token(out, RenderClass::Number, &v.to_string());
                self.token(out, RenderClass::Operator, "..");
            }
            _ => return Err(Error::Corrupt("unknown match pattern")),
        }
        Ok(())
    }

    fn render_encoded_number(&mut self, c: &mut Cursor<'_>, out: &mut String) -> Result<(), Error> {
        let opcode = c.u8()?;
        let text = match opcode {
            OP_NUM_U8 => c.u8()?.to_string(),
            OP_NUM_I8 => (c.u8()? as i8).to_string(),
            OP_NUM_CONST => format_f32(self.db.constant(c.u8()?)?),
            OP_NUM_F32 => format_f32(f32::from_bits(c.u32()?)),
            _ => return Err(Error::Corrupt("match RHS is not a number opcode")),
        };
        self.token(out, RenderClass::Number, &text);
        Ok(())
    }

    #[inline]
    fn token(&mut self, out: &mut String, class: RenderClass, text: &str) {
        (self.emit)(out, class, text);
    }
}

fn push_indent(out: &mut String, units: u8) {
    for _ in 0..units as usize * 2 {
        out.push(' ');
    }
}

fn format_f32(v: f32) -> String {
    // Rust's Display uses a short round-tripping representation, which is exactly
    // what we want because textual identity with the input is not required.
    v.to_string()
}

fn push_html_escaped(out: &mut String, text: &str) {
    for ch in text.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Tiny expression parser that preserves operator line/column information.
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum TokenKind {
    Number(String),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Dot,
    Comma,
    Range,
    RangeEq,
    FatArrow,
}

#[derive(Debug, Clone)]
struct Token {
    kind: TokenKind,
    line: u32,
    column: u32,
}

fn tokenize(source: &str) -> Result<Vec<Token>, Error> {
    let bytes = source.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    let mut line = 1u32;
    let mut col = 0u32;

    while i < bytes.len() {
        match bytes[i] {
            b' ' => {
                i += 1;
                col += 1;
            }
            b'\t' => {
                return Err(Error::Parse {
                    line,
                    column: col,
                    message: "tabs are not supported; rustfmt input should use spaces".into(),
                });
            }
            b'\n' => {
                i += 1;
                line += 1;
                col = 0;
            }
            b'\r' => {
                if bytes.get(i + 1) == Some(&b'\n') {
                    i += 2;
                } else {
                    i += 1;
                }
                line += 1;
                col = 0;
            }
            b'0'..=b'9' => {
                let start = i;
                let start_col = col;
                while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'_') {
                    i += 1;
                    col += 1;
                }
                if i < bytes.len() && bytes[i] == b'.' && bytes.get(i + 1) != Some(&b'.') {
                    i += 1;
                    col += 1;
                    while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'_') {
                        i += 1;
                        col += 1;
                    }
                }
                if i < bytes.len() && matches!(bytes[i], b'e' | b'E') {
                    i += 1;
                    col += 1;
                    if i < bytes.len() && matches!(bytes[i], b'+' | b'-') {
                        i += 1;
                        col += 1;
                    }
                    while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'_') {
                        i += 1;
                        col += 1;
                    }
                }
                // Optional Rust float suffix. Semantically everything is f32 here.
                if source[i..].starts_with("f32") || source[i..].starts_with("f64") {
                    i += 3;
                    col += 3;
                }
                out.push(Token {
                    kind: TokenKind::Number(source[start..i].to_string()),
                    line,
                    column: start_col,
                });
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let start = i;
                let start_col = col;
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                    col += 1;
                }
                out.push(Token {
                    kind: TokenKind::Ident(source[start..i].to_string()),
                    line,
                    column: start_col,
                });
            }
            b'+' => one(&mut out, TokenKind::Plus, &mut i, line, &mut col),
            b'-' => one(&mut out, TokenKind::Minus, &mut i, line, &mut col),
            b'*' => one(&mut out, TokenKind::Star, &mut i, line, &mut col),
            b'/' => one(&mut out, TokenKind::Slash, &mut i, line, &mut col),
            b'(' => one(&mut out, TokenKind::LParen, &mut i, line, &mut col),
            b')' => one(&mut out, TokenKind::RParen, &mut i, line, &mut col),
            b'{' => one(&mut out, TokenKind::LBrace, &mut i, line, &mut col),
            b'}' => one(&mut out, TokenKind::RBrace, &mut i, line, &mut col),
            b',' => one(&mut out, TokenKind::Comma, &mut i, line, &mut col),
            b'.' => {
                let start_col = col;
                if bytes.get(i + 1) == Some(&b'.') {
                    if bytes.get(i + 2) == Some(&b'=') {
                        out.push(Token {
                            kind: TokenKind::RangeEq,
                            line,
                            column: start_col,
                        });
                        i += 3;
                        col += 3;
                    } else {
                        out.push(Token {
                            kind: TokenKind::Range,
                            line,
                            column: start_col,
                        });
                        i += 2;
                        col += 2;
                    }
                } else {
                    one(&mut out, TokenKind::Dot, &mut i, line, &mut col);
                }
            }
            b'=' if bytes.get(i + 1) == Some(&b'>') => {
                out.push(Token {
                    kind: TokenKind::FatArrow,
                    line,
                    column: col,
                });
                i += 2;
                col += 2;
            }
            other => {
                return Err(Error::Parse {
                    line,
                    column: col,
                    message: format!("unsupported byte `{}`", other as char),
                });
            }
        }
    }
    Ok(out)
}

fn one(out: &mut Vec<Token>, kind: TokenKind, i: &mut usize, line: u32, col: &mut u32) {
    out.push(Token {
        kind,
        line,
        column: *col,
    });
    *i += 1;
    *col += 1;
}

#[derive(Debug)]
struct Parsed {
    ast: Ast,
    start_line: u32,
    start_col: u32,
    end_line: u32,
}

struct Parser<'a> {
    tokens: Vec<Token>,
    pos: usize,
    ctx_resolver: CtxResolver,
    refs: &'a HashMap<String, u8>,
    match_defs: &'a mut Vec<MatchDef>,
    match_ids: &'a mut HashMap<MatchDef, u8>,
}

impl Parser<'_> {
    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn bump(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    fn parse_expression(&mut self, min_precedence: u8) -> Result<Parsed, Error> {
        let mut left = self.parse_unary()?;

        loop {
            let Some(tok) = self.peek() else { break };
            let (op, precedence) = match tok.kind {
                TokenKind::Plus => (BinaryOp::Add, 1),
                TokenKind::Minus => (BinaryOp::Sub, 1),
                TokenKind::Star => (BinaryOp::Mul, 2),
                TokenKind::Slash => (BinaryOp::Div, 2),
                _ => break,
            };
            if precedence < min_precedence {
                break;
            }

            let op_tok = self.bump().unwrap();
            let right = self.parse_expression(precedence + 1)?;
            let layout = if op_tok.line > left.end_line {
                layout_from_column(BreakKind::BeforeOperator, &op_tok)?
            } else if right.start_line > op_tok.line {
                let fake = Token {
                    kind: TokenKind::Plus,
                    line: right.start_line,
                    column: right.start_col,
                };
                layout_from_column(BreakKind::AfterOperator, &fake)?
            } else {
                OpLayout::INLINE
            };

            left = Parsed {
                start_line: left.start_line,
                start_col: left.start_col,
                end_line: right.end_line,
                ast: Ast::Binary {
                    op,
                    layout,
                    left: Box::new(left.ast),
                    right: Box::new(right.ast),
                },
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Parsed, Error> {
        if matches!(self.peek().map(|t| &t.kind), Some(TokenKind::Minus)) {
            let minus = self.bump().unwrap();
            let inner = self.parse_unary()?;
            let ast = match inner.ast {
                Ast::Number(bits) => Ast::Number((-f32::from_bits(bits)).to_bits()),
                other => Ast::Neg(Box::new(other)),
            };
            return Ok(Parsed {
                ast,
                start_line: minus.line,
                start_col: minus.column,
                end_line: inner.end_line,
            });
        }
        if matches!(self.peek().map(|t| &t.kind), Some(TokenKind::Plus)) {
            self.bump();
            return self.parse_unary();
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Parsed, Error> {
        let token = self.bump().ok_or_else(|| Error::Parse {
            line: 1,
            column: 0,
            message: "unexpected end of formula".into(),
        })?;

        match token.kind.clone() {
            TokenKind::Number(s) => {
                let value = parse_f32_literal(&s).map_err(|msg| Error::Parse {
                    line: token.line,
                    column: token.column,
                    message: msg,
                })?;
                Ok(Parsed {
                    ast: Ast::Number(value.to_bits()),
                    start_line: token.line,
                    start_col: token.column,
                    end_line: token.line,
                })
            }
            TokenKind::LParen => {
                let inner = self.parse_expression(0)?;
                let close = self.expect_simple(TokenKind::RParen, "expected `)`")?;
                Ok(Parsed {
                    ast: Ast::Group(Box::new(inner.ast)),
                    start_line: token.line,
                    start_col: token.column,
                    end_line: close.line,
                })
            }
            TokenKind::Ident(ref s) if s == "match" => self.parse_match(token),
            TokenKind::Ident(ref s) if s == "ctx" => self.parse_ctx(token),
            TokenKind::Ident(name) => self.parse_function_call(token, name),
            _ => Err(parse_err(
                &token,
                "expected number, ctx field, function call, match, or parenthesized expression",
            )),
        }
    }

    fn parse_ctx(&mut self, start: Token) -> Result<Parsed, Error> {
        self.expect_simple(TokenKind::Dot, "expected `.` after `ctx`")?;
        let field_tok = self
            .bump()
            .ok_or_else(|| parse_err(&start, "expected ctx field"))?;
        let field = match &field_tok.kind {
            TokenKind::Ident(s) => s.clone(),
            _ => return Err(parse_err(&field_tok, "expected ctx field identifier")),
        };
        let id = (self.ctx_resolver)(&field).ok_or_else(|| Error::UnknownCtxVar(field.clone()))?;
        Ok(Parsed {
            ast: Ast::Ctx(id),
            start_line: start.line,
            start_col: start.column,
            end_line: field_tok.line,
        })
    }

    fn parse_function_call(&mut self, start: Token, name: String) -> Result<Parsed, Error> {
        self.expect_simple(TokenKind::LParen, "expected `(` after function name")?;

        let arg = self
            .bump()
            .ok_or_else(|| parse_err(&start, "expected function argument"))?;

        match &arg.kind {
            TokenKind::Ident(s) if s == "ctx" => {}
            _ => {
                return Err(parse_err(
                    &arg,
                    "local function calls must have exactly `ctx` as argument",
                ));
            }
        }

        if matches!(self.peek().map(|t| &t.kind), Some(TokenKind::Comma)) {
            self.bump();
        }

        let close = self.expect_simple(TokenKind::RParen, "expected `)` after `ctx`")?;

        let local = *self
            .refs
            .get(&name)
            .ok_or_else(|| Error::UnknownFunction(name.clone()))?;

        Ok(Parsed {
            ast: Ast::RefLocal(local),
            start_line: start.line,
            start_col: start.column,
            end_line: close.line,
        })
    }

    fn parse_match(&mut self, start: Token) -> Result<Parsed, Error> {
        let ctx_tok = self
            .bump()
            .ok_or_else(|| parse_err(&start, "expected match scrutinee"))?;
        match &ctx_tok.kind {
            TokenKind::Ident(s) if s == "ctx" => {}
            _ => return Err(parse_err(&ctx_tok, "match scrutinee must start with `ctx`")),
        }
        self.expect_simple(TokenKind::Dot, "expected `.` in match scrutinee")?;
        let field_tok = self
            .bump()
            .ok_or_else(|| parse_err(&ctx_tok, "expected ctx field"))?;
        let field = match &field_tok.kind {
            TokenKind::Ident(s) => s.clone(),
            _ => return Err(parse_err(&field_tok, "expected ctx field identifier")),
        };
        let ctx = (self.ctx_resolver)(&field).ok_or_else(|| Error::UnknownCtxVar(field.clone()))?;

        let mut cast_u8 = false;
        if matches!(self.peek().map(|t| &t.kind), Some(TokenKind::Ident(s)) if s == "as") {
            self.bump();
            let ty = self
                .bump()
                .ok_or_else(|| parse_err(&field_tok, "expected `u8` after `as`"))?;
            match &ty.kind {
                TokenKind::Ident(s) if s == "u8" => cast_u8 = true,
                _ => {
                    return Err(parse_err(
                        &ty,
                        "only `as u8` is supported in match scrutinee",
                    ));
                }
            }
        }

        self.expect_simple(TokenKind::LBrace, "expected `{` after match scrutinee")?;
        let mut arms = Vec::new();
        while !matches!(self.peek().map(|t| &t.kind), Some(TokenKind::RBrace)) {
            if self.is_eof() {
                return Err(parse_err(&start, "unterminated match"));
            }
            let pattern = self.parse_match_pattern()?;
            self.expect_simple(TokenKind::FatArrow, "expected `=>` in match arm")?;
            let value = self.parse_match_rhs_number()?;
            arms.push(MatchArm {
                pattern,
                value: value.to_bits(),
            });
            if arms.len() > u8::MAX as usize {
                return Err(Error::TooManyMatchArms);
            }
            if matches!(self.peek().map(|t| &t.kind), Some(TokenKind::Comma)) {
                self.bump();
            } else if !matches!(self.peek().map(|t| &t.kind), Some(TokenKind::RBrace)) {
                let t = self.peek().unwrap();
                return Err(parse_err(t, "expected `,` or `}` after match arm"));
            }
        }
        let close = self.expect_simple(TokenKind::RBrace, "expected `}`")?;

        let def = MatchDef { ctx, cast_u8, arms };
        let id = if let Some(&id) = self.match_ids.get(&def) {
            id
        } else {
            if self.match_defs.len() >= 256 {
                return Err(Error::TooManyMatches);
            }
            let id = self.match_defs.len() as u8;
            self.match_defs.push(def.clone());
            self.match_ids.insert(def, id);
            id
        };

        Ok(Parsed {
            ast: Ast::Match(id),
            start_line: start.line,
            start_col: start.column,
            end_line: close.line,
        })
    }

    fn parse_match_pattern(&mut self) -> Result<MatchPattern, Error> {
        let first = self.bump().ok_or_else(|| Error::Parse {
            line: 1,
            column: 0,
            message: "unexpected end in match pattern".into(),
        })?;
        match first.kind.clone() {
            TokenKind::Range => {
                let bound = self.expect_pattern_u8("expected upper bound after `..`")?;
                Ok(MatchPattern::ToExclusive(bound))
            }
            TokenKind::RangeEq => {
                let bound = self.expect_pattern_u8("expected upper bound after `..=`")?;
                Ok(MatchPattern::ToInclusive(bound))
            }
            TokenKind::Number(s) => {
                let start = parse_pattern_u8(&s).map_err(|msg| Error::Parse {
                    line: first.line,
                    column: first.column,
                    message: msg,
                })?;
                match self.peek().map(|t| &t.kind) {
                    Some(TokenKind::Range) => {
                        self.bump();
                        if matches!(self.peek().map(|t| &t.kind), Some(TokenKind::FatArrow)) {
                            Ok(MatchPattern::From(start))
                        } else {
                            let end = self.expect_pattern_u8("expected range end")?;
                            Ok(MatchPattern::RangeExclusive(start, end))
                        }
                    }
                    Some(TokenKind::RangeEq) => {
                        self.bump();
                        let end = self.expect_pattern_u8("expected inclusive range end")?;
                        Ok(MatchPattern::RangeInclusive(start, end))
                    }
                    _ => Ok(MatchPattern::Exact(start)),
                }
            }
            _ => Err(parse_err(&first, "unsupported match pattern")),
        }
    }

    fn expect_pattern_u8(&mut self, msg: &str) -> Result<u8, Error> {
        let t = self.bump().ok_or_else(|| Error::Parse {
            line: 1,
            column: 0,
            message: msg.into(),
        })?;
        let s = match &t.kind {
            TokenKind::Number(s) => s,
            _ => return Err(parse_err(&t, msg)),
        };
        parse_pattern_u8(s).map_err(|m| Error::Parse {
            line: t.line,
            column: t.column,
            message: m,
        })
    }

    fn parse_match_rhs_number(&mut self) -> Result<f32, Error> {
        let negative = matches!(self.peek().map(|t| &t.kind), Some(TokenKind::Minus));
        if negative {
            self.bump();
        }
        let t = self.bump().ok_or_else(|| Error::Parse {
            line: 1,
            column: 0,
            message: "expected numeric match RHS".into(),
        })?;
        let s = match &t.kind {
            TokenKind::Number(s) => s,
            _ => return Err(parse_err(&t, "match RHS must be a number")),
        };
        let mut v = parse_f32_literal(s).map_err(|msg| Error::Parse {
            line: t.line,
            column: t.column,
            message: msg,
        })?;
        if negative {
            v = -v;
        }
        Ok(v)
    }

    fn expect_simple(&mut self, expected: TokenKind, msg: &str) -> Result<Token, Error> {
        let t = self.bump().ok_or_else(|| Error::Parse {
            line: 1,
            column: 0,
            message: msg.into(),
        })?;
        if std::mem::discriminant(&t.kind) != std::mem::discriminant(&expected) {
            return Err(parse_err(&t, msg));
        }
        Ok(t)
    }
}

fn layout_from_column(kind: BreakKind, token: &Token) -> Result<OpLayout, Error> {
    if token.column % 2 != 0 || token.column / 2 > u8::MAX as u32 {
        return Err(Error::InvalidIndent {
            line: token.line,
            column: token.column,
        });
    }
    Ok(OpLayout {
        kind,
        indent_units: (token.column / 2) as u8,
    })
}

fn parse_err(token: &Token, message: impl Into<String>) -> Error {
    Error::Parse {
        line: token.line,
        column: token.column,
        message: message.into(),
    }
}

fn parse_f32_literal(s: &str) -> Result<f32, String> {
    let mut cleaned = s.replace('_', "");
    if cleaned.ends_with("f32") || cleaned.ends_with("f64") {
        cleaned.truncate(cleaned.len() - 3);
    }
    cleaned
        .parse::<f32>()
        .map_err(|_| format!("invalid f32 literal `{s}`"))
}

fn parse_pattern_u8(s: &str) -> Result<u8, String> {
    let v = parse_f32_literal(s)?;
    if !v.is_finite() || v.fract() != 0.0 || !(0.0..=255.0).contains(&v) {
        return Err(format!("match pattern bound `{s}` is not a u8"));
    }
    Ok(v as u8)
}

// -------------------------------------------------------------------------------------------------
// Binary helpers
// -------------------------------------------------------------------------------------------------

struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }
    fn is_eof(&self) -> bool {
        self.pos == self.bytes.len()
    }
    fn u8(&mut self) -> Result<u8, Error> {
        let v = *self
            .bytes
            .get(self.pos)
            .ok_or(Error::Corrupt("unexpected end of bytecode"))?;
        self.pos += 1;
        Ok(v)
    }
    fn u32(&mut self) -> Result<u32, Error> {
        let end = self
            .pos
            .checked_add(4)
            .ok_or(Error::Corrupt("offset overflow"))?;
        let slice = self
            .bytes
            .get(self.pos..end)
            .ok_or(Error::Corrupt("unexpected end of bytecode"))?;
        self.pos = end;
        Ok(u32::from_le_bytes(slice.try_into().unwrap()))
    }
}

fn push_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn read_u16_at(bytes: &[u8], pos: usize) -> Result<u16, Error> {
    let end = pos
        .checked_add(2)
        .ok_or(Error::Corrupt("offset overflow"))?;
    let s = bytes
        .get(pos..end)
        .ok_or(Error::Corrupt("u16 out of bounds"))?;
    Ok(u16::from_le_bytes(s.try_into().unwrap()))
}

fn read_u32_at(bytes: &[u8], pos: usize) -> Result<u32, Error> {
    let end = pos
        .checked_add(4)
        .ok_or(Error::Corrupt("offset overflow"))?;
    let s = bytes
        .get(pos..end)
        .ok_or(Error::Corrupt("u32 out of bounds"))?;
    Ok(u32::from_le_bytes(s.try_into().unwrap()))
}

fn checked_add(a: usize, b: usize) -> Result<usize, Error> {
    a.checked_add(b)
        .ok_or(Error::Corrupt("section offset overflow"))
}

// -------------------------------------------------------------------------------------------------
// Example integration (not compiled as tests because your real enums are project-specific)
// -------------------------------------------------------------------------------------------------

/*
BUILD-TIME SKETCH
-----------------

fn ctx_id(name: &str) -> Option<u8> {
    CtxVar::from_str(name).map(|v| v as u8)
}

let mut b = FormulaDbBuilder::new(
    ChampionId::LEN as u16,
    ItemId::LEN as u16,   // owner index table; owners with no damage stay mask=0
    RuneId::LEN as u16,
    ctx_id,
);

for champion in ChampionId::ALL {
    let mut refs = HashMap::<String, u8>::new();
    let mut formulas = Vec::new();

    for (local, (ability_id, ability)) in champion.abilities().iter().enumerate() {
        // Use whatever exact naming helper produced calls such as `gnar_qmin(ctx)`.
        refs.insert(format!("{}_{}", champion.as_fn_prefix(), ability_id.discriminant().to_lowercase()), local as u8);
        formulas.push(FormulaSource {
            local: local as u8,
            source: ability.damage.as_str(),
        });
    }

    b.push_champion(champion.owner_index_u16(), &formulas, &refs)?;
}

for item in ItemId::ALL {
    // If the item has no damage: do nothing. Its dense owner table entry remains mask=0.
    let Some(damage) = item.damage() else { continue };

    let mut refs = HashMap::<String, u8>::new();
    refs.insert(item.melee_min_fn_name(), DamageSlot::MeleeMin as u8);
    refs.insert(item.melee_max_fn_name(), DamageSlot::MeleeMax as u8);
    refs.insert(item.ranged_min_fn_name(), DamageSlot::RangedMin as u8);
    refs.insert(item.ranged_max_fn_name(), DamageSlot::RangedMax as u8);

    let mut slots = Vec::new();
    if let Some(s) = damage.melee_min.as_deref() {
        slots.push(SlottedFormulaSource { slot: DamageSlot::MeleeMin, source: s });
    }
    if let Some(s) = damage.melee_max.as_deref() {
        slots.push(SlottedFormulaSource { slot: DamageSlot::MeleeMax, source: s });
    }
    if let Some(s) = damage.ranged_min.as_deref() {
        slots.push(SlottedFormulaSource { slot: DamageSlot::RangedMin, source: s });
    }
    if let Some(s) = damage.ranged_max.as_deref() {
        slots.push(SlottedFormulaSource { slot: DamageSlot::RangedMax, source: s });
    }

    b.push_item(item.owner_index_u16(), &slots, &refs)?;
}

// Runes use exactly the same pattern as Items.
let binary: Vec<u8> = b.finish()?;
std::fs::write(out_dir.join("formula_damage.bin"), &binary)?;

RUNTIME SKETCH
--------------

static FORMULA_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/formula_damage.bin"));
let db = FormulaDb::parse(FORMULA_BYTES)?;

// Champion:
let local = champion.indexof_ability(ability_id)? as u8;
let formula_id = db.champion_formula_id(champion.owner_index_u16(), local)?;
let html = db.render_formula_html(
    formula_id,
    |ctx| CtxVar::from_repr(ctx).unwrap().as_var().to_owned(),
    |local| {
        let ability = &champion.abilities()[local as usize];
        format!("{}_{}", champion.as_fn_prefix(), ability.kind.discriminant().to_lowercase())
    },
    |class| match class {
        RenderClass::Number => Class::Number as u8,
        RenderClass::Context => Class::Variable as u8,
        RenderClass::Field => Class::Field as u8,
        RenderClass::Operator => Class::Operator as u8,
        RenderClass::Keyword => Class::Keyword as u8,
        RenderClass::Type => Class::Type as u8,
        RenderClass::Function => Class::Function as u8,
        RenderClass::Punctuation => Class::Punctuation as u8,
    },
)?;

// Item:
if let Some(formula_id) = db.item_formula_id(item.owner_index_u16(), DamageSlot::RangedMin) {
    // `local` passed to the ref resolver is a LOGICAL DamageSlot, not physical rank.
    let html = db.render_formula_html(
        formula_id,
        |ctx| CtxVar::from_repr(ctx).unwrap().as_var().to_owned(),
        |local| {
            let slot = match local {
                0 => DamageSlot::MeleeMin,
                1 => DamageSlot::MeleeMax,
                2 => DamageSlot::RangedMin,
                3 => DamageSlot::RangedMax,
                _ => unreachable!(),
            };
            item.damage_fn_name(slot)
        },
        class_mapper,
    )?;
}
*/
