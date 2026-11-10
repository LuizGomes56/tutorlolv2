//! Share codes (spec §11.2): 8 Crockford base32 symbols, case-insensitive,
//! displayed as `ABCD-EFGH`.

use alloc::string::String;

/// Crockford base32: digits and letters without `I`, `L`, `O`, `U`.
pub const CODE_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
const CODE_LEN: usize = 8;

/// Turns 40 random bits (from the OS CSPRNG) into a code, 5 bits per symbol,
/// most significant first.
pub fn code_from_random(bytes: [u8; 5]) -> String {
    let bits = bytes.iter().fold(0u64, |acc, &b| (acc << 8) | b as u64);

    (0..CODE_LEN)
        .rev()
        .map(|i| CODE_ALPHABET[((bits >> (i * 5)) & 0x1F) as usize] as char)
        .collect()
}

/// Normalizes what a person typed: uppercase, spaces and `-` dropped, and the
/// confusable letters mapped (`O`→`0`, `I`/`L`→`1`). `None` unless exactly 8
/// alphabet symbols remain.
pub fn normalize_code(input: &str) -> Option<String> {
    let code = input
        .chars()
        .filter(|c| !matches!(c, ' ' | '-'))
        .map(|c| match c.to_ascii_uppercase() {
            'O' => '0',
            'I' | 'L' => '1',
            other => other,
        })
        .collect::<String>();

    let valid = code.len() == CODE_LEN && code.bytes().all(|b| CODE_ALPHABET.contains(&b));

    valid.then_some(code)
}

/// `ABCDEFGH` ⇒ `ABCD-EFGH`.
pub fn display_code(code: &str) -> String {
    let (head, tail) = code.split_at(code.len().min(CODE_LEN / 2));
    let mut out = String::with_capacity(code.len() + 1);

    out.push_str(head);
    out.push('-');
    out.push_str(tail);
    out
}
