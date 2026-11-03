use tutorlolv2_api::*;

#[test]
fn paths_encode_each_segment_and_query_is_typed() {
    assert_eq!(
        ShareInfoEndpoint::path(&ShareInfoEndpointPath {
            code: "雪 /?%".into()
        }),
        "/api/share/%E9%9B%AA%20%2F%3F%25"
    );
    assert_eq!(
        ListRecordings::query(&Pagination {
            before: None,
            limit: Some(10)
        })
        .unwrap(),
        "limit=10"
    );
}

#[test]
fn account_envelope_round_trip_and_status_validation() {
    let bytes = bincode::encode_to_vec(
        protocol::ApiResponse::new(
            200,
            Ok(protocol::AccountInfo {
                email: "a@b".into(),
                created_at_ms: 1,
            }),
        )
        .unwrap(),
        tutorlolv2_types::BINCODE_CONFIG,
    )
    .unwrap();
    let response =
        ResponseBuffer::<GetAccount>::new(200, ResponseHeaders::default(), bytes.clone()).unwrap();
    assert_eq!(response.decode().unwrap().email, "a@b");
    assert!(
        ResponseBuffer::<GetAccount>::new(201, ResponseHeaders::default(), bytes)
            .unwrap()
            .decode()
            .is_err()
    );
}

#[test]
fn empty_statuses_and_body_are_checked() {
    assert_eq!(
        ResponseBuffer::<UploadChunk>::new(201, ResponseHeaders::default(), vec![])
            .unwrap()
            .decode()
            .unwrap()
            .status,
        201
    );
    assert!(
        ResponseBuffer::<UploadChunk>::new(204, ResponseHeaders::default(), vec![])
            .unwrap()
            .decode()
            .is_err()
    );
    assert!(
        ResponseBuffer::<UploadChunk>::new(200, ResponseHeaders::default(), vec![1])
            .unwrap()
            .decode()
            .is_err()
    );
}

#[test]
fn borrowed_raw_response_is_retained_and_trailing_bytes_rejected() {
    let bytes = bincode::encode_to_vec(
        tutorlolv2::riot::FrameChunk {
            format: protocol::FRAMES_FORMAT,
            frames: vec![],
        },
        tutorlolv2_types::BINCODE_CONFIG,
    )
    .unwrap();
    let response =
        ResponseBuffer::<OwnerFrames>::new(200, ResponseHeaders::default(), bytes.clone()).unwrap();
    assert_eq!(response.decode().unwrap().format, protocol::FRAMES_FORMAT);
    let mut invalid = bytes;
    invalid.push(0);
    assert!(
        ResponseBuffer::<OwnerFrames>::new(200, ResponseHeaders::default(), invalid)
            .unwrap()
            .decode()
            .is_err()
    );
}
