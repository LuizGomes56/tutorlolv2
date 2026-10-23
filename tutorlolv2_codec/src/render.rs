use crate::common::{
    DamageSlot, EntityKind, Error, HEADER_LEN, MAGIC, OP_BIN_BASE, OP_BIN_MAX, OP_CTX, OP_GROUP,
    OP_MATCH_BLOCK, OP_MATCH_INLINE, OP_NEG, OP_NUM_CONST, OP_NUM_F32, OP_NUM_I8, OP_NUM_U8,
    OP_REF_LOCAL, VERSION,
};
use crate::common::{ParseError, const_try};
use heck::ToPascalCase;
use std::convert::TryInto;
use std::fmt::{self, Display};

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, strum::EnumString, strum::IntoStaticStr, strum::FromRepr,
)]
pub enum Class {
    Comment,
    String,
    Lifetime,
    Keyword,
    Control,
    Constant,
    Type,
    Primitive,
    Number,
    Boolean,
    Macro,
    Function,
    Variable,
    Bracket1,
    Bracket2,
    Bracket3,
}

impl Class {
    pub const fn bracket(depth: usize) -> Self {
        match depth % 3 {
            0 => Self::Bracket1,
            1 => Self::Bracket2,
            2 => Self::Bracket3,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
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
    pub const fn new(bytes: &'a [u8]) -> Result<Self, ParseError> {
        let mut this = Self {
            bytes,
            constants: 0,
            matches: 0,
            champions: 0,
            items: 0,
            runes: 0,
            formulas: 0,
            match_data_len: 0,
            layout: Layout {
                constant_pool: 0,
                match_offsets: 0,
                champion_bases: 0,
                item_entries: 0,
                rune_entries: 0,
                formula_offsets: 0,
                match_data: 0,
                formula_data: 0,
            },
        };

        const_try!(this.parse());
        Ok(this)
    }

    pub const fn u16_at(&self, offset: usize) -> Result<u16, ParseError> {
        let end = match offset.checked_add(2) {
            Some(end) => end,
            None => return Err(ParseError::Corrupt("offset overflow")),
        };
        if end > self.bytes.len() {
            return Err(ParseError::Corrupt("u16 out of bounds"));
        }
        Ok(u16::from_le_bytes([
            self.bytes[offset],
            self.bytes[offset + 1],
        ]))
    }

    pub const fn u32_at(&self, offset: usize) -> Result<u32, ParseError> {
        let end = match offset.checked_add(4) {
            Some(end) => end,
            None => return Err(ParseError::Corrupt("offset overflow")),
        };
        if end > self.bytes.len() {
            return Err(ParseError::Corrupt("u32 out of bounds"));
        }
        Ok(u32::from_le_bytes([
            self.bytes[offset],
            self.bytes[offset + 1],
            self.bytes[offset + 2],
            self.bytes[offset + 3],
        ]))
    }

    pub const fn parse(&mut self) -> Result<(), ParseError> {
        if self.bytes.len() < HEADER_LEN {
            return Err(ParseError::Corrupt("buffer is shorter than header"));
        }

        if self.bytes[0] != MAGIC[0]
            || self.bytes[1] != MAGIC[1]
            || self.bytes[2] != MAGIC[2]
            || self.bytes[3] != MAGIC[3]
        {
            return Err(ParseError::Corrupt("bad magic"));
        }

        if self.bytes[4] != VERSION {
            return Err(ParseError::Corrupt("unsupported version"));
        }

        if const_try!(self.u16_at(6)) as usize != HEADER_LEN {
            return Err(ParseError::Corrupt("unexpected header length"));
        }

        let constants = const_try!(self.u16_at(8));
        let matches = const_try!(self.u16_at(10));
        let champions = const_try!(self.u16_at(12));
        let items = const_try!(self.u16_at(14));
        let runes = const_try!(self.u16_at(16));
        let formulas = const_try!(self.u16_at(18));
        let match_data_len = const_try!(self.u32_at(20));

        if constants > 256 || matches > 256 {
            return Err(ParseError::Corrupt("u8-indexed pool exceeds 256 entries"));
        }

        let constant_pool = HEADER_LEN;
        let match_offsets = const_try!(Self::checked_add(constant_pool, constants as usize * 4));
        let champion_bases =
            const_try!(Self::checked_add(match_offsets, (matches as usize + 1) * 4));
        let item_entries = const_try!(Self::checked_add(champion_bases, champions as usize * 2));
        let rune_entries = const_try!(Self::checked_add(item_entries, items as usize * 3));
        let formula_offsets = const_try!(Self::checked_add(rune_entries, runes as usize * 3));
        let match_data = const_try!(Self::checked_add(
            formula_offsets,
            (formulas as usize + 1) * 4
        ));
        let formula_data = const_try!(Self::checked_add(match_data, match_data_len as usize));

        if formula_data > self.bytes.len() {
            return Err(ParseError::Corrupt("section layout exceeds buffer"));
        }

        *self = Self {
            bytes: self.bytes,
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

        self.validate_offsets()
    }

    pub const fn champion_formula_id(&self, id: u16, local: u8) -> Option<u16> {
        if id >= self.champions {
            return None;
        }

        let base = match self.u16_at(self.layout.champion_bases + id as usize * 2) {
            Ok(value) => value,
            Err(_) => return None,
        };
        match base.checked_add(local as u16) {
            Some(id) if id < self.formulas => Some(id),
            _ => None,
        }
    }

    pub const fn item_or_rune_formula_id(
        &self,
        kind: EntityKind,
        id: u16,
        slot: DamageSlot,
    ) -> Option<u16> {
        let (count, start) = match kind {
            EntityKind::Item => (self.items, self.layout.item_entries),
            EntityKind::Rune => (self.runes, self.layout.rune_entries),
            EntityKind::Champion => return None,
        };

        if id >= count {
            return None;
        }

        let pos = start + id as usize * 3;
        let first = match self.u16_at(pos) {
            Ok(value) => value,
            Err(_) => return None,
        };
        if pos + 2 >= self.bytes.len() {
            return None;
        }
        let mask = self.bytes[pos + 2];
        let bit = slot.bit();

        if mask & bit == 0 {
            return None;
        }

        let before_mask = if slot as u8 == 0 {
            0
        } else {
            mask & ((1u8 << slot as u8) - 1)
        };

        let rank = before_mask.count_ones() as u16;
        match first.checked_add(rank) {
            Some(id) if id < self.formulas => Some(id),
            _ => None,
        }
    }

    pub const fn formula_bytes(&self, formula_id: u16) -> Option<&'a [u8]> {
        if formula_id >= self.formulas {
            return None;
        }

        let start = match self.formula_offset(formula_id) {
            Ok(value) => value as usize,
            Err(_) => return None,
        };
        let end = match self.formula_offset(formula_id + 1) {
            Ok(value) => value as usize,
            Err(_) => return None,
        };

        if start > end || end > self.bytes.len() - self.layout.formula_data {
            return None;
        }
        Some(
            self.bytes
                .split_at(self.layout.formula_data + start)
                .1
                .split_at(end - start)
                .0,
        )
    }

    pub fn render_formula_plain<FC, FR>(
        &self,
        formula_id: u16,
        mut ctx_name: FC,
        mut local_ref_name: FR,
    ) -> Result<String, Error>
    where
        FC: FnMut(u8) -> String,
        FR: FnMut(u8) -> FnBuilder,
    {
        let mut emit = |out: &mut String, _: Option<Class>, text: &str| out.push_str(text);
        self.render_formula(formula_id, &mut ctx_name, &mut local_ref_name, &mut emit)
    }

    pub fn render_formula_html<FC, FR>(
        &self,
        formula_id: u16,
        mut ctx_name: FC,
        mut local_ref_name: FR,
    ) -> Result<String, Error>
    where
        FC: FnMut(u8) -> String,
        FR: FnMut(u8) -> FnBuilder,
    {
        let mut emit = |out: &mut String, class: Option<Class>, text: &str| match class {
            Some(class) => Highlighter::push_span(out, class, text),
            None => Highlighter::push_escaped(out, text),
        };
        self.render_formula(formula_id, &mut ctx_name, &mut local_ref_name, &mut emit)
    }

    fn render_formula(
        &self,
        formula_id: u16,
        ctx_name: &mut dyn FnMut(u8) -> String,
        local_ref_name: &mut dyn FnMut(u8) -> FnBuilder,
        emit: &mut dyn FnMut(&mut String, Option<Class>, &str),
    ) -> Result<String, Error> {
        let bytes = self
            .formula_bytes(formula_id)
            .ok_or(Error::Corrupt("formula id out of range"))?;

        let mut cursor = Cursor::new(bytes);
        let mut out = String::new();

        let mut renderer = FormulaRenderer {
            db: self,
            ctx_name,
            local_ref_name,
            emit,
            bracket_stack: Vec::new(),
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
        Ok(f32::from_bits(self.u32_at(pos)?))
    }

    fn match_bytes(&self, id: u8) -> Result<&'a [u8], Error> {
        if id as u16 >= self.matches {
            return Err(Error::Corrupt("match id out of range"));
        }

        let start = self.match_offset(id as u16)? as usize;
        let end = self.match_offset(id as u16 + 1)? as usize;

        if end < start || end > self.match_data_len as usize {
            return Err(Error::Corrupt("bad match offset"));
        }

        self.bytes
            .get(self.layout.match_data + start..self.layout.match_data + end)
            .ok_or(Error::Corrupt("match slice out of bounds"))
    }

    const fn match_offset(&self, index: u16) -> Result<u32, ParseError> {
        self.u32_at(self.layout.match_offsets + index as usize * 4)
    }

    const fn formula_offset(&self, index: u16) -> Result<u32, ParseError> {
        self.u32_at(self.layout.formula_offsets + index as usize * 4)
    }

    const fn validate_offsets(&self) -> Result<(), ParseError> {
        let mut last = 0u32;

        let mut index = 0usize;
        while index <= self.matches as usize {
            let value = const_try!(self.match_offset(index as u16));

            if value < last || value > self.match_data_len {
                return Err(ParseError::Corrupt("non-monotonic match offsets"));
            }

            last = value;
            index += 1;
        }

        if last != self.match_data_len {
            return Err(ParseError::Corrupt(
                "match offsets do not end at match_data_len",
            ));
        }

        let formula_data_len = self.bytes.len() - self.layout.formula_data;
        let mut last = 0u32;

        let mut index = 0usize;
        while index <= self.formulas as usize {
            let value = const_try!(self.formula_offset(index as u16));

            if value < last || value as usize > formula_data_len {
                return Err(ParseError::Corrupt("non-monotonic formula offsets"));
            }

            last = value;
            index += 1;
        }

        if last as usize != formula_data_len {
            return Err(ParseError::Corrupt(
                "formula offsets do not end at formula data end",
            ));
        }

        Ok(())
    }

    const fn checked_add(a: usize, b: usize) -> Result<usize, ParseError> {
        match a.checked_add(b) {
            Some(value) => Ok(value),
            None => Err(ParseError::Corrupt("section offset overflow")),
        }
    }
}

pub struct FnBuilder {
    pub fn_struct: String,
    pub fn_type: String,
    pub fn_tag: String,
}

#[derive(Clone, Copy)]
enum MatchRenderLayout {
    Inline,
    Block {
        arm_indent_units: u8,
        close_indent_units: u8,
    },
}

struct FormulaRenderer<'db, 'bytes, 'cb> {
    db: &'db FormulaDb<'bytes>,
    ctx_name: &'cb mut dyn FnMut(u8) -> String,
    local_ref_name: &'cb mut dyn FnMut(u8) -> FnBuilder,
    emit: &'cb mut dyn FnMut(&mut String, Option<Class>, &str),
    bracket_stack: Vec<Class>,
}

impl FormulaRenderer<'_, '_, '_> {
    fn expr(&mut self, cursor: &mut Cursor<'_>, out: &mut String) -> Result<(), Error> {
        let opcode = cursor.u8()?;
        match opcode {
            OP_NUM_U8 => {
                let value = cursor.u8()?;
                self.styled(out, Class::Number, &value.to_string());
            }
            OP_NUM_I8 => {
                let value = cursor.u8()? as i8;
                self.styled(out, Class::Number, &value.to_string());
            }
            OP_NUM_CONST => {
                let value = self.db.constant(cursor.u8()?)?;
                self.styled(out, Class::Number, &Self::format_f32(value));
            }
            OP_NUM_F32 => {
                let value = f32::from_bits(cursor.u32()?);
                self.styled(out, Class::Number, &Self::format_f32(value));
            }
            OP_CTX => {
                let name = (self.ctx_name)(cursor.u8()?);
                self.styled(out, Class::Variable, &name);
            }
            OP_REF_LOCAL => {
                let fn_builder = (self.local_ref_name)(cursor.u8()?);
                self.styled(out, Class::Type, &fn_builder.fn_struct);
                self.raw(out, "::");
                self.styled(out, Class::Function, &fn_builder.fn_type);
                self.open_bracket(out, "(");
                self.styled(out, Class::Constant, &fn_builder.fn_tag.to_pascal_case());
                self.close_bracket(out, ")");
            }
            OP_MATCH_INLINE => {
                let id = cursor.u8()?;
                self.render_match(id, MatchRenderLayout::Inline, out)?;
            }
            OP_MATCH_BLOCK => {
                let id = cursor.u8()?;
                let arm_indent_units = cursor.u8()?;
                let close_indent_units = cursor.u8()?;
                self.render_match(
                    id,
                    MatchRenderLayout::Block {
                        arm_indent_units,
                        close_indent_units,
                    },
                    out,
                )?;
            }
            OP_GROUP => {
                self.open_bracket(out, "(");
                self.expr(cursor, out)?;
                self.close_bracket(out, ")");
            }
            OP_NEG => {
                self.raw(out, "-");
                self.expr(cursor, out)?;
            }
            OP_BIN_BASE..=OP_BIN_MAX => {
                let operator = match opcode & 0x03 {
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
                        self.raw(out, " ");
                        self.raw(out, operator);
                        self.raw(out, " ");
                    }
                    1 => {
                        self.raw(out, "\n");
                        self.indent(out, indent);
                        self.raw(out, operator);
                        self.raw(out, " ");
                    }
                    2 => {
                        self.raw(out, " ");
                        self.raw(out, operator);
                        self.raw(out, "\n");
                        self.indent(out, indent);
                    }
                    _ => return Err(Error::Corrupt("reserved binary break kind")),
                }
                self.expr(cursor, out)?;
            }
            _ => return Err(Error::Corrupt("unknown expression opcode")),
        }
        Ok(())
    }

    fn render_match(
        &mut self,
        id: u8,
        layout: MatchRenderLayout,
        out: &mut String,
    ) -> Result<(), Error> {
        let bytes = self.db.match_bytes(id)?;
        let mut cursor = Cursor::new(bytes);
        let ctx = cursor.u8()?;
        let flags = cursor.u8()?;
        let arm_count = cursor.u8()?;

        self.styled(out, Class::Control, "match");
        self.raw(out, " ");
        let name = (self.ctx_name)(ctx);
        self.styled(out, Class::Variable, &name);

        if flags & 1 != 0 {
            self.raw(out, " ");
            self.styled(out, Class::Keyword, "as");
            self.raw(out, " ");
            self.styled(out, Class::Primitive, "u8");
        }

        self.raw(out, " ");
        self.open_bracket(out, "{");

        match layout {
            MatchRenderLayout::Inline => {
                self.raw(out, " ");
                for index in 0..arm_count {
                    self.render_match_arm(&mut cursor, out)?;
                    if index + 1 != arm_count {
                        self.raw(out, ", ");
                    }
                }
                self.raw(out, " ");
            }
            MatchRenderLayout::Block {
                arm_indent_units,
                close_indent_units,
            } => {
                for _ in 0..arm_count {
                    self.raw(out, "\n");
                    self.indent(out, arm_indent_units);
                    self.render_match_arm(&mut cursor, out)?;
                    self.raw(out, ",");
                }
                self.raw(out, "\n");
                self.indent(out, close_indent_units);
            }
        }

        self.close_bracket(out, "}");
        if !cursor.is_eof() {
            return Err(Error::Corrupt("match has trailing bytes"));
        }
        Ok(())
    }

    fn render_match_arm(&mut self, cursor: &mut Cursor<'_>, out: &mut String) -> Result<(), Error> {
        let pattern = cursor.u8()?;
        self.render_pattern(cursor, out, pattern)?;
        self.raw(out, " => ");
        self.render_encoded_number(cursor, out)
    }

    fn render_pattern(
        &mut self,
        cursor: &mut Cursor<'_>,
        out: &mut String,
        pattern: u8,
    ) -> Result<(), Error> {
        match pattern {
            0 => {
                let value = cursor.u8()?;
                self.styled(out, Class::Number, &value.to_string());
            }
            1 => {
                self.raw(out, "..");
                let value = cursor.u8()?;
                self.styled(out, Class::Number, &value.to_string());
            }
            2 => {
                self.raw(out, "..=");
                let value = cursor.u8()?;
                self.styled(out, Class::Number, &value.to_string());
            }
            3 | 4 => {
                let start = cursor.u8()?;
                let end = cursor.u8()?;
                self.styled(out, Class::Number, &start.to_string());
                self.raw(out, if pattern == 3 { ".." } else { "..=" });
                self.styled(out, Class::Number, &end.to_string());
            }
            5 => {
                let value = cursor.u8()?;
                self.styled(out, Class::Number, &value.to_string());
                self.raw(out, "..");
            }
            _ => return Err(Error::Corrupt("unknown match pattern")),
        }
        Ok(())
    }

    fn render_encoded_number(
        &mut self,
        cursor: &mut Cursor<'_>,
        out: &mut String,
    ) -> Result<(), Error> {
        let text = match cursor.u8()? {
            OP_NUM_U8 => cursor.u8()?.to_string(),
            OP_NUM_I8 => (cursor.u8()? as i8).to_string(),
            OP_NUM_CONST => Self::format_f32(self.db.constant(cursor.u8()?)?),
            OP_NUM_F32 => Self::format_f32(f32::from_bits(cursor.u32()?)),
            _ => return Err(Error::Corrupt("match RHS is not a number opcode")),
        };
        self.styled(out, Class::Number, &text);
        Ok(())
    }

    fn styled(&mut self, out: &mut String, class: Class, text: &str) {
        (self.emit)(out, Some(class), text);
    }

    fn raw(&mut self, out: &mut String, text: &str) {
        (self.emit)(out, None, text);
    }

    fn open_bracket(&mut self, out: &mut String, text: &str) {
        let class = Class::bracket(self.bracket_stack.len());
        self.bracket_stack.push(class);
        self.styled(out, class, text);
    }

    fn close_bracket(&mut self, out: &mut String, text: &str) {
        let class = self.bracket_stack.pop().unwrap_or(Class::Bracket1);
        self.styled(out, class, text);
    }

    fn indent(&mut self, out: &mut String, units: u8) {
        for _ in 0..units as usize * 2 {
            self.raw(out, " ");
        }
    }

    fn format_f32(value: f32) -> String {
        value.to_string()
    }
}

struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    fn is_eof(&self) -> bool {
        self.pos == self.bytes.len()
    }

    fn u8(&mut self) -> Result<u8, Error> {
        let value = *self
            .bytes
            .get(self.pos)
            .ok_or(Error::Corrupt("unexpected end of bytecode"))?;
        self.pos += 1;
        Ok(value)
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

#[derive(Debug, Clone, Copy)]
pub enum Bracket {
    LParen,
    RParen,
    LCurly,
    RCurly,
    LBracket,
    RBracket,
}

impl Bracket {
    const fn is_open(self) -> bool {
        matches!(self, Self::LParen | Self::LCurly | Self::LBracket)
    }
}

impl Display for Bracket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::LParen => "(",
            Self::RParen => ")",
            Self::LCurly => "{",
            Self::RCurly => "}",
            Self::LBracket => "[",
            Self::RBracket => "]",
        })
    }
}

#[derive(Default)]
pub struct Highlighter {
    inner: String,
    bracket_stack: Vec<Class>,
}

impl Highlighter {
    const MAX_LINE_WIDTH: usize = 48;
    /// Display width of the tab that indents each struct field.
    const INDENT_WIDTH: usize = 4;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_fragment(self) -> String {
        self.inner
    }

    pub fn into_document(self, title: &str, css: &str) -> String {
        let mut out = String::with_capacity(self.inner.len() + css.len() + 128);
        out.push_str("<html><head><title>");
        Self::push_escaped(&mut out, title);
        out.push_str("</title><style>");
        out.push_str(css);
        out.push_str("</style></head><body><pre>");
        out.push_str(&self.inner);
        out.push_str("</pre></body></html>");
        out
    }

