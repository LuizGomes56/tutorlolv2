#[allow(dead_code)]
mod export_code;
mod html_gen;

use bincode::{Decode, Encode};
pub use html_gen::*;
pub(self) use serde::{Deserialize, Serialize};
use tutorlolv2_types::*;

pub static MEGA_BLOCK: &'static str = include_str!("../assets/mega_block.txt");

macro_rules! impl_url {
    ($($kind:ident),* $(,)?) => {
        paste::paste! {
            pub struct Url;

            impl Url {
                pub const BASE: &'static str = "http://localhost:8082";
                $(
                    pub const [<$kind:upper>]: &'static str =
                        concat!("http://localhost:8082/img/", stringify!($kind));
                )*
            }
        }
    };
}

impl_url!(champions, items, runes);
