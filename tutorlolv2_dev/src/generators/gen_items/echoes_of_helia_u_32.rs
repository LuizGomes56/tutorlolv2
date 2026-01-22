use super::*;

impl Generator<ItemData> for EchoesOfHeliaU32 {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
