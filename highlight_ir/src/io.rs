//! Mirror-image reader/writer for the two on-disk buffers: the main
//! segment stream and the auxiliary literal-text buffer. Every `Segment`
//! and `Hole` reads and writes itself through one of these, so cursor
//! bookkeeping never has to be duplicated at call sites.

use crate::header::{Header, HEADER_SIZE};

/// Sequential reader over an `.ir` segment stream plus its `.txt` companion.
pub struct Cursor<'a> {
    ir: &'a [u8],
    ir_pos: usize,
    aux: &'a [u8],
    aux_pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(ir: &'a [u8], aux: &'a [u8]) -> Self {
        Cursor { ir, ir_pos: 0, aux, aux_pos: 0 }
    }

    pub fn read_u8(&mut self) -> u8 {
        let byte = self.ir[self.ir_pos];
        self.ir_pos += 1;
        byte
    }

    pub fn read_u16_le(&mut self) -> u16 {
        let bytes = [self.ir[self.ir_pos], self.ir[self.ir_pos + 1]];
        self.ir_pos += 2;
        u16::from_le_bytes(bytes)
    }

    /// Reads a one-byte length prefix, then that many bytes from the
    /// auxiliary buffer, advancing its cursor independently of `ir_pos`.
    pub fn read_aux_text(&mut self) -> &'a str {
        let len = self.read_u8() as usize;
        let slice = &self.aux[self.aux_pos..self.aux_pos + len];
        self.aux_pos += len;
        std::str::from_utf8(slice).expect("aux buffer is guaranteed ASCII")
    }
}

/// Accumulates bytes into the two output buffers as segments are encoded,
/// then folds in the header once the final counts are known.
#[derive(Default)]
pub struct Writer {
    ir: Vec<u8>,
    aux: Vec<u8>,
    segment_count: u32,
}

impl Writer {
    pub fn new() -> Self {
        Writer::default()
    }

    pub fn write_u8(&mut self, byte: u8) {
        self.ir.push(byte);
    }

    pub fn write_u16_le(&mut self, value: u16) {
        self.ir.extend_from_slice(&value.to_le_bytes());
    }

    /// Appends `text` to the aux buffer and writes its length as a single
    /// byte into the main stream. `text` must be ASCII and under 256 bytes,
    /// which the whole-program-known input always satisfies here.
    pub fn write_aux_text(&mut self, text: &str) {
        debug_assert!(text.len() <= u8::MAX as usize, "aux literal too long");
        self.write_u8(text.len() as u8);
        self.aux.extend_from_slice(text.as_bytes());
    }

    pub fn count_segment(&mut self) {
        self.segment_count += 1;
    }

    /// Consumes the writer, producing the finished `.ir` bytes (header +
    /// segment stream) and the `.txt` auxiliary bytes.
    pub fn finish(self) -> (Vec<u8>, Vec<u8>) {
        let header = Header::new(self.segment_count, self.aux.len() as u32);
        let mut ir = Vec::with_capacity(HEADER_SIZE + self.ir.len());
        header.write_to(&mut ir);
        ir.extend_from_slice(&self.ir);
        (ir, self.aux)
    }
}