    pub fn push(&mut self, class: Class, value: impl Display) -> &mut Self {
        Self::push_span(&mut self.inner, class, &value.to_string());
        self
    }

    /// Builds one standalone highlighted span. Useful when an API adapter needs a small
    /// pre-highlighted fragment such as a comment body.
    pub fn span(class: Class, value: impl Display) -> String {
        let mut out = String::new();
        Self::push_span(&mut out, class, &value.to_string());
        out
    }

    pub fn raw(&mut self, value: &str) -> &mut Self {
        self.inner.push_str(value);
        self
    }

    pub fn text(&mut self, value: &str) -> &mut Self {
        Self::push_escaped(&mut self.inner, value);
        self
    }

    pub fn string(&mut self, value: impl Display) -> &mut Self {
        let value = format!("\"{value}\"");
        self.push(Class::String, value)
    }

    pub fn bracket(&mut self, bracket: Bracket) -> &mut Self {
        let class = if bracket.is_open() {
            let class = Class::bracket(self.bracket_stack.len());
            self.bracket_stack.push(class);
            class
        } else {
            self.bracket_stack.pop().unwrap_or(Class::Bracket1)
        };
        self.push(class, bracket)
    }

    pub fn field(&mut self, name: &str, class: Class, value: impl Display) -> &mut Self {
        self.push(Class::Variable, name).raw(": ");

        if class == Class::String {
            self.string(value);
        } else {
            self.push(class, value);
        }

        self.field_end()
    }

