pub mod cache;
pub mod data;

use std::hash::Hash;

pub use cache::*;
pub use data::*;
pub use tutorlolv2_types::*;

impl Hash for ChampionId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(*self as u8);
    }
}
