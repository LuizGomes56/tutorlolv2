use super::*;

impl Generator<ItemData> for NullMagicMantle {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
