use super::*;

pub fn export_code(out_dir: &str) {
    let mut byte_content = Vec::new();
    for file in ["champions", "items", "runes"] {
        let content = fs::read(format!("comptime_exports/{}.txt", file)).unwrap();
        byte_content.extend_from_slice(&content);
    }
    let bytes = compress_bytes!(0 byte_content);
    let _ = fs::write(Path::new(out_dir).join("export_code.br"), &bytes);
    let _ = fs::write("comptime_exports/export_code.br", bytes);
}
