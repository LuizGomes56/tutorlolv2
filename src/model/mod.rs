use bincode::{Encode, enc::Encoder, error::EncodeError};
use tinyset::SetU32;

pub mod base;
pub mod calculator;
pub mod dev;
pub mod realtime;
pub mod riot;

pub const SIZE_ITEMS_EXPECTED: usize = 7;
pub const SIZE_RUNES_EXPECTED: usize = 3;
pub const SIZE_ABILITIES: usize = 7;
pub const SIZE_ENEMIES_EXPECTED: usize = 5;

pub struct WrapSetU32(pub SetU32);

impl From<SetU32> for WrapSetU32 {
    #[inline]
    fn from(set: SetU32) -> Self {
        WrapSetU32(set)
    }
}

impl Encode for WrapSetU32 {
    fn encode<E: Encoder>(&self, enc: &mut E) -> Result<(), EncodeError> {
        self.0.len().encode(enc)?;
        for x in self.0.iter() {
            x.encode(enc)?;
        }
        Ok(())
    }
}
