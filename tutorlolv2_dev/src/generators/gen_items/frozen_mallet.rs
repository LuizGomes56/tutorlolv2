use super::*;

impl Generator<ItemData> for FrozenMallet {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
