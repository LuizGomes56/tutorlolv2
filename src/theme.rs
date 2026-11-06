//! Palette from the web app: `std-*` is Tailwind's zinc scale (`public/tailwind.css`),
//! the rest comes from `public/main.css` and `public/highliter.css`.
use tutorlolv2::{AdaptiveType, DamageType};
use tutorlolv2_codec::Class;

pub const STD_100: u32 = 0xf4f4f5;
pub const STD_200: u32 = 0xe4e4e7;
pub const STD_300: u32 = 0xd4d4d8;
pub const STD_400: u32 = 0xa1a1aa;
pub const STD_500: u32 = 0x71717a;
pub const STD_600: u32 = 0x52525b;
pub const STD_700: u32 = 0x3f3f46;
pub const STD_800: u32 = 0x27272a;
/// Application shell background (`bg-std-900`).
pub const STD_900: u32 = 0x18181b;
pub const STD_950: u32 = 0x09090b;

/// `body { color }`.
pub const TEXT: u32 = 0xffffff;

/// `.box { box-shadow: 0 0 10px 2.5px #08080f }`.
pub const BOX_SHADOW: u32 = 0x08080f;

/// Selector dropdown, `bg-[#16161c]`.
pub const DROPDOWN: u32 = 0x16161c;

/// Table rows: `tbody tr:nth-child(odd)` and `(even)`.
pub const ROW_ODD: u32 = 0x16161c;
pub const ROW_EVEN: u32 = 0x1c1c21;

/// `hover:bg-std-800/60` (RGBA).
pub const HOVER: u32 = 0x27272a99;

/// Checked checkbox: `border-violet-500 bg-violet-600`.
pub const VIOLET_500: u32 = 0x8b5cf6;
pub const VIOLET_600: u32 = 0x7c3aed;

/// Selector button counters: `text-sky-300 bg-sky-500/15 border-sky-500/20` (the last two RGBA).
pub const SKY_300: u32 = 0x7dd3fc;
pub const CYAN_400: u32 = 0x22d3ee;
pub const SKY_BADGE: u32 = 0x0ea5e926;
pub const SKY_BADGE_BORDER: u32 = 0x0ea5e933;

/// Text selection in inputs (sky-400 at 30% alpha, RGBA).
pub const SELECTION: u32 = 0x38bdf84d;

/// `code, pre { color }`.
pub const CODE_TEXT: u32 = 0xd4d4d4;

/// `code, pre { font-family }`. The first choice, `.ZedMono` (Lilex), ships only as WOFF2.
pub const CODE_FONT: &str = "Consolas";

/// Damage cell colors (`DamageType::class` in `tutorlolv2_types/src/yew.rs`).
pub const fn damage_color(damage_type: DamageType) -> u32 {
    match damage_type {
        // `text-orange-500`
        DamageType::Physical => 0xf97316,
        // `text-sky-500`
        DamageType::Magic => 0x0ea5e9,
        // `text-indigo-500`
        DamageType::Mixed => 0x6366f1,
        DamageType::True => 0xffffff,
        // `text-purple-500`
        DamageType::Adaptive => 0xa855f7,
        DamageType::Unspecified => STD_300,
    }
}

/// Adaptive damage takes the color of the type it resolves to.
pub const fn adaptive_color(adaptive_type: AdaptiveType) -> u32 {
    match adaptive_type {
        AdaptiveType::Physical => damage_color(DamageType::Physical),
        AdaptiveType::Magic => damage_color(DamageType::Magic),
    }
}

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