    /// Adds a field whose value is already highlighted HTML generated by this crate.
    pub fn html_field(&mut self, name: &str, value_html: &str) -> &mut Self {
        self.push(Class::Variable, name)
            .raw(": ")
            .raw(value_html)
            .field_end()
    }

    pub fn array_field<T: Display>(&mut self, name: &str, class: Class, values: &[T]) -> &mut Self {
        let values = values.iter().map(ToString::to_string).collect::<Vec<_>>();
        let width = values.iter().map(|v| v.chars().count()).sum();

        self.list_field(name, &values, width, |h, value| {
            h.push(class, value);
        })
    }

    pub fn tuple_field<T: Display, U: Display>(
        &mut self,
        name: &str,
        class: Class,
        values: &[(T, U)],
    ) -> &mut Self {
        if values.is_empty() {
            return self;
        }

        let values = values
            .iter()
            .map(|(field, value)| (field.to_string().to_pascal_case(), value.to_string()))
            .collect::<Vec<_>>();

        let width = values
            .iter()
            .map(|(field, value)| field.chars().count() + value.chars().count() + 2)
            .sum();

        self.list_field(name, &values, width, |h, (field, value)| {
            h.push(Class::Type, field)
                .bracket(Bracket::LParen)
                .push(class, value)
                .bracket(Bracket::RParen);
        })
    }

