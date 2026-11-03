//! Generated Rust endpoint contracts. Run `cargo run -p tutorlolv2_api_codegen -- generate`.
pub use tutorlolv2_protocol as protocol;
pub use tutorlolv2_protocol::endpoint::*;
mod generated;
pub use generated::*;
pub use uuid::Uuid;
pub use tutorlolv2::riot::{FrameChunk, RiotRealtime};

/// Endpoint inputs are tied to their marker; callers cannot choose another payload.
/// ```compile_fail
/// use tutorlolv2_api::{Endpoint, GetAccount, NoPath, NoQuery, NoHeaders};
/// GetAccount::request("http://localhost", &NoPath, &NoQuery, &NoHeaders, &7u32).unwrap();
/// ```
/// ```compile_fail
/// use tutorlolv2_api::{Endpoint, ListRecordings, After, NoPath, NoHeaders, NoBody};
/// ListRecordings::request("http://localhost", &NoPath, &After::default(), &NoHeaders, &NoBody).unwrap();
/// ```
pub struct ContractTypeChecks;
