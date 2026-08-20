use crate::common::{
    DamageSlot, EntityKind, Error, HEADER_LEN, MAGIC, OP_BIN_BASE, OP_CTX, OP_GROUP,
    OP_MATCH_BLOCK, OP_MATCH_INLINE, OP_NEG, OP_NUM_CONST, OP_NUM_F32, OP_NUM_I8, OP_NUM_U8,
    OP_REF_LOCAL, VERSION,
};
use std::collections::HashMap;
use std::convert::TryFrom;

/// Build-time conversion used for `ctx.foo -> u8`.
///
/// In the tutorlolv2 integration this can simply wrap `CtxVar::from_str`.
pub type CtxResolver = fn(&str) -> Option<u8>;

/// One build-time champion formula. `local` must be dense `0..len`.
/// The builder parses it immediately, so the source can be borrowed.
#[derive(Debug, Clone, Copy)]
pub struct FormulaSource<'a> {
    pub local: u8,
    pub source: &'a str,
}

#[derive(Debug, Clone)]
enum Ast {
    Number(u32),
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
    Match {
        id: u8,
        layout: MatchLayout,
    },
}

impl Ast {
    fn count_numbers(&self, freq: &mut HashMap<u32, u32>) {
        match self {
            Self::Number(bits) => *freq.entry(*bits).or_default() += 1,
            Self::Binary { left, right, .. } => {
                left.count_numbers(freq);
                right.count_numbers(freq);
            }
            Self::Group(inner) | Self::Neg(inner) => inner.count_numbers(freq),
            Self::Ctx(_) | Self::RefLocal(_) | Self::Match { .. } => {}
        }
    }

