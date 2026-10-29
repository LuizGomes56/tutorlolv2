use tutorlolv2_protocol::*;

fn chunk(seq: u32, snapshots: &[(i64, f32)]) -> RawChunk {
    RawChunk {
        format: RAW_CHUNK_FORMAT,
        seq,
        snapshots: snapshots
            .iter()
            .map(|&(captured_at_ms, game_time)| RawSnapshot {
                captured_at_ms,
                game_time,
                json: vec![b'{', b'}'],
            })
            .collect(),
    }
}

#[test]
fn raw_chunk_roundtrips_through_bincode() {
    let original = chunk(7, &[(1000, 1.0), (2000, 2.0)]);
    let config = bincode::config::standard();
    let bytes = bincode::encode_to_vec(&original, config).unwrap();
    let (decoded, used): (RawChunk, _) = bincode::decode_from_slice(&bytes, config).unwrap();
    assert_eq!(used, bytes.len());
    assert_eq!(decoded, original);
}

#[test]
fn validate_chunk_accepts_increasing_snapshots() {
    assert_eq!(chunk(3, &[(1000, 1.0), (2000, 2.0)]).validate(3), Ok(()));
}

#[test]
fn validate_chunk_rejects_seq_mismatch() {
    assert_eq!(
        chunk(3, &[(1000, 1.0)]).validate(4),
        Err(ChunkError::SeqMismatch {
            expected: 4,
            got: 3
        })
    );
}

#[test]
fn validate_chunk_rejects_empty_and_121() {
    assert_eq!(chunk(0, &[]).validate(0), Err(ChunkError::Empty));
    let many: Vec<_> = (0..121).map(|i| (i as i64 * 1000, i as f32)).collect();
    assert_eq!(
        chunk(0, &many).validate(0),
        Err(ChunkError::TooManySnapshots(121))
    );
}

#[test]
fn validate_chunk_rejects_non_increasing_captured_at() {
    assert_eq!(
        chunk(0, &[(1000, 1.0), (1000, 2.0)]).validate(0),
        Err(ChunkError::CapturedAtNotIncreasing(1))
    );
}

#[test]
fn validate_chunk_tolerates_two_seconds_of_game_time_jitter() {
    assert_eq!(chunk(0, &[(1000, 10.0), (2000, 8.1)]).validate(0), Ok(()));
    assert_eq!(
        chunk(0, &[(1000, 10.0), (2000, 7.9)]).validate(0),
        Err(ChunkError::GameTimeWentBack(1))
    );
}

#[test]
fn validate_chunk_rejects_unknown_format() {
    let mut c = chunk(0, &[(1000, 1.0)]);
    c.format = 2;
    assert_eq!(c.validate(0), Err(ChunkError::Format(2)));
}
