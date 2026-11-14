//! Versioned Bincode records saved as files: the web app's `utils/storage.rs`.
//!
//! The record format is the web's (`TLST` magic, version, kind, value), without the base64
//! step that `localStorage` needed. Files live in `%APPDATA%\tutorlolv2`. A record that does not
//! decode (corrupt, from another version or of another kind) is deleted, as on the web.
use bincode::{Decode, Encode, config::Configuration};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

const MAGIC: [u8; 4] = *b"TLST";
const VERSION: u8 = 1;
const MAX_RECORD_BYTES: usize = 1_048_576;
const CONFIG: Configuration = bincode::config::standard();

#[derive(Encode, Decode)]
struct Record<T> {
    magic: [u8; 4],
    version: u8,
    kind: u8,
    value: T,
}

/// `%APPDATA%\tutorlolv2`, or `None` when `APPDATA` is not set.
fn data_dir() -> Option<PathBuf> {
    std::env::var_os("APPDATA").map(|dir| Path::new(&dir).join("tutorlolv2"))
}

fn encode_record(kind: u8, value: &impl Encode) -> Option<Vec<u8>> {
    let record = Record {
        magic: MAGIC,
        version: VERSION,
        kind,
        value,
    };

    let bytes = bincode::encode_to_vec(record, CONFIG).ok()?;

    (bytes.len() <= MAX_RECORD_BYTES).then_some(bytes)
}

fn decode_record<T: Decode<()>>(kind: u8, bytes: &[u8]) -> Option<T> {
    if bytes.len() > MAX_RECORD_BYTES {
        return None;
    }

    let (record, read): (Record<T>, _) =
        bincode::decode_from_slice(bytes, CONFIG.with_limit::<MAX_RECORD_BYTES>()).ok()?;

    (read == bytes.len()
        && record.magic == MAGIC
        && record.version == VERSION
        && record.kind == kind)
        .then_some(record.value)
}

/// Reads the record in `name`, deleting the file if it holds no valid record of `kind`.
pub fn read_record<T: Decode<()>>(name: &str, kind: u8) -> Option<T> {
    let path = data_dir()?.join(name);
    let bytes = fs::read(&path).ok()?;
    let value = decode_record(kind, &bytes);

    if value.is_none() {
        let _ = fs::remove_file(&path);
    }

    value
}

/// Writes the record to `name`. A temporary file is renamed over the old one, so an
/// interrupted write keeps the previous record.
pub fn write_record(name: &str, kind: u8, value: &impl Encode) -> io::Result<()> {
    let dir = data_dir().ok_or_else(|| io::Error::other("APPDATA is not set"))?;
    let bytes = encode_record(kind, value)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "record too large"))?;

    fs::create_dir_all(&dir)?;

    let temporary = dir.join(format!("{name}.tmp"));

    fs::write(&temporary, bytes)?;
    fs::rename(temporary, dir.join(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_roundtrip_and_reject_other_kinds_versions_and_trailing_bytes() {
        let bytes = encode_record(1, &vec![1_u32, 2, 3]).unwrap();

        assert_eq!(decode_record::<Vec<u32>>(1, &bytes), Some(vec![1, 2, 3]));
        assert_eq!(decode_record::<Vec<u32>>(2, &bytes), None);

        let mut trailing = bytes.clone();
        trailing.push(0);
        assert_eq!(decode_record::<Vec<u32>>(1, &trailing), None);

        let mut future = bytes.clone();
        future[4] = future[4].wrapping_add(1);
        assert_eq!(decode_record::<Vec<u32>>(1, &future), None);

        let mut wrong_magic = bytes;
        wrong_magic[0] ^= 1;
        assert_eq!(decode_record::<Vec<u32>>(1, &wrong_magic), None);

        // A forged collection length must respect the allocation limit.
        let forged = encode_record(1, &u64::MAX).unwrap();
        assert_eq!(decode_record::<Vec<u32>>(1, &forged), None);
    }
}
