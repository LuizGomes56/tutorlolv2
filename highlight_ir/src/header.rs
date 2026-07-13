//! Fixed 15-byte header written at the start of every `.ir` file.

pub const MAGIC: [u8; 4] = *b"HLIR";
pub const HEADER_SIZE: usize = 15;

#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub version: u8,
    pub dict_version: u8,
    pub template_version: u8,
    pub segment_count: u32,
    pub aux_length: u32,
}

impl Header {
    /// `dict_version`/`template_version` are informational only: encoder
    /// and decoder are always built from the same dictionary and template
    /// tables, so nothing here is validated against them at decode time.
    pub fn new(segment_count: u32, aux_length: u32) -> Self {
        Header {
            version: 1,
            dict_version: 1,
            template_version: 1,
            segment_count,
            aux_length,
        }
    }

    pub fn write_to(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&MAGIC);
        out.push(self.version);
        out.push(self.dict_version);
        out.push(self.template_version);
        out.extend_from_slice(&self.segment_count.to_le_bytes());
        out.extend_from_slice(&self.aux_length.to_le_bytes());
    }

    /// Parses a header from the start of `bytes`. Returns `None` if the
    /// buffer is too short or the magic number doesn't match.
    pub fn read_from(bytes: &[u8]) -> Option<Header> {
        if bytes.len() < HEADER_SIZE || bytes[0..4] != MAGIC {
            return None;
        }
        Some(Header {
            version: bytes[4],
            dict_version: bytes[5],
            template_version: bytes[6],
            segment_count: u32::from_le_bytes(bytes[7..11].try_into().unwrap()),
            aux_length: u32::from_le_bytes(bytes[11..15].try_into().unwrap()),
        })
    }
}
