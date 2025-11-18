use super::*;

impl Generator<ItemData> for VerdantBarrier {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
