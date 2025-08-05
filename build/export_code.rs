use super::*;

pub fn export_code(out_dir: &str) {
    let mut mega_block = String::with_capacity(1 << 24);
    export_champions(out_dir, &mut mega_block);
    export_items(out_dir, &mut mega_block);
    export_runes(out_dir, &mut mega_block);
    let mut bytes = Vec::new();
    for file in ["champions", "items", "runes"] {
        let content = fs::read(format!("comptime_exports/{}.txt", file)).unwrap();
        bytes.extend_from_slice(&content);
    }
    let mega_block_compressed = compress_bytes!(mega_block.as_bytes());
    bytes.extend_from_slice(
        format!(
            "pub static UNCOMPRESSED_MEGA_BLOCK_SIZE: usize = {}; pub static MEGA_BLOCK: [u8; {}] = [{}];",
            mega_block.len(),
            mega_block_compressed.len(),
            mega_block_compressed.join(",")
        )
        .as_bytes(),
    );
    let _ = fs::write("comptime_exports/export_code.txt", bytes);
}
