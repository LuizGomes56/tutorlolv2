//! The sixteen syntax-highlighting classes. Each variant's discriminant is
//! also the `classByte` written into the `.ir` segment stream, so the two
//! must always stay in sync — `Class::ALL` is ordered to match.

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    Comment = 0x00,
    String = 0x01,
    Lifetime = 0x02,
    Keyword = 0x03,
    Control = 0x04,
    Constant = 0x05,
    Type = 0x06,
    Primitive = 0x07,
    Number = 0x08,
    Boolean = 0x09,
    Macro = 0x0A,
    Function = 0x0B,
    Variable = 0x0C,
    Bracket1 = 0x0D,
    Bracket2 = 0x0E,
    Bracket3 = 0x0F,
}

impl Class {
    /// Ordered so that `ALL[byte as usize] == Class::from_byte(byte)`.
    pub const ALL: [Class; 16] = [
        Class::Comment,
        Class::String,
        Class::Lifetime,
        Class::Keyword,
        Class::Control,
        Class::Constant,
        Class::Type,
        Class::Primitive,
        Class::Number,
        Class::Boolean,
        Class::Macro,
        Class::Function,
        Class::Variable,
        Class::Bracket1,
        Class::Bracket2,
        Class::Bracket3,
    ];

    /// Recovers a `Class` from a raw segment tag byte. Returns `None` for
    /// `0xFE` (template marker), `0xFF` (literal marker), and any value
    /// above `0x0F`.
    pub fn from_byte(byte: u8) -> Option<Class> {
        Class::ALL.get(byte as usize).copied()
    }

    /// Whether this class keeps a static dictionary of known tokens. Known
    /// tokens are stored as a one-byte index instead of literal text;
    /// classes without a dictionary always carry their literal text.
    pub fn has_dictionary(self) -> bool {
        matches!(
            self,
            Class::Keyword
                | Class::Control
                | Class::Constant
                | Class::Type
                | Class::Primitive
                | Class::Macro
                | Class::Function
                | Class::Variable
        )
    }
}
