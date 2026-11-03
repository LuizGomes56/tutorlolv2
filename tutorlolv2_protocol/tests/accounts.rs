use tutorlolv2_protocol::{AccountInfo, DesktopSession, RedeemTicket};
use tutorlolv2_types::BINCODE_CONFIG;

#[test]
fn account_http_payloads_roundtrip_without_changing_existing_protocol() {
    let info = AccountInfo {
        email: "player@example.test".into(),
        created_at_ms: 1234,
    };
    let bytes = bincode::encode_to_vec(&info, BINCODE_CONFIG).unwrap();
    let (decoded, used): (AccountInfo, _) =
        bincode::decode_from_slice(&bytes, BINCODE_CONFIG).unwrap();
    assert_eq!((decoded, used), (info, bytes.len()));
    let session = DesktopSession {
        token: "native-secret".into(),
        email: "player@example.test".into(),
    };
    let bytes = bincode::encode_to_vec(&session, BINCODE_CONFIG).unwrap();
    let (decoded, used): (DesktopSession, _) =
        bincode::decode_from_slice(&bytes, BINCODE_CONFIG).unwrap();
    assert!(decoded == session);
    assert_eq!(used, bytes.len());
    let ticket = RedeemTicket {
        ticket: "one-use-ticket".into(),
    };
    let bytes = bincode::encode_to_vec(&ticket, BINCODE_CONFIG).unwrap();
    let (decoded, used): (RedeemTicket, _) =
        bincode::decode_from_slice(&bytes, BINCODE_CONFIG).unwrap();
    assert!(decoded == ticket);
    assert_eq!(used, bytes.len());
}
