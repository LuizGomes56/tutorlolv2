//! A `Hole` is one piece of dynamic data inside a template invocation. Its
//! wire shape (`Text`, `Index8`, `Index16`) is fixed per position by the
//! template that owns it — the stream carries no tag distinguishing them,
//! since encoder and decoder are always compiled from the same template
//! table and can never disagree on the schema.

use crate::io::Writer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hole {
    /// Arbitrary literal text, stored in the aux buffer behind a `u8`
    /// length prefix — used when a hole's value isn't drawn from any
    /// known dictionary or domain enum.
    Text(String),
    /// A one-byte index, typically into an existing `Class` dictionary
    /// (e.g. a `Type` name reused inside a template).
    Index8(u8),
    /// A two-byte index, typically the discriminant of a domain enum such
    /// as an item id, resolved back to text via that enum's own
    /// `Debug`/`Display` at render time.
    Index16(u16),
}

impl Hole {
    /// Writes this hole's payload. Which variant to expect is never stored
    /// on the wire — the template's `read_holes` already knows it.
    pub fn write(&self, writer: &mut Writer) {
        match self {
            Hole::Text(text) => writer.write_aux_text(text),
            Hole::Index8(index) => writer.write_u8(*index),
            Hole::Index16(index) => writer.write_u16_le(*index),
        }
    }
}
