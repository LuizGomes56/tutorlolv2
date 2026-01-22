use super::*;

impl Generator<ItemData> for Bloodthirster {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
