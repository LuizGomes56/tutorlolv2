use brotli2::read::BrotliDecoder;
use std::fs::File;
use std::io::{BufReader, Read};

#[test]
fn __test_export_read() {
    let file = File::open("__test.txt").expect("Error opening __test.txt");
    let buf = BufReader::new(file);
    let mut decoder = BrotliDecoder::new(buf);
    let mut decompressed_bytes = Vec::new();
    decoder
        .read_to_end(&mut decompressed_bytes)
        .expect("falha ao descomprimir");
    let decompressed_str =
        String::from_utf8(decompressed_bytes.clone()).expect("bytes are not utf-8 valid");
    println!("Decompressed str: {}", decompressed_str);
    println!(
        "[2] decompressed_bytes [len = {}]: {:#?}",
        decompressed_bytes.len(),
        unsafe { str::from_utf8_unchecked(&decompressed_bytes) }
    );
}
