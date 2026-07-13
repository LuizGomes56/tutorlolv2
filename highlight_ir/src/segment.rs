//! `Segment` is the top-level unit of the `.ir` stream: one colored span,
//! one template invocation, or one uncolored literal run of text.

use crate::class::Class;
use crate::dictionary::Dictionary;
use crate::hole::Hole;
use crate::io::{Cursor, Writer};
use crate::template::TemplateRegistry;

const TEMPLATE_TAG: u8 = 0xFE;
const LITERAL_TAG: u8 = 0xFF;

/// What a colored token carries: either a compact dictionary index, or the
/// literal text for a token the dictionary doesn't know.
#[derive(Debug, Clone)]
pub enum TokenPayload {
    Known(u8),
    Unknown(String),
}

impl TokenPayload {
    /// Mirrors `read`'s branching exactly: a dictionary class needs the
    /// `0xFF` sentinel before its fallback text so the reader can tell
    /// "unknown" apart from a real index; a class with no dictionary at
    /// all never had an index to distinguish from, so no sentinel is
    /// written or expected.
    fn write(&self, writer: &mut Writer, class: Class) {
        match self {
            TokenPayload::Known(index) => writer.write_u8(*index),
            TokenPayload::Unknown(text) => {
                if class.has_dictionary() {
                    writer.write_u8(LITERAL_TAG);
                }
                writer.write_aux_text(text);
            }
        }
    }

    /// `class` decides how to read the payload: classes without a
    /// dictionary are always literal text; classes with one read an index
    /// byte first, falling back to literal text on the `0xFF` sentinel.
    fn read(cursor: &mut Cursor, class: Class) -> TokenPayload {
        if !class.has_dictionary() {
            return TokenPayload::Unknown(cursor.read_aux_text().to_string());
        }
        match cursor.read_u8() {
            LITERAL_TAG => TokenPayload::Unknown(cursor.read_aux_text().to_string()),
            index => TokenPayload::Known(index),
        }
    }

    fn resolve<'a>(&'a self, class: Class, dictionary: &'a Dictionary) -> &'a str {
        match self {
            TokenPayload::Known(index) => dictionary.word_at(class, *index),
            TokenPayload::Unknown(text) => text,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Segment {
    Colored { class: Class, payload: TokenPayload },
    Template { id: u8, holes: Vec<Hole> },
    Literal(String),
}

impl Segment {
    pub fn colored_known(class: Class, index: u8) -> Self {
        Segment::Colored { class, payload: TokenPayload::Known(index) }
    }

    pub fn colored_unknown(class: Class, text: impl Into<String>) -> Self {
        Segment::Colored { class, payload: TokenPayload::Unknown(text.into()) }
    }

    pub fn literal(text: impl Into<String>) -> Self {
        Segment::Literal(text.into())
    }

    pub fn template(id: u8, holes: Vec<Hole>) -> Self {
        Segment::Template { id, holes }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.count_segment();
        match self {
            Segment::Colored { class, payload } => {
                writer.write_u8(*class as u8);
                payload.write(writer, *class);
            }
            Segment::Template { id, holes } => {
                writer.write_u8(TEMPLATE_TAG);
                writer.write_u8(*id);
                for hole in holes {
                    hole.write(writer);
                }
            }
            Segment::Literal(text) => {
                writer.write_u8(LITERAL_TAG);
                writer.write_aux_text(text);
            }
        }
    }

    pub fn read(cursor: &mut Cursor, templates: &TemplateRegistry) -> Segment {
        match cursor.read_u8() {
            TEMPLATE_TAG => {
                let id = cursor.read_u8();
                let holes = templates.get(id).read_holes(cursor);
                Segment::Template { id, holes }
            }
            LITERAL_TAG => Segment::Literal(cursor.read_aux_text().to_string()),
            byte => {
                let class = Class::from_byte(byte).expect("invalid class byte in segment stream");
                Segment::Colored { class, payload: TokenPayload::read(cursor, class) }
            }
        }
    }

    pub fn render(&self, dictionary: &Dictionary, templates: &TemplateRegistry) -> String {
        match self {
            Segment::Literal(text) => text.clone(),
            Segment::Colored { class, payload } => {
                let text = payload.resolve(*class, dictionary);
                format!(r#"<span class="{class:?}">{text}</span>"#)
            }
            Segment::Template { id, holes } => templates.get(*id).render(holes, dictionary),
        }
    }
}
