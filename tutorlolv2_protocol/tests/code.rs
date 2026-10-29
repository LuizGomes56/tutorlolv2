use tutorlolv2_protocol::*;

/// The `share_codes.code` CHECK constraint: `^[0-9A-HJKMNP-TV-Z]{8}$`.
fn matches_db_check(code: &str) -> bool {
    code.len() == 8
        && code.bytes().all(|b| {
            matches!(b, b'0'..=b'9' | b'A'..=b'H' | b'J' | b'K' | b'M' | b'N' | b'P'..=b'T' | b'V'..=b'Z')
        })
}

#[test]
fn zero_bytes_give_all_zeros() {
    assert_eq!(code_from_random([0; 5]), "00000000");
}

#[test]
fn max_bytes_give_all_z() {
    assert_eq!(code_from_random([0xFF; 5]), "ZZZZZZZZ");
}

#[test]
fn bits_are_read_most_significant_first() {
    assert_eq!(code_from_random([0b0000_1000, 0, 0, 0, 0]), "10000000");
    assert_eq!(code_from_random([0, 0, 0, 0, 0b0000_0001]), "00000001");
}

#[test]
fn normalize_maps_confusables() {
    assert_eq!(normalize_code("abcd-efgo").as_deref(), Some("ABCDEFG0"));
    assert_eq!(normalize_code(" il 23 45 67").as_deref(), Some("11234567"));
}

#[test]
fn normalize_rejects_u_and_wrong_length() {
    assert_eq!(normalize_code("ABCDEFGU"), None);
    assert_eq!(normalize_code("ABC"), None);
    assert_eq!(normalize_code("ABCDEFGHJ"), None);
}

#[test]
fn display_inserts_dash() {
    assert_eq!(display_code("ABCDEFGH"), "ABCD-EFGH");
}

#[test]
fn alphabet_matches_db_check() {
    assert!(CODE_ALPHABET.iter().all(|&b| matches_db_check(&(b as char).to_string().repeat(8))));
}

#[test]
fn generated_codes_match_db_check() {
    let mut state = 0x2545_F491_4F6C_DD1Du64;
    for _ in 0..1000 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let bytes: [u8; 5] = state.to_le_bytes()[..5].try_into().unwrap();
        let code = code_from_random(bytes);
        assert!(matches_db_check(&code), "{code}");
        assert_eq!(normalize_code(&display_code(&code)).as_deref(), Some(code.as_str()));
    }
}
