//! Types and rules shared by the desktop app, the server and the web app for
//! recording, uploading and sharing games. See `docs/game-recording-and-sharing-spec.md`.
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod api;
pub mod code;
pub mod enums;
pub mod format;
pub mod probe;
pub mod snapshot;

pub use api::*;
pub use code::*;
pub use enums::*;
pub use format::*;
pub use probe::*;
pub use snapshot::*;

#[doc(hidden)]
pub mod __private {
    pub use alloc::string::String;
    pub use bincode;
    pub use serde;
}
