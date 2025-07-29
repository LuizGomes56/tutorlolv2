use super::*;

pub fn export_code(_: &str) {
    let mut bytes = Vec::new();
    for file in ["champions", "items", "runes"] {
        let content = fs::read(format!("comptime_exports/{}.txt", file)).unwrap();
        bytes.extend_from_slice(&content);
    }
    let _ = fs::write("comptime_exports/export_code.txt", bytes);
}