    fn encode(&self, constants: &ConstantPool, out: &mut Vec<u8>) {
        match self {
            Self::Number(bits) => constants.encode_number(*bits, out),
            Self::Ctx(id) => {
                out.push(OP_CTX);
                out.push(*id);
            }
            Self::RefLocal(local) => {
                out.push(OP_REF_LOCAL);
                out.push(*local);
            }
            Self::Match {
                id,
                layout: MatchLayout::Inline,
            } => {
                out.push(OP_MATCH_INLINE);
                out.push(*id);
            }
            Self::Match {
                id,
                layout:
                    MatchLayout::Block {
                        arm_indent_units,
                        close_indent_units,
                    },
            } => {
                out.push(OP_MATCH_BLOCK);
                out.push(*id);
                out.push(*arm_indent_units);
                out.push(*close_indent_units);
            }
            Self::Group(inner) => {
                out.push(OP_GROUP);
                inner.encode(constants, out);
            }
            Self::Neg(inner) => {
                out.push(OP_NEG);
                inner.encode(constants, out);
            }
            Self::Binary {
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
                left.encode(constants, out);
                right.encode(constants, out);
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BinaryOp {
    Add = 0,
    Sub = 1,
    Mul = 2,
    Div = 3,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BreakKind {
    Inline = 0,
    BeforeOperator = 1,
    AfterOperator = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OpLayout {
    kind: BreakKind,
    indent_units: u8,
}

impl OpLayout {
    const INLINE: Self = Self {
        kind: BreakKind::Inline,
        indent_units: 0,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatchLayout {
    Inline,
    Block {
        arm_indent_units: u8,
        close_indent_units: u8,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MatchDef {
    ctx: u8,
    cast_u8: bool,
    arms: Vec<MatchArm>,
}

impl MatchDef {
    fn count_numbers(&self, freq: &mut HashMap<u32, u32>) {
        for arm in &self.arms {
            *freq.entry(arm.value).or_default() += 1;
        }
    }

    fn encode(&self, constants: &ConstantPool, out: &mut Vec<u8>) -> Result<(), Error> {
        if self.arms.len() > u8::MAX as usize {
            return Err(Error::TooManyMatchArms);
        }

        out.push(self.ctx);
        out.push(self.cast_u8 as u8);
        out.push(self.arms.len() as u8);

        for arm in &self.arms {
            arm.pattern.encode(out);
            constants.encode_number(arm.value, out);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MatchArm {
    pattern: MatchPattern,
    value: u32,
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

impl MatchPattern {
    fn encode(self, out: &mut Vec<u8>) {
        match self {
            Self::Exact(v) => {
                out.push(0);
                out.push(v);
            }
            Self::ToExclusive(v) => {
                out.push(1);
                out.push(v);
            }
            Self::ToInclusive(v) => {
                out.push(2);
                out.push(v);
            }
            Self::RangeExclusive(a, b) => {
                out.push(3);
                out.push(a);
                out.push(b);
            }
            Self::RangeInclusive(a, b) => {
                out.push(4);
                out.push(a);
                out.push(b);
            }
            Self::From(v) => {
                out.push(5);
                out.push(v);
            }
        }
    }
}

#[derive(Clone, Debug)]
struct ChampionPending {
    formulas: Vec<Ast>,
}

#[derive(Clone, Debug)]
struct SparsePending {
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

/// Build-time database builder. It works only with numeric owner indices; API-specific ID enums stay
/// outside this crate.
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

    pub fn push_champion(
        &mut self,
        owner_index: u16,
        formulas: &[FormulaSource<'_>],
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

        let mut parsed = Vec::with_capacity(formulas.len());
        for formula in formulas {
            parsed.push((formula.local, self.parse_formula(formula.source, refs)?));
        }
        parsed.sort_by_key(|(local, _)| *local);

        for (index, (local, _)) in parsed.iter().enumerate() {
            if *local as usize != index {
                return Err(Error::ChampionLocalsMustBeDense);
            }
        }

        self.champions[owner_index as usize] = Some(ChampionPending {
            formulas: parsed.into_iter().map(|(_, ast)| ast).collect(),
        });

        Ok(())
    }

    pub fn push_item(
        &mut self,
        owner_index: u16,
        formulas: &[FormulaSource<'_>],
        refs: &HashMap<String, u8>,
    ) -> Result<(), Error> {
        self.push_sparse(EntityKind::Item, owner_index, formulas, refs)
    }

    pub fn push_rune(
        &mut self,
        owner_index: u16,
        formulas: &[FormulaSource<'_>],
        refs: &HashMap<String, u8>,
    ) -> Result<(), Error> {
        self.push_sparse(EntityKind::Rune, owner_index, formulas, refs)
    }

    /// Kept public for callers that already have an `EntityKind`, but prefer `push_item`/`push_rune`
    /// when possible.
    pub fn push_sparse(
        &mut self,
        kind: EntityKind,
        owner_index: u16,
        formulas: &[FormulaSource<'_>],
        refs: &HashMap<String, u8>,
    ) -> Result<(), Error> {
        let table_len = match kind {
            EntityKind::Item => self.items.len(),
            EntityKind::Rune => self.runes.len(),
            EntityKind::Champion => {
                return Err(Error::Corrupt("push_sparse cannot be used for champions"));
            }
        };

        if owner_index as usize >= table_len {
            return Err(Error::OwnerOutOfRange {
                kind,
                owner: owner_index,
                count: table_len as u16,
            });
        }

        let already_present = match kind {
            EntityKind::Item => self.items[owner_index as usize].is_some(),
            EntityKind::Rune => self.runes[owner_index as usize].is_some(),
            EntityKind::Champion => unreachable!(),
        };

        if already_present {
            return Err(Error::DuplicateOwner {
                kind,
                owner: owner_index,
            });
        }

        let mut seen = [false; 4];
        let mut parsed = Vec::with_capacity(formulas.len());
        let mut mask = 0u8;

        for formula in formulas {
            let slot =
                DamageSlot::from_u8(formula.local).ok_or(Error::Corrupt("formula.local > 4"))?;
            let index = slot as usize;

            if seen[index] {
                return Err(Error::DuplicateLocal(slot as u8));
            }

            seen[index] = true;
            mask |= slot.bit();
            parsed.push((slot, self.parse_formula(formula.source, refs)?));
        }

        parsed.sort_by_key(|(slot, _)| *slot as u8);
        DamageSlot::validate_mask(mask)?;

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
        let tokens = Lexer::new(source).tokenize()?;
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
            let token = parser.peek().unwrap();
            return Err(token.error("unexpected token after end of formula"));
        }

        Ok(expr.ast)
    }

    /// Finalizes pools/tables and returns the custom little-endian binary representation.
    pub fn finish(self) -> Result<Vec<u8>, Error> {
        let champion_count =
            u16::try_from(self.champions.len()).map_err(|_| Error::TooManyOwners)?;
        let item_count = u16::try_from(self.items.len()).map_err(|_| Error::TooManyOwners)?;
        let rune_count = u16::try_from(self.runes.len()).map_err(|_| Error::TooManyOwners)?;

        let mut formulas = Vec::<Ast>::new();
        let mut champion_bases = Vec::with_capacity(self.champions.len());

        for pending in self.champions {
            champion_bases.push(Self::formula_index(formulas.len())?);
            if let Some(pending) = pending {
                formulas.extend(pending.formulas);
            }
        }

        let mut item_entries = Vec::with_capacity(self.items.len());
        for pending in self.items {
            let first_formula = Self::formula_index(formulas.len())?;
            let mask = pending.as_ref().map_or(0, |p| p.mask);
            item_entries.push(EntityEntry {
                first_formula,
                mask,
            });
            if let Some(pending) = pending {
                formulas.extend(pending.formulas.into_iter().map(|(_, ast)| ast));
            }
        }

        let mut rune_entries = Vec::with_capacity(self.runes.len());
        for pending in self.runes {
            let first_formula = Self::formula_index(formulas.len())?;
            let mask = pending.as_ref().map_or(0, |p| p.mask);
            rune_entries.push(EntityEntry {
                first_formula,
                mask,
            });
            if let Some(pending) = pending {
                formulas.extend(pending.formulas.into_iter().map(|(_, ast)| ast));
            }
        }

        let formula_count = Self::formula_index(formulas.len())?;
        let match_count =
            u16::try_from(self.match_defs.len()).map_err(|_| Error::TooManyMatches)?;

        let mut frequency = HashMap::<u32, u32>::new();
        for ast in &formulas {
            ast.count_numbers(&mut frequency);
        }
        for definition in &self.match_defs {
            definition.count_numbers(&mut frequency);
        }

        let constants = ConstantPool::new(frequency);
        let constant_count = constants.values.len() as u16;

        let mut match_data = Vec::new();
        let mut match_offsets = Vec::with_capacity(self.match_defs.len() + 1);
        for definition in &self.match_defs {
            match_offsets.push(match_data.len() as u32);
            definition.encode(&constants, &mut match_data)?;
        }
        match_offsets.push(match_data.len() as u32);

        let mut formula_data = Vec::new();
        let mut formula_offsets = Vec::with_capacity(formulas.len() + 1);
        for ast in &formulas {
            formula_offsets.push(formula_data.len() as u32);
            ast.encode(&constants, &mut formula_data);
        }
        formula_offsets.push(formula_data.len() as u32);

        let match_data_len = u32::try_from(match_data.len())
            .map_err(|_| Error::Corrupt("match data exceeds u32"))?;

        let mut out = Vec::with_capacity(
            HEADER_LEN
                + constants.values.len() * 4
                + match_offsets.len() * 4
                + champion_bases.len() * 2
                + item_entries.len() * 3
                + rune_entries.len() * 3
                + formula_offsets.len() * 4
                + match_data.len()
                + formula_data.len(),
        );

        out.extend_from_slice(&MAGIC);
        out.push(VERSION);
        out.push(0);
        out.push_u16_le(HEADER_LEN as u16);
        out.push_u16_le(constant_count);
        out.push_u16_le(match_count);
        out.push_u16_le(champion_count);
        out.push_u16_le(item_count);
        out.push_u16_le(rune_count);
        out.push_u16_le(formula_count);
        out.push_u32_le(match_data_len);
        debug_assert_eq!(out.len(), HEADER_LEN);

        for bits in constants.values {
            out.push_u32_le(bits);
        }
        for offset in match_offsets {
            out.push_u32_le(offset);
        }
        for base in champion_bases {
            out.push_u16_le(base);
        }
        for entry in item_entries {
            entry.encode(&mut out);
        }
        for entry in rune_entries {
            entry.encode(&mut out);
        }
        for offset in formula_offsets {
            out.push_u32_le(offset);
        }
        out.extend_from_slice(&match_data);
        out.extend_from_slice(&formula_data);

        Ok(out)
    }

    #[inline]
    fn formula_index(len: usize) -> Result<u16, Error> {
        u16::try_from(len).map_err(|_| Error::TooManyFormulas)
    }
}

#[derive(Debug, Clone, Copy)]
struct EntityEntry {
    first_formula: u16,
    mask: u8,
}

impl EntityEntry {
    fn encode(self, out: &mut Vec<u8>) {
        out.push_u16_le(self.first_formula);
        out.push(self.mask);
    }
}

struct ConstantPool {
    values: Vec<u32>,
    ids: HashMap<u32, u8>,
}

impl ConstantPool {
    fn new(freq: HashMap<u32, u32>) -> Self {
        let mut candidates = freq
            .into_iter()
            .filter(|(bits, count)| *count >= 2 && ImmediateNumber::from_bits(*bits).is_none())
            .map(|(bits, count)| {
                // Inline: 5*n. Pooled: 2*n + 4. Saving = 3*n - 4.
                let saving = count.saturating_mul(3).saturating_sub(4);
                (saving, count, bits)
            })
            .collect::<Vec<_>>();

        candidates.sort_by(|a, b| {
            b.0.cmp(&a.0)
                .then_with(|| b.1.cmp(&a.1))
                .then_with(|| a.2.cmp(&b.2))
        });
        candidates.truncate(256);

        let values = candidates
            .iter()
            .map(|(_, _, bits)| *bits)
            .collect::<Vec<_>>();
        let ids = values
            .iter()
            .enumerate()
            .map(|(index, bits)| (*bits, index as u8))
            .collect();

        Self { values, ids }
    }

    fn encode_number(&self, bits: u32, out: &mut Vec<u8>) {
        if let Some(immediate) = ImmediateNumber::from_bits(bits) {
            immediate.encode(out);
        } else if let Some(&id) = self.ids.get(&bits) {
            out.push(OP_NUM_CONST);
            out.push(id);
        } else {
            out.push(OP_NUM_F32);
            out.push_u32_le(bits);
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ImmediateNumber {
    U8(u8),
    I8(i8),
}

impl ImmediateNumber {
    fn from_bits(bits: u32) -> Option<Self> {
        let value = f32::from_bits(bits);
        if !value.is_finite() || value.fract() != 0.0 {
            return None;
        }
        if (0.0..=255.0).contains(&value) {
            return Some(Self::U8(value as u8));
        }
        if (-128.0..0.0).contains(&value) {
            return Some(Self::I8(value as i8));
        }
        None
    }

    fn encode(self, out: &mut Vec<u8>) {
        match self {
            Self::U8(value) => {
                out.push(OP_NUM_U8);
                out.push(value);
            }
            Self::I8(value) => {
                out.push(OP_NUM_I8);
                out.push(value as u8);
            }
        }
    }
}

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

impl Token {
    fn error(&self, message: impl Into<String>) -> Error {
        Error::Parse {
            line: self.line,
            column: self.column,
            message: message.into(),
        }
    }

    fn indent_units(&self) -> Result<u8, Error> {
        if self.column % 2 != 0 || self.column / 2 > u8::MAX as u32 {
            return Err(Error::InvalidIndent {
                line: self.line,
                column: self.column,
            });
        }
        Ok((self.column / 2) as u8)
    }
}

struct Lexer<'a> {
    source: &'a str,
    bytes: &'a [u8],
    out: Vec<Token>,
    index: usize,
    line: u32,
    column: u32,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            bytes: source.as_bytes(),
            out: Vec::new(),
            index: 0,
            line: 1,
            column: 0,
        }
    }

    fn tokenize(mut self) -> Result<Vec<Token>, Error> {
        while self.index < self.bytes.len() {
            match self.bytes[self.index] {
                b' ' => {
                    self.index += 1;
                    self.column += 1;
                }
                b'\t' => {
                    return Err(Error::Parse {
                        line: self.line,
                        column: self.column,
                        message: "tabs are not supported; rustfmt input should use spaces".into(),
                    });
                }
                b'\n' => self.newline(1),
                b'\r' => {
                    let width = usize::from(self.bytes.get(self.index + 1) == Some(&b'\n')) + 1;
                    self.newline(width);
                }
                b'0'..=b'9' => self.number(),
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.ident(),
                b'+' => self.single(TokenKind::Plus),
                b'-' => self.single(TokenKind::Minus),
                b'*' => self.single(TokenKind::Star),
                b'/' => self.single(TokenKind::Slash),
                b'(' => self.single(TokenKind::LParen),
                b')' => self.single(TokenKind::RParen),
                b'{' => self.single(TokenKind::LBrace),
                b'}' => self.single(TokenKind::RBrace),
                b',' => self.single(TokenKind::Comma),
                b'.' => self.dot(),
                b'=' if self.bytes.get(self.index + 1) == Some(&b'>') => {
                    self.push(TokenKind::FatArrow, 2)
                }
                other => {
                    return Err(Error::Parse {
                        line: self.line,
                        column: self.column,
                        message: format!("unsupported byte `{}`", other as char),
                    });
                }
            }
        }
        Ok(self.out)
    }

    fn newline(&mut self, width: usize) {
        self.index += width;
        self.line += 1;
        self.column = 0;
    }

    fn single(&mut self, kind: TokenKind) {
        self.push(kind, 1);
    }

    fn push(&mut self, kind: TokenKind, width: usize) {
        self.out.push(Token {
            kind,
            line: self.line,
            column: self.column,
        });
        self.index += width;
        self.column += width as u32;
    }

    fn dot(&mut self) {
        if self.bytes.get(self.index + 1) == Some(&b'.') {
            if self.bytes.get(self.index + 2) == Some(&b'=') {
                self.push(TokenKind::RangeEq, 3);
            } else {
                self.push(TokenKind::Range, 2);
            }
        } else {
            self.single(TokenKind::Dot);
        }
    }

    fn ident(&mut self) {
        let start = self.index;
        let start_column = self.column;
        while self.index < self.bytes.len()
            && (self.bytes[self.index].is_ascii_alphanumeric() || self.bytes[self.index] == b'_')
        {
            self.index += 1;
            self.column += 1;
        }
        self.out.push(Token {
            kind: TokenKind::Ident(self.source[start..self.index].to_owned()),
            line: self.line,
            column: start_column,
        });
    }

    fn number(&mut self) {
        let start = self.index;
        let start_column = self.column;

        while self.index < self.bytes.len()
            && (self.bytes[self.index].is_ascii_digit() || self.bytes[self.index] == b'_')
        {
            self.index += 1;
            self.column += 1;
        }

        if self.index < self.bytes.len()
            && self.bytes[self.index] == b'.'
            && self.bytes.get(self.index + 1) != Some(&b'.')
        {
            self.index += 1;
            self.column += 1;
            while self.index < self.bytes.len()
                && (self.bytes[self.index].is_ascii_digit() || self.bytes[self.index] == b'_')
            {
                self.index += 1;
                self.column += 1;
            }
        }

        if self.index < self.bytes.len() && matches!(self.bytes[self.index], b'e' | b'E') {
            self.index += 1;
            self.column += 1;
            if self.index < self.bytes.len() && matches!(self.bytes[self.index], b'+' | b'-') {
                self.index += 1;
                self.column += 1;
            }
            while self.index < self.bytes.len()
                && (self.bytes[self.index].is_ascii_digit() || self.bytes[self.index] == b'_')
            {
                self.index += 1;
                self.column += 1;
            }
        }

        if self.source[self.index..].starts_with("f32")
            || self.source[self.index..].starts_with("f64")
        {
            self.index += 3;
            self.column += 3;
        }

        self.out.push(Token {
            kind: TokenKind::Number(self.source[start..self.index].to_owned()),
            line: self.line,
            column: start_column,
        });
    }
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
        let token = self.tokens.get(self.pos).cloned();
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    fn parse_expression(&mut self, min_precedence: u8) -> Result<Parsed, Error> {
        let mut left = self.parse_unary()?;

        loop {
            let Some(token) = self.peek() else { break };
            let (op, precedence) = match token.kind {
                TokenKind::Plus => (BinaryOp::Add, 1),
                TokenKind::Minus => (BinaryOp::Sub, 1),
                TokenKind::Star => (BinaryOp::Mul, 2),
                TokenKind::Slash => (BinaryOp::Div, 2),
                _ => break,
            };

            if precedence < min_precedence {
                break;
            }

            let operator = self.bump().unwrap();
            let right = self.parse_expression(precedence + 1)?;
            let layout = if operator.line > left.end_line {
                OpLayout {
                    kind: BreakKind::BeforeOperator,
                    indent_units: operator.indent_units()?,
                }
            } else if right.start_line > operator.line {
                OpLayout {
                    kind: BreakKind::AfterOperator,
                    indent_units: Token {
                        kind: TokenKind::Plus,
                        line: right.start_line,
                        column: right.start_col,
                    }
                    .indent_units()?,
                }
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
            TokenKind::Number(source) => {
                let value = Self::parse_f32(&source).map_err(|message| Error::Parse {
                    line: token.line,
                    column: token.column,
                    message,
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
                let close = self.expect(TokenKind::RParen, "expected `)`")?;
                Ok(Parsed {
                    ast: Ast::Group(Box::new(inner.ast)),
                    start_line: token.line,
                    start_col: token.column,
                    end_line: close.line,
                })
            }
            TokenKind::Ident(ref value) if value == "match" => self.parse_match(token),
            TokenKind::Ident(ref value) if value == "ctx" => self.parse_ctx(token),
            TokenKind::Ident(name) => self.parse_function_call(token, name),
            _ => Err(token.error(
                "expected number, ctx field, function call, match, or parenthesized expression",
            )),
        }
    }

    fn parse_ctx(&mut self, start: Token) -> Result<Parsed, Error> {
        self.expect(TokenKind::Dot, "expected `.` after `ctx`")?;
        let field_token = self
            .bump()
            .ok_or_else(|| start.error("expected ctx field"))?;
        let field = match &field_token.kind {
            TokenKind::Ident(value) => value.clone(),
            _ => return Err(field_token.error("expected ctx field identifier")),
        };
        let id = (self.ctx_resolver)(&field).ok_or_else(|| Error::UnknownCtxVar(field.clone()))?;

        Ok(Parsed {
            ast: Ast::Ctx(id),
            start_line: start.line,
            start_col: start.column,
            end_line: field_token.line,
        })
    }

    fn parse_function_call(&mut self, start: Token, name: String) -> Result<Parsed, Error> {
        self.expect(TokenKind::LParen, "expected `(` after function name")?;
        let argument = self
            .bump()
            .ok_or_else(|| start.error("expected function argument"))?;

        if !matches!(&argument.kind, TokenKind::Ident(value) if value == "ctx") {
            return Err(argument.error("local function calls must have exactly `ctx` as argument"));
        }

        if matches!(self.peek().map(|t| &t.kind), Some(TokenKind::Comma)) {
            self.bump();
        }

        let close = self.expect(TokenKind::RParen, "expected `)` after `ctx`")?;
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
        let ctx_token = self
            .bump()
            .ok_or_else(|| start.error("expected match scrutinee"))?;
        if !matches!(&ctx_token.kind, TokenKind::Ident(value) if value == "ctx") {
            return Err(ctx_token.error("match scrutinee must start with `ctx`"));
        }

        self.expect(TokenKind::Dot, "expected `.` in match scrutinee")?;
        let field_token = self
            .bump()
            .ok_or_else(|| ctx_token.error("expected ctx field"))?;
        let field = match &field_token.kind {
            TokenKind::Ident(value) => value.clone(),
            _ => return Err(field_token.error("expected ctx field identifier")),
        };
        let ctx = (self.ctx_resolver)(&field).ok_or_else(|| Error::UnknownCtxVar(field.clone()))?;

        let mut cast_u8 = false;
        if matches!(self.peek().map(|t| &t.kind), Some(TokenKind::Ident(value)) if value == "as") {
            self.bump();
            let ty = self
                .bump()
                .ok_or_else(|| field_token.error("expected `u8` after `as`"))?;
            if matches!(&ty.kind, TokenKind::Ident(value) if value == "u8") {
                cast_u8 = true;
            } else {
                return Err(ty.error("only `as u8` is supported in match scrutinee"));
            }
        }

        let open = self.expect(TokenKind::LBrace, "expected `{` after match scrutinee")?;
        let first_arm = self.peek().cloned();
        let mut arms = Vec::new();

        while !matches!(self.peek().map(|t| &t.kind), Some(TokenKind::RBrace)) {
            if self.is_eof() {
                return Err(start.error("unterminated match"));
            }

            let pattern = self.parse_match_pattern()?;
            self.expect(TokenKind::FatArrow, "expected `=>` in match arm")?;
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
                return Err(self
                    .peek()
                    .unwrap()
                    .error("expected `,` or `}` after match arm"));
            }
        }

        let close = self.expect(TokenKind::RBrace, "expected `}`")?;
        let layout = if close.line > open.line {
            let arm = first_arm.ok_or_else(|| start.error("empty match is unsupported"))?;
            MatchLayout::Block {
                arm_indent_units: arm.indent_units()?,
                close_indent_units: close.indent_units()?,
            }
        } else {
            MatchLayout::Inline
        };

        let definition = MatchDef { ctx, cast_u8, arms };
        let id = if let Some(&id) = self.match_ids.get(&definition) {
            id
        } else {
            if self.match_defs.len() >= 256 {
                return Err(Error::TooManyMatches);
            }
            let id = self.match_defs.len() as u8;
            self.match_defs.push(definition.clone());
            self.match_ids.insert(definition, id);
            id
        };

        Ok(Parsed {
            ast: Ast::Match { id, layout },
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
            TokenKind::Range => Ok(MatchPattern::ToExclusive(
                self.expect_pattern_u8("expected upper bound after `..`")?,
            )),
            TokenKind::RangeEq => Ok(MatchPattern::ToInclusive(
                self.expect_pattern_u8("expected upper bound after `..=`")?,
            )),
            TokenKind::Number(source) => {
                let start = Self::parse_pattern_u8(&source).map_err(|message| Error::Parse {
                    line: first.line,
                    column: first.column,
                    message,
                })?;

                match self.peek().map(|t| &t.kind) {
                    Some(TokenKind::Range) => {
                        self.bump();
                        if matches!(self.peek().map(|t| &t.kind), Some(TokenKind::FatArrow)) {
                            Ok(MatchPattern::From(start))
                        } else {
                            Ok(MatchPattern::RangeExclusive(
                                start,
                                self.expect_pattern_u8("expected range end")?,
                            ))
                        }
                    }
                    Some(TokenKind::RangeEq) => {
                        self.bump();
                        Ok(MatchPattern::RangeInclusive(
                            start,
                            self.expect_pattern_u8("expected inclusive range end")?,
                        ))
                    }
                    _ => Ok(MatchPattern::Exact(start)),
                }
            }
            _ => Err(first.error("unsupported match pattern")),
        }
    }

    fn expect_pattern_u8(&mut self, message: &str) -> Result<u8, Error> {
        let token = self.bump().ok_or_else(|| Error::Parse {
            line: 1,
            column: 0,
            message: message.into(),
        })?;
        let source = match &token.kind {
            TokenKind::Number(source) => source,
            _ => return Err(token.error(message)),
        };
        Self::parse_pattern_u8(source).map_err(|message| Error::Parse {
            line: token.line,
            column: token.column,
            message,
        })
    }

    fn parse_match_rhs_number(&mut self) -> Result<f32, Error> {
        let negative = matches!(self.peek().map(|t| &t.kind), Some(TokenKind::Minus));
        if negative {
            self.bump();
        }

        let token = self.bump().ok_or_else(|| Error::Parse {
            line: 1,
            column: 0,
            message: "expected numeric match RHS".into(),
        })?;
        let source = match &token.kind {
            TokenKind::Number(source) => source,
            _ => return Err(token.error("match RHS must be a number")),
        };
        let mut value = Self::parse_f32(source).map_err(|message| Error::Parse {
            line: token.line,
            column: token.column,
            message,
        })?;
        if negative {
            value = -value;
        }
        Ok(value)
    }

    fn expect(&mut self, expected: TokenKind, message: &str) -> Result<Token, Error> {
        let token = self.bump().ok_or_else(|| Error::Parse {
            line: 1,
            column: 0,
            message: message.into(),
        })?;
        if std::mem::discriminant(&token.kind) != std::mem::discriminant(&expected) {
            return Err(token.error(message));
        }
        Ok(token)
    }

    fn parse_f32(source: &str) -> Result<f32, String> {
        let mut cleaned = source.replace('_', "");
        if cleaned.ends_with("f32") || cleaned.ends_with("f64") {
            cleaned.truncate(cleaned.len() - 3);
        }
        cleaned
            .parse::<f32>()
            .map_err(|_| format!("invalid f32 literal `{source}`"))
    }

    fn parse_pattern_u8(source: &str) -> Result<u8, String> {
        let value = Self::parse_f32(source)?;
        if !value.is_finite() || value.fract() != 0.0 || !(0.0..=255.0).contains(&value) {
            return Err(format!("match pattern bound `{source}` is not a u8"));
        }
        Ok(value as u8)
    }
}

trait ByteVecExt {
    fn push_u16_le(&mut self, value: u16);
    fn push_u32_le(&mut self, value: u32);
}

impl ByteVecExt for Vec<u8> {
    #[inline]
    fn push_u16_le(&mut self, value: u16) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    #[inline]
    fn push_u32_le(&mut self, value: u32) {
        self.extend_from_slice(&value.to_le_bytes());
    }
}