    /// Renders `name: [a, b],` on a single line when the whole line, indentation and
    /// trailing comma included, fits in [`Self::MAX_LINE_WIDTH`]. Otherwise each item
    /// goes on its own line. `width` is the sum of the plain-text width of every item.
    fn list_field<T>(
        &mut self,
        name: &str,
        values: &[T],
        width: usize,
        mut item: impl FnMut(&mut Self, &T),
    ) -> &mut Self {
        // `\t` + `name: ` + `[` + items + `, ` separators + `]` + `,`
        let line_width = Self::INDENT_WIDTH
            + name.len()
            + 2
            + 1
            + width
            + values.len().saturating_sub(1) * 2
            + 1
            + 1;

        self.push(Class::Variable, name)
            .raw(": ")
            .bracket(Bracket::LBracket);

        if line_width <= Self::MAX_LINE_WIDTH {
            for (index, value) in values.iter().enumerate() {
                if index != 0 {
                    self.raw(", ");
                }
                item(self, value);
            }
        } else {
            for value in values {
                self.raw("\n\t\t");
                item(self, value);
                self.raw(",");
            }
            self.raw("\n\t");
        }

        self.bracket(Bracket::RBracket).field_end()
    }

    pub fn field_end(&mut self) -> &mut Self {
        self.raw(",\n\t")
    }

