//! Palette from the web app: `std-*` is Tailwind's zinc scale (`public/tailwind.css`),
//! the rest comes from `public/main.css` and `public/highliter.css`.
use tutorlolv2_codec::Class;

pub const STD_200: u32 = 0xe4e4e7;
pub const STD_300: u32 = 0xd4d4d8;
pub const STD_400: u32 = 0xa1a1aa;
pub const STD_500: u32 = 0x71717a;
pub const STD_700: u32 = 0x3f3f46;
pub const STD_800: u32 = 0x27272a;
/// Application shell background (`bg-std-900`).
pub const STD_900: u32 = 0x18181b;

/// `body { color }`.
pub const TEXT: u32 = 0xffffff;

/// `.box { box-shadow: 0 0 10px 2.5px #08080f }`.
pub const BOX_SHADOW: u32 = 0x08080f;

/// Selector dropdown, `bg-[#16161c]`.
pub const DROPDOWN: u32 = 0x16161c;

/// Table rows: `tbody tr:nth-child(odd)` and `(even)`.
pub const ROW_ODD: u32 = 0x16161c;
pub const ROW_EVEN: u32 = 0x1c1c21;

/// Text selection in inputs (sky-400 at 30% alpha, RGBA).
pub const SELECTION: u32 = 0x38bdf84d;

/// `code, pre { color }`.
pub const CODE_TEXT: u32 = 0xd4d4d4;

/// `code, pre { font-family }`. The first choice, `.ZedMono` (Lilex), ships only as WOFF2.
pub const CODE_FONT: &str = "Consolas";

/// Foreground and background of each highlight class (`public/highliter.css`).
pub const fn class_colors(class: Class) -> (u32, Option<u32>) {
    let color = match class {
        Class::Comment => 0x969696,
        Class::String => 0xce9178,
        Class::Lifetime | Class::Keyword | Class::Boolean | Class::Macro => 0x569cd6,
        Class::Control => 0xc586c0,
        Class::Constant => 0x4fc1ff,
        Class::Type | Class::Primitive => 0x4ec8b0,
        Class::Number => 0xb3cda8,
        Class::Function => 0xdcdcaa,
        Class::Variable => 0x9cdcfe,
        Class::Bracket1 => 0xffd700,
        Class::Bracket2 => 0xda70d6,
        Class::Bracket3 => 0x189fff,
    };

    let background = match class {
        Class::Comment => Some(0x262626),
        _ => None,
    };

    (color, background)
}
