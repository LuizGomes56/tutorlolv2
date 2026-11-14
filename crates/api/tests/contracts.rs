use api::*;

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
        types::BINCODE_CONFIG,
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
        math::riot::FrameChunk {
            format: protocol::FRAMES_FORMAT,
            frames: vec![],
        },
        types::BINCODE_CONFIG,
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

#[test]
fn browser_error_pages_remain_html_and_redirects_require_location() {
    let response = ResponseBuffer::<GoogleCallback>::new(
        400,
        ResponseHeaders::default(),
        b"<h1>Retry sign-in</h1>".to_vec(),
    )
    .unwrap();
    let page = response.decode().unwrap();
    assert_eq!(page.status, 400);
    assert_eq!(page.body, b"<h1>Retry sign-in</h1>");
    assert!(
        ResponseBuffer::<GoogleStart>::new(302, ResponseHeaders::default(), vec![])
            .unwrap()
            .decode()
            .is_err()
    );
}

#[test]
fn calculator_borrowed_view_matches_the_owned_server_request() {
    let input = math::model::InputGame::default();
    let bytes = Calculator::encode_body(&input).unwrap();
    let (owned, used): (math::model::OwnedInputGame, _) =
        bincode::decode_from_slice(&bytes, types::BINCODE_CONFIG).unwrap();
    assert_eq!(used, bytes.len());
    assert_eq!(owned.prepare().as_input(), input);
}

#[test]
fn generated_contract_dtos_support_bincode() {
    let query = Pagination {
        before: Some(uuid::Uuid::from_u128(42)),
        limit: Some(25),
    };
    let bytes = bincode::encode_to_vec(&query, types::BINCODE_CONFIG).unwrap();
    let (decoded, used): (Pagination, _) =
        bincode::decode_from_slice(&bytes, types::BINCODE_CONFIG).unwrap();
    assert_eq!(used, bytes.len());
    assert_eq!(decoded.before, query.before);
    assert_eq!(decoded.limit, query.limit);
}