    pub fn pop(&mut self) -> &mut Self {
        self.inner.pop();
        self
    }

    pub fn global_struct(&mut self, name: &str) -> &mut Self {
        self.push(Class::Type, name)
            .raw(" ")
            .bracket(Bracket::LCurly)
            .raw("\n\t")
    }

    pub fn finish_struct(&mut self) -> &mut Self {
        if self.inner.ends_with(",\n\t") {
            self.inner.truncate(self.inner.len() - 3);
            self.inner.push('\n');
        }
        self.bracket(Bracket::RCurly)
    }

    pub fn function(
        &mut self,
        owner: impl Display,
        function: impl Display,
        tag: impl Display,
        body_html: &str,
    ) -> &mut Self {
        self.push(Class::Keyword, "fn")
            .raw(" ")
            .push(Class::Type, owner)
            .raw("::")
            .push(Class::Function, function)
            .bracket(Bracket::LParen)
            .push(Class::Constant, tag.to_string().to_pascal_case())
            .bracket(Bracket::RParen)
            .raw(" ")
            .bracket(Bracket::LCurly)
            .raw("\n\t")
            .raw(body_html)
            .raw("\n")
            .bracket(Bracket::RCurly)
    }

    pub(crate) fn push_span(out: &mut String, class: Class, text: &str) {
        out.push_str("<span class=\"C");
        out.push_str(&(class as u8).to_string());
        out.push_str("\">");
        Self::push_escaped(out, text);
        out.push_str("</span>");
    }

    pub(crate) fn push_escaped(out: &mut String, text: &str) {
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
}
