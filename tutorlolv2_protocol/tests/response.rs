use tutorlolv2_protocol::*;

#[test]
fn validates_status_and_full_bounded_decode() {
    let response = ApiResponse::new(200, Ok(7u32)).unwrap();
    let bytes = bincode::encode_to_vec(response, tutorlolv2_types::BINCODE_CONFIG).unwrap();
    assert_eq!(
        ApiResponse::<u32>::decode(&bytes, 200).unwrap().result,
        Ok(7)
    );
    assert!(ApiResponse::<u32>::decode(&bytes, 201).is_err());
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(ApiResponse::<u32>::decode(&trailing, 200).is_err());
    assert!(ApiResponse::<u32>::decode(&vec![0; MAX_RESPONSE_BYTES + 1], 200).is_err());
    assert!(ApiResponse::<u32>::new(204, Ok(7)).is_err());
    assert!(ApiResponse::<u32>::new(500, Ok(7)).is_err());
    assert!(ApiResponse::<u32>::decode(&[255], 200).is_err());
}
#[test]
fn borrowing_and_allocation_limits_are_enforced() {
    let bytes = bincode::encode_to_vec(
        ApiResponse::new(200, Ok("borrowed")).unwrap(),
        tutorlolv2_types::BINCODE_CONFIG,
    )
    .unwrap();
    let response = ApiResponse::<&str>::borrow_decode(&bytes, 200).unwrap();
    let text = response.result.unwrap();
    assert_eq!(text, "borrowed");
    assert!(
        (bytes.as_ptr() as usize..bytes.as_ptr() as usize + bytes.len())
            .contains(&(text.as_ptr() as usize))
    );
    let hostile =
        bincode::encode_to_vec((200u16, 0u32, u64::MAX), tutorlolv2_types::BINCODE_CONFIG).unwrap();
    assert!(ApiResponse::<String>::decode(&hostile, 200).is_err());
}
#[test]
fn failure_layout_is_independent_of_success_type() {
    let error = ApiError {
        code: ErrorCode::NotFound,
        message: String::from("missing"),
        details: None,
        traces: vec![],
    };
    let bytes = bincode::encode_to_vec(
        ApiResponse::<()>::new(404, Err(error.clone())).unwrap(),
        tutorlolv2_types::BINCODE_CONFIG,
    )
    .unwrap();
    assert_eq!(
        ApiResponse::<String>::decode(&bytes, 404).unwrap().result,
        Err(error.clone())
    );
    assert_eq!(
        ApiResponse::<Vec<u64>>::decode(&bytes, 404).unwrap().result,
        Err(error)
    );
}
